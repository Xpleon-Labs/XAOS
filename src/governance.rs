//! Deterministic Governance Gate.
//! Evaluates high-impact tasks against the configured policy engine.

use crate::types::{TaskSpec, AgentRecord};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GovernanceAction {
    Nominal = 0x00,
    Monitor = 0x10,
    Throttle = 0x50,
    EmergencyStop = 0x99,
    HardHalt = 0xFF,
}

pub struct GovernanceDecision {
    pub allowed: bool,
    pub action: GovernanceAction,
    pub rule_id: String,
}

pub trait GovernanceGate: Send + Sync {
    fn evaluate(&self, agent: &AgentRecord, task: &TaskSpec) -> GovernanceDecision;
}

pub struct CompositeGate;

impl GovernanceGate for CompositeGate {
    fn evaluate(&self, _agent: &AgentRecord, _task: &TaskSpec) -> GovernanceDecision {
        // Implementation requires a configured policy engine backend.
        unimplemented!("Governance logic must be provided by a backend implementation.")
    }
}
