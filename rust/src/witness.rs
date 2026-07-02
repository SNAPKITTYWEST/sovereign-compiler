// witness.rs — WORM Witness generation

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

/// WORM Witness for a Fontana declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WORMWitness {
    pub declaration_hash: String,
    pub seal: String,
    pub timestamp: String,
    pub content_hash: String,
}

impl WORMWitness {
    /// Generate a WORM witness for a declaration.
    pub fn generate(declaration_hash: &str, content: &str) -> Self {
        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            hex::encode(hasher.finalize())
        };

        let seal_input = format!("WORM:{}:{}", declaration_hash, content_hash);
        let seal = {
            let mut hasher = Sha256::new();
            hasher.update(seal_input.as_bytes());
            hex::encode(hasher.finalize())
        };

        WORMWitness {
            declaration_hash: declaration_hash.to_string(),
            seal,
            timestamp: chrono::Utc::now().to_rfc3339(),
            content_hash,
        }
    }

    /// Verify a WORM witness.
    pub fn verify(&self) -> bool {
        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(self.content_hash.as_bytes());
            hex::encode(hasher.finalize())
        };

        let expected_seal = format!("WORM:{}:{}", self.declaration_hash, content_hash);
        let mut hasher = Sha256::new();
        hasher.update(expected_seal.as_bytes());
        let expected = hex::encode(hasher.finalize());

        self.seal == expected
    }
}
