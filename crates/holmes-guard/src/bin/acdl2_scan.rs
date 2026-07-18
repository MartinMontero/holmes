//! AC-DL-2 gate CLI. Deterministic: same input, same verdict.
//! Default discovers and walks every lockfile in the tree (§1); `--lockfile`
//! restricts to one (used by the positive control's planted fixture).
//! Exit 0 = clean, 1 = violations, 2 = usage or I/O error.

use holmes_guard::scan;
use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!("usage: acdl2-scan --root <dir> [--lockfile <path>]...");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut root: Option<PathBuf> = None;
    let mut lockfiles: Vec<PathBuf> = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--lockfile" => match args.next() {
                Some(p) => lockfiles.push(PathBuf::from(p)),
                None => return usage(),
            },
            _ => return usage(),
        }
    }
    let Some(root) = root else {
        return usage();
    };

    match scan::scan_workspace_full(&root, &lockfiles) {
        Ok(report) => {
            println!("AC-DL-2 deterministic dependency-tree gate");
            println!("  lockfiles walked:  {}", report.lockfiles_walked.len());
            for lf in &report.lockfiles_walked {
                println!("    - {lf}");
            }
            println!("  packages scanned:  {}", report.packages_scanned);
            println!("  files scanned:     {}", report.files_scanned);
            for (path, reason) in &report.exemptions_applied {
                println!("  exemption applied: {path} — {reason}");
            }
            if report.clean() {
                println!("  verdict: CLEAN");
                ExitCode::SUCCESS
            } else {
                for v in &report.violations {
                    println!(
                        "  VIOLATION [{:?}] '{}' at {} — {}",
                        v.kind,
                        v.subject,
                        v.location,
                        v.path_note()
                    );
                }
                println!("  verdict: FAIL ({} violations)", report.violations.len());
                ExitCode::from(1)
            }
        }
        Err(err) => {
            eprintln!("acdl2-scan: {err}");
            ExitCode::from(2)
        }
    }
}
