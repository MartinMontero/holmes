//! Lock 2e — AC-WP weight provenance (spec §7 Phase 2 lock 2e; security.md
//! honest-limits: "verifying downloaded Tier-2 open weights
//! (checksum/signature/attestation, fail closed)").
//!
//! Any downloaded model weight file is verified **before load**, failing
//! closed on mismatch. "Before load" is enforced structurally: a loader
//! accepts only a [`VerifiedWeights`] token, and the sole way to mint one
//! is [`verify_weights`] returning `Ok`. There is no path from a raw file
//! to a load without passing the check — an unverified or tampered weight
//! is unrepresentable at the point of load.
//!
//! Checksum is SHA-256 (required). A detached signature check is optional
//! and pluggable via [`SignatureVerifier`]; when a manifest carries a
//! signature requirement and no verifier confirms it, verification fails
//! closed.

use sha2::{Digest, Sha256};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ProvenanceError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    ChecksumMismatch {
        expected: String,
        actual: String,
    },
    SizeMismatch {
        expected: u64,
        actual: u64,
    },
    SignatureRequiredButUnverified,
    MalformedExpectedDigest(String),
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProvenanceError::Read { path, source } => {
                write!(f, "weight read failed for {}: {source}", path.display())
            }
            ProvenanceError::ChecksumMismatch { expected, actual } => write!(
                f,
                "FAIL CLOSED: weight sha256 {actual} does not match expected {expected}"
            ),
            ProvenanceError::SizeMismatch { expected, actual } => write!(
                f,
                "FAIL CLOSED: weight size {actual} bytes does not match expected {expected}"
            ),
            ProvenanceError::SignatureRequiredButUnverified => {
                write!(
                    f,
                    "FAIL CLOSED: manifest requires a signature but none verified"
                )
            }
            ProvenanceError::MalformedExpectedDigest(s) => {
                write!(
                    f,
                    "rejected: expected digest '{s}' is not 64 lowercase hex chars"
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

/// What a weight file must match to be loadable.
#[derive(Debug, Clone)]
pub struct WeightManifest {
    /// Lowercase hex SHA-256, 64 chars.
    pub sha256: String,
    /// Optional exact byte size (a cheap early mismatch signal).
    pub size_bytes: Option<u64>,
    /// True when a valid detached signature is also required.
    pub require_signature: bool,
    /// Where the weight was obtained (recorded, not trusted).
    pub source: String,
}

/// Pluggable detached-signature check (e.g. minisign/cosign in a later
/// phase). Returns true only on a cryptographically valid signature.
pub trait SignatureVerifier {
    fn verify(&self, weight_path: &Path, manifest: &WeightManifest) -> bool;
}

/// Proof that a weight file passed verification. Holds the verified path;
/// only `verify_weights` constructs it, so a loader requiring this type
/// cannot be handed an unverified file.
#[derive(Debug, Clone)]
pub struct VerifiedWeights {
    path: PathBuf,
    sha256: String,
}

impl VerifiedWeights {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Verify a weight file against its manifest. Returns [`VerifiedWeights`]
/// only when the checksum (and size, and signature if required) all pass;
/// otherwise fails closed.
pub fn verify_weights(
    path: &Path,
    manifest: &WeightManifest,
    signature: Option<&dyn SignatureVerifier>,
) -> Result<VerifiedWeights, ProvenanceError> {
    if !is_hex64(&manifest.sha256) {
        return Err(ProvenanceError::MalformedExpectedDigest(
            manifest.sha256.clone(),
        ));
    }
    let bytes = std::fs::read(path).map_err(|e| ProvenanceError::Read {
        path: path.to_path_buf(),
        source: e,
    })?;
    if let Some(expected_size) = manifest.size_bytes {
        if bytes.len() as u64 != expected_size {
            return Err(ProvenanceError::SizeMismatch {
                expected: expected_size,
                actual: bytes.len() as u64,
            });
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let actual = hex_lower(&hasher.finalize());
    if actual != manifest.sha256 {
        return Err(ProvenanceError::ChecksumMismatch {
            expected: manifest.sha256.clone(),
            actual,
        });
    }
    if manifest.require_signature {
        let ok = signature.map(|v| v.verify(path, manifest)).unwrap_or(false);
        if !ok {
            return Err(ProvenanceError::SignatureRequiredButUnverified);
        }
    }
    Ok(VerifiedWeights {
        path: path.to_path_buf(),
        sha256: actual,
    })
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xf) as u32, 16).unwrap());
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_tmp(name: &str, content: &[u8]) -> PathBuf {
        let p = std::env::temp_dir().join(format!("holmes-weight-{}-{name}", std::process::id()));
        let mut f = std::fs::File::create(&p).unwrap();
        f.write_all(content).unwrap();
        p
    }

    fn sha256_hex(content: &[u8]) -> String {
        let mut h = Sha256::new();
        h.update(content);
        hex_lower(&h.finalize())
    }

    #[test]
    fn matching_checksum_verifies_tamper_fails_closed() {
        let content = b"pretend-gguf-weights-v1";
        let path = write_tmp("ok.bin", content);
        let manifest = WeightManifest {
            sha256: sha256_hex(content),
            size_bytes: Some(content.len() as u64),
            require_signature: false,
            source: "fixture".into(),
        };
        let verified = verify_weights(&path, &manifest, None).expect("valid weights verify");
        assert_eq!(verified.sha256(), &manifest.sha256);

        // Tamper the file on disk at equal length (23 bytes) so the
        // checksum path — not the size path — is what fails closed.
        assert_eq!(b"pretend-gguf-weights-v2".len(), content.len());
        std::fs::write(&path, b"pretend-gguf-weights-v2").unwrap();
        assert!(matches!(
            verify_weights(&path, &manifest, None).unwrap_err(),
            ProvenanceError::ChecksumMismatch { .. }
        ));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn size_mismatch_and_bad_digest_fail_closed() {
        let content = b"abc";
        let path = write_tmp("size.bin", content);
        let mut manifest = WeightManifest {
            sha256: sha256_hex(content),
            size_bytes: Some(999),
            require_signature: false,
            source: "fixture".into(),
        };
        assert!(matches!(
            verify_weights(&path, &manifest, None).unwrap_err(),
            ProvenanceError::SizeMismatch { .. }
        ));
        manifest.sha256 = "not-hex".into();
        assert!(matches!(
            verify_weights(&path, &manifest, None).unwrap_err(),
            ProvenanceError::MalformedExpectedDigest(_)
        ));
        let _ = std::fs::remove_file(&path);
    }

    struct AlwaysReject;
    impl SignatureVerifier for AlwaysReject {
        fn verify(&self, _p: &Path, _m: &WeightManifest) -> bool {
            false
        }
    }

    #[test]
    fn required_signature_with_no_valid_verifier_fails_closed() {
        let content = b"signed-weights";
        let path = write_tmp("sig.bin", content);
        let manifest = WeightManifest {
            sha256: sha256_hex(content),
            size_bytes: None,
            require_signature: true,
            source: "fixture".into(),
        };
        // Checksum matches, but signature required and none verifies.
        assert!(matches!(
            verify_weights(&path, &manifest, None).unwrap_err(),
            ProvenanceError::SignatureRequiredButUnverified
        ));
        assert!(matches!(
            verify_weights(&path, &manifest, Some(&AlwaysReject)).unwrap_err(),
            ProvenanceError::SignatureRequiredButUnverified
        ));
        let _ = std::fs::remove_file(&path);
    }
}
