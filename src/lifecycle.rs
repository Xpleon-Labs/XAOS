//! Durable Quarantine and Heal protocols.
//! Ensures agents do not flap by using strict wall-clock hold times.

use crate::types::AgentRecord;
use chrono::{DateTime, Utc};

pub struct QuarantineRecord {
    pub agent_id: uuid::Uuid,
    pub eligible_at: DateTime<Utc>,
    pub heal_attempts: u32,
}

pub struct QuarantineRegistry;

impl QuarantineRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn try_heal_agent(&self, _agent: &mut AgentRecord, _max_faults: u32) -> Option<QuarantineRecord> {
        // Requires check logic implementation.
        unimplemented!("Heal check logic must be provided by a backend implementation.")
    }
}
