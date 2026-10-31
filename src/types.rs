use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    pub kind: String,
    pub requires: Vec<String>,
    pub timeout_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRecord {
    pub id: Uuid,
    pub role: String,
    pub fault_count: u32,
}
