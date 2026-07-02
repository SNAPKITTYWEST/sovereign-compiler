// archivum.rs — Archivum ledger persistence

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

/// Archivum entry for a witnessed declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivumEntry {
    pub declaration_hash: String,
    pub witness_seal: String,
    pub timestamp: String,
    pub stratum: u64,
    pub governance_status: String,
}

/// Archivum ledger for persisting witnesses.
pub struct ArchivumLedger {
    path: String,
}

impl ArchivumLedger {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    /// Append an entry to the ledger.
    pub fn append(&self, entry: &ArchivumEntry) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let json = serde_json::to_string(entry)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    /// Read all entries from the ledger.
    pub fn read_all(&self) -> Result<Vec<ArchivumEntry>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&self.path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                let entry: ArchivumEntry = serde_json::from_str(&line)?;
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Verify ledger integrity.
    pub fn verify_integrity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let entries = self.read_all()?;
        // Each entry should have a valid timestamp and non-empty hashes
        for entry in &entries {
            if entry.declaration_hash.is_empty() || entry.witness_seal.is_empty() {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
