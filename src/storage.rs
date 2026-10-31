//! Causal Write-Ahead Log (WAL).
//! Cryptographically chains every governance decision for auditability.

pub struct CausalStore;

impl CausalStore {
    pub fn new() -> Self {
        Self
    }

    pub fn append_event(&self, _kind: &str, _payload: &str) -> String {
        // Requires cryptographic chaining implementation.
        unimplemented!("Hash chaining and rotation must be provided by a backend implementation.")
    }
}
