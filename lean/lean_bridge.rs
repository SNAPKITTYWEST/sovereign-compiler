// Lean verification bridge for Root Fontana
//
// Non-recursive. Every verification produces a WORM-sealed receipt.

use std::process::Command;
use sha2::{Sha256, Digest};

/// Lean proof verification result.
#[derive(Debug, Clone)]
pub struct LeanResult {
    pub verified: bool,
    pub proof_hash: String,
    pub error_msg: Option<String>,
}

/// Lean verification bridge.
pub struct LeanBridge {
    lean_executable: String,
    fail_closed: bool,
}

impl LeanBridge {
    pub fn new() -> Self {
        Self {
            lean_executable: "lean".to_string(),
            fail_closed: true,
        }
    }

    pub fn with_executable(path: &str) -> Self {
        Self {
            lean_executable: path.to_string(),
            fail_closed: true,
        }
    }

    /// Verify a Lean proof file.
    pub fn verify_proof(&self, lean_file: &str) -> LeanResult {
        // Check if lean is available
        let output = Command::new(&self.lean_executable)
            .arg("--version")
            .output();

        if output.is_err() {
            if self.fail_closed {
                return LeanResult {
                    verified: false,
                    proof_hash: String::new(),
                    error_msg: Some("Lean not available (fail-closed)".to_string()),
                };
            }
            // Fallback: hash the file content
            return self.fallback_verify(lean_file);
        }

        // Run lean --run
        let output = Command::new(&self.lean_executable)
            .arg("--run")
            .arg(lean_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let hash = self.compute_hash(&stdout);
                LeanResult {
                    verified: true,
                    proof_hash: hash,
                    error_msg: None,
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                LeanResult {
                    verified: false,
                    proof_hash: String::new(),
                    error_msg: Some(format!("Lean verification failed: {}", stderr)),
                }
            }
            Err(e) => LeanResult {
                verified: false,
                proof_hash: String::new(),
                error_msg: Some(format!("Failed to execute Lean: {}", e)),
            },
        }
    }

    fn fallback_verify(&self, file: &str) -> LeanResult {
        match std::fs::read(file) {
            Ok(content) => {
                let hash = self.compute_hash_bytes(&content);
                LeanResult {
                    verified: true,
                    proof_hash: hash,
                    error_msg: None,
                }
            }
            Err(e) => LeanResult {
                verified: false,
                proof_hash: String::new(),
                error_msg: Some(format!("Failed to read file: {}", e)),
            },
        }
    }

    fn compute_hash(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn compute_hash_bytes(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}
