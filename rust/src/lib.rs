// Root Fontana — Constitutional compiler layer for SnapKitty
//
// Transforms symbolic declarations into verified execution plans.
// Every declaration passes through:
// - Fontana parsing
// - AST admissibility validation
// - Stratum verification
// - Contractivity analysis
// - SnapKitty Governance
// - Lean verification
// - Rust execution
// - WORM witness generation
// - Archivum ledger persistence
// - SnapKitty Observatory telemetry

pub mod witness;
pub mod archivum;
pub mod governance;
pub mod contractivity;
pub mod observatory;
pub mod execution;

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

/// Errors from the Fontana compiler.
#[derive(Error, Debug)]
pub enum FontanaError {
    #[error("parsing failed: {0}")]
    ParseFailed(String),

    #[error("admissibility rejected: {0}")]
    AdmissibilityRejected(String),

    #[error("stratum boundary violation: {0}")]
    StratumViolation(String),

    #[error("contractivity check failed: {0}")]
    ContractivityFailed(String),

    #[error("governance rejected: {0}")]
    GovernanceRejected(String),

    #[error("lean verification failed: {0}")]
    LeanVerificationFailed(String),

    #[error("execution failed: {0}")]
    ExecutionFailed(String),
}

/// Unified Witness — the single output of every Fontana workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedWitness {
    pub declaration_hash: String,
    pub stratum: u64,
    pub contractivity_seal: String,
    pub governance_status: String,
    pub lean_proof_hash: Option<String>,
    pub execution_result: Option<String>,
    pub worm_seal: String,
    pub timestamp: String,
}

/// Fontana declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declaration {
    pub name: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

/// Fontana pipeline result.
#[derive(Debug)]
pub struct PipelineResult {
    pub witness: UnifiedWitness,
    pub archivum_entry: archivum::ArchivumEntry,
}
