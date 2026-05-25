// runtime/protocol/src/message.rs

use serde::{Deserialize, Serialize};

use crate::heartbeat::HeartbeatPacket;
use crate::leadership::LeadershipPacket;
use crate::task::TaskPacket;

/// Represents all distributed runtime message types
/// supported by the AetherFabric protocol layer.
///
/// This becomes the universal orchestration
/// communication abstraction.
///
/// Future networking layers should ONLY exchange:
/// - serialized RuntimeMessage packets
///
/// This design cleanly separates:
/// - protocol semantics
/// from:
/// - transport implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeMessage {
    /// Node heartbeat synchronization
    Heartbeat(HeartbeatPacket),

    /// Leadership transition/update
    Leadership(LeadershipPacket),

    /// Distributed workload packet
    Task(TaskPacket),
}

impl RuntimeMessage {
    /// Returns message type name.
    pub fn message_type(&self) -> &'static str {
        match self {
            RuntimeMessage::Heartbeat(_) => "HEARTBEAT",

            RuntimeMessage::Leadership(_) => "LEADERSHIP",

            RuntimeMessage::Task(_) => "TASK",
        }
    }

    /// Serializes runtime message into JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes runtime message from JSON.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Returns formatted runtime message summary.
    pub fn summary(&self) -> String {
        match self {
            RuntimeMessage::Heartbeat(packet) => {
                format!(
                    "\
========================================
RUNTIME MESSAGE
========================================

Message Type : HEARTBEAT

{}
",
                    packet.summary()
                )
            }

            RuntimeMessage::Leadership(packet) => {
                format!(
                    "\
========================================
RUNTIME MESSAGE
========================================

Message Type : LEADERSHIP

{}
",
                    packet.summary()
                )
            }

            RuntimeMessage::Task(packet) => {
                format!(
                    "\
========================================
RUNTIME MESSAGE
========================================

Message Type : TASK

{}
",
                    packet.summary()
                )
            }
        }
    }
}
