// runtime/protocol/src/leadership.rs

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Leadership transition type.
///
/// Used to indicate WHY a leadership
/// change occurred inside the fabric.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeadershipTransition {
    /// Initial leader election
    InitialElection,

    /// Runtime-triggered reelection
    Reelection,

    /// Thermal migration triggered
    /// leadership reassignment
    ThermalFailover,

    /// Node failure triggered
    /// leadership reassignment
    NodeFailure,

    /// Manual/debug reassignment
    ManualOverride,
}

/// Distributed leadership announcement packet.
///
/// Synchronizes:
/// - orchestration ownership
/// - leadership epochs
/// - failover events
/// - runtime authority
///
/// This packet is critical for maintaining
/// distributed orchestration consistency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipPacket {
    /// Protocol version
    pub protocol_version: String,

    /// Current orchestration leader
    pub leader_id: String,

    /// Previous orchestration leader
    pub previous_leader: Option<String>,

    /// Leadership transition type
    pub transition: LeadershipTransition,

    /// Leadership epoch
    ///
    /// Incremented whenever leadership changes.
    pub epoch: u64,

    /// Leadership announcement timestamp
    pub timestamp: u64,
}

impl LeadershipPacket {
    /// Creates a new leadership packet.
    pub fn new(
        leader_id: String,
        previous_leader: Option<String>,
        transition: LeadershipTransition,
        epoch: u64,
    ) -> Self {
        Self {
            protocol_version: String::from("0.1.0"),

            leader_id,
            previous_leader,
            transition,
            epoch,

            timestamp: current_timestamp(),
        }
    }

    /// Serializes leadership packet into JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes leadership packet from JSON.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Returns formatted leadership summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
LEADERSHIP PACKET
========================================

Protocol Version : {}

Leader ID         : {}
Previous Leader   : {}

Transition Type   : {:?}

Epoch             : {}
Timestamp         : {}

",
            self.protocol_version,
            self.leader_id,
            self.previous_leader.as_deref().unwrap_or("NONE"),
            self.transition,
            self.epoch,
            self.timestamp,
        )
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
