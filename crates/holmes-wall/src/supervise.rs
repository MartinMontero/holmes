//! Lock 2d — supervised backend lifecycle (spec §4.4: "a Holmes-managed
//! local service a non-developer never sees; install/run/health/recover").
//!
//! The wall's storage backend (Neo4j) runs as a **supervised child
//! process**, not a system service: Holmes owns its lifecycle. The
//! guarantees, enforced here and proven by test:
//! - **kill-on-close / no orphans** — dropping the supervisor kills the
//!   child and reaps it; nothing leaks past Holmes's own lifetime.
//! - **path-confined data directory** — the backend's data lives under a
//!   Holmes-owned root; a data dir outside that root is refused.
//! - **no system-wide install requirement** — the backend is launched
//!   from an explicit program path the caller supplies (a bundled server
//!   binary or a container run); nothing here shells out to a package
//!   manager or assumes a global install.
//!
//! The type is backend-agnostic (it supervises any configured process),
//! so the no-orphan guarantee is provable deterministically with a
//! stand-in child, independent of Neo4j reachability.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub enum SuperviseError {
    ProgramNotAbsolute(PathBuf),
    DataDirNotConfined { data_dir: PathBuf, root: PathBuf },
    Spawn(String),
}

impl std::fmt::Display for SuperviseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuperviseError::ProgramNotAbsolute(p) => {
                write!(
                    f,
                    "refused: backend program path '{}' is not absolute",
                    p.display()
                )
            }
            SuperviseError::DataDirNotConfined { data_dir, root } => write!(
                f,
                "refused: data dir '{}' is not confined under Holmes root '{}'",
                data_dir.display(),
                root.display()
            ),
            SuperviseError::Spawn(s) => write!(f, "backend spawn failed: {s}"),
        }
    }
}

impl std::error::Error for SuperviseError {}

pub struct BackendSpec<'a> {
    /// Absolute path to the backend server program (bundled binary or a
    /// container runtime). Relative paths are refused.
    pub program: &'a Path,
    pub args: Vec<String>,
    /// Where the backend keeps its data; must resolve under `confine_root`.
    pub data_dir: &'a Path,
    /// The Holmes-owned root the data dir must live under.
    pub confine_root: &'a Path,
}

/// A running backend the supervisor owns. Dropping it kills the child.
#[derive(Debug)]
pub struct SupervisedBackend {
    child: Child,
    pid: u32,
    data_dir: PathBuf,
}

/// Confinement check: `data_dir`, once normalized, must live under
/// `confine_root`. Uses lexical normalization (no symlink resolution
/// needed since the dir may not exist yet) plus a `..`-escape guard.
fn is_confined(data_dir: &Path, confine_root: &Path) -> bool {
    let norm = |p: &Path| -> PathBuf {
        let mut out = PathBuf::new();
        for comp in p.components() {
            use std::path::Component::*;
            match comp {
                ParentDir => {
                    out.pop();
                }
                CurDir => {}
                other => out.push(other.as_os_str()),
            }
        }
        out
    };
    let d = norm(data_dir);
    let r = norm(confine_root);
    // Under-or-equal after `..` components are collapsed, so `root/../etc`
    // cannot escape confinement.
    d.starts_with(&r)
}

impl SupervisedBackend {
    /// Validate confinement, create the data dir, and spawn the backend.
    pub fn start(spec: &BackendSpec) -> Result<Self, SuperviseError> {
        if !spec.program.is_absolute() {
            return Err(SuperviseError::ProgramNotAbsolute(
                spec.program.to_path_buf(),
            ));
        }
        if !is_confined(spec.data_dir, spec.confine_root) {
            return Err(SuperviseError::DataDirNotConfined {
                data_dir: spec.data_dir.to_path_buf(),
                root: spec.confine_root.to_path_buf(),
            });
        }
        std::fs::create_dir_all(spec.data_dir)
            .map_err(|e| SuperviseError::Spawn(format!("create data dir: {e}")))?;
        let child = Command::new(spec.program)
            .args(&spec.args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| SuperviseError::Spawn(e.to_string()))?;
        let pid = child.id();
        Ok(Self {
            child,
            pid,
            data_dir: spec.data_dir.to_path_buf(),
        })
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// True while the child is still running (has not exited).
    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }
}

impl Drop for SupervisedBackend {
    fn drop(&mut self) {
        // kill-on-close: terminate and reap so no orphan survives Holmes.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    fn alive(pid: u32) -> bool {
        // signal 0 probes existence without affecting the process.
        unsafe { libc_kill(pid as i32, 0) == 0 }
    }

    #[cfg(unix)]
    extern "C" {
        #[link_name = "kill"]
        fn libc_kill(pid: i32, sig: i32) -> i32;
    }

    #[test]
    fn program_path_must_be_absolute() {
        let tmp = std::env::temp_dir().join("holmes-wall-confine");
        let spec = BackendSpec {
            program: Path::new("sleep"),
            args: vec!["100".into()],
            data_dir: &tmp.join("data"),
            confine_root: &tmp,
        };
        assert!(matches!(
            SupervisedBackend::start(&spec).unwrap_err(),
            SuperviseError::ProgramNotAbsolute(_)
        ));
    }

    #[test]
    fn data_dir_outside_the_root_is_refused() {
        let root = std::env::temp_dir().join("holmes-wall-root");
        let outside = Path::new("/etc/holmes-should-not-write");
        let spec = BackendSpec {
            program: Path::new("/bin/sleep"),
            args: vec!["100".into()],
            data_dir: outside,
            confine_root: &root,
        };
        assert!(matches!(
            SupervisedBackend::start(&spec).unwrap_err(),
            SuperviseError::DataDirNotConfined { .. }
        ));
    }

    #[cfg(unix)]
    #[test]
    fn dropping_the_supervisor_kills_the_child_no_orphan() {
        let root = std::env::temp_dir().join(format!("holmes-wall-{}", std::process::id()));
        let sleep = if Path::new("/bin/sleep").exists() {
            "/bin/sleep"
        } else {
            "/usr/bin/sleep"
        };
        let data = root.join("data");
        let spec = BackendSpec {
            program: Path::new(sleep),
            args: vec!["300".into()],
            data_dir: &data,
            confine_root: &root,
        };
        let pid = {
            let mut backend = SupervisedBackend::start(&spec).expect("spawn stand-in backend");
            assert!(backend.is_running());
            assert!(alive(backend.pid()));
            assert!(backend.data_dir().starts_with(&root), "data dir confined");
            backend.pid()
            // backend dropped here
        };
        // Give the OS a moment to reap.
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert!(!alive(pid), "child {pid} must not outlive the supervisor");
        let _ = std::fs::remove_dir_all(&root);
    }
}
