// governance.rs — SnapKitty Governance layer

use serde::{Deserialize, Serialize};

/// Governance policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub blocked_actions: Vec<String>,
    pub required_capabilities: Vec<String>,
}

/// Governance context.
#[derive(Debug, Clone)]
pub struct GovernanceContext {
    pub agent_id: String,
    pub policies: Vec<String>,
}

/// Governance result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceResult {
    pub approved: bool,
    pub reason: String,
    pub policy_checked: String,
}

/// SnapKitty Governance engine.
pub struct GovernanceEngine {
    policies: Vec<Policy>,
}

impl GovernanceEngine {
    pub fn new(policies: Vec<Policy>) -> Self {
        Self { policies }
    }

    /// Evaluate a declaration against governance policies.
    pub fn evaluate(
        &self,
        declaration_hash: &str,
        context: &GovernanceContext,
    ) -> GovernanceResult {
        for policy_name in &context.policies {
            if let Some(policy) = self.policies.iter().find(|p| &p.name == policy_name) {
                // Check blocked actions
                if policy.blocked_actions.contains(&declaration_hash.to_string()) {
                    return GovernanceResult {
                        approved: false,
                        reason: format!("Action blocked by policy: {}", policy_name),
                        policy_checked: policy_name.clone(),
                    };
                }
            }
        }

        GovernanceResult {
            approved: true,
            reason: "All policies passed".to_string(),
            policy_checked: context.policies.join(","),
        }
    }
}
