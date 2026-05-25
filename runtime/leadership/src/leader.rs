// runtime/leadership/src/leader.rs

use serde::{Deserialize, Serialize};

/// Represents the current orchestration leadership state
/// of the AetherFabric runtime.
///
/// Only one node should act as orchestration leader
/// at any given time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderState {
    /// ID of the current leader node
    pub current_leader: Option<String>,

    /// UNIX timestamp of the last successful election
    pub last_election_timestamp: u64,

    /// Leadership epoch counter
    ///
    /// Incremented every time leadership changes.
    pub epoch: u64,
}

impl LeaderState {
    /// Creates a new empty leadership state.
    pub fn new() -> Self {
        Self {
            current_leader: None,
            last_election_timestamp: 0,
            epoch: 0,
        }
    }

    /// Updates leadership ownership.
    pub fn update_leader(&mut self, leader_id: String, timestamp: u64) {
        let leadership_changed = match &self.current_leader {
            Some(current) => current != &leader_id,
            None => true,
        };

        if leadership_changed {
            self.current_leader = Some(leader_id);
            self.last_election_timestamp = timestamp;
            self.epoch += 1;
        }
    }

    /// Clears leadership state.
    pub fn clear(&mut self) {
        self.current_leader = None;
    }

    /// Returns true if a leader currently exists.
    pub fn has_leader(&self) -> bool {
        self.current_leader.is_some()
    }

    /// Returns current leader ID if available.
    pub fn leader_id(&self) -> Option<&String> {
        self.current_leader.as_ref()
    }

    /// Returns formatted leadership summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Leadership State
-------------------------
Current Leader : {}
Epoch           : {}
Last Election   : {}
",
            self.current_leader.as_deref().unwrap_or("NONE"),
            self.epoch,
            self.last_election_timestamp
        )
    }
}
