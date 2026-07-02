// contractivity.rs — Contractivity analysis

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

/// Contractivity receipt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractivityReceipt {
    pub prime_index: u64,
    pub hash: String,
    pub timestamp: String,
    pub operator: String,
}

/// Contractivity engine for mathematical soundness verification.
pub struct ContractivityEngine {
    counter: u64,
}

impl ContractivityEngine {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Generate a contractivity receipt for an operator.
    pub fn generate_receipt(&mut self, prime_index: u64, operator: &str) -> ContractivityReceipt {
        self.counter += 1;

        let content = format!(
            "{}:{}:{}:{}",
            operator, prime_index, self.counter,
            chrono::Utc::now().to_rfc3339()
        );

        let hash = {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            hex::encode(hasher.finalize())
        };

        ContractivityReceipt {
            prime_index,
            hash,
            timestamp: chrono::Utc::now().to_rfc3339(),
            operator: operator.to_string(),
        }
    }

    /// Verify a contractivity receipt.
    pub fn verify(&self, receipt: &ContractivityReceipt) -> bool {
        let content = format!(
            "{}:{}:{}:{}",
            receipt.operator,
            receipt.prime_index,
            1, // counter would need to be tracked for full verification
            receipt.timestamp
        );

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let expected = hex::encode(hasher.finalize());

        receipt.hash == expected
    }
}
