// runtime/protocol/src/task.rs

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents distributed task lifecycle state.
///
/// This protocol-level state is synchronized
/// across orchestration nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Created,
    Queued,
    Assigned,
    Executing,
    Completed,
    Failed,
}

/// Represents orchestration priority level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Represents accelerator/runtime operation type.
///
/// This abstraction allows:
/// - distributed orchestration
/// - accelerator dispatch
/// - workload classification
/// - execution specialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskOpcode {
    /// Generic compute operation
    Compute,

    /// Lightweight inference operation
    Inference,

    /// Telemetry aggregation task
    TelemetryAggregation,

    /// Thermal management operation
    ThermalManagement,

    /// Accelerator dispatch operation
    AcceleratorDispatch,

    /// Synthetic benchmarking workload
    SyntheticBenchmark,
}

/// Distributed orchestration task packet.
///
/// This packet synchronizes:
/// - workload state
/// - execution ownership
/// - orchestration lifecycle
/// - accelerator dispatch metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPacket {
    /// Protocol version
    pub protocol_version: String,

    /// Unique task identifier
    pub task_id: u64,

    /// Human-readable task name
    pub task_name: String,

    /// Runtime operation type
    pub opcode: TaskOpcode,

    /// Current task state
    pub state: TaskState,

    /// Task priority level
    pub priority: TaskPriority,

    /// Assigned orchestration node
    pub assigned_node: Option<String>,

    /// Source node which generated task
    pub source_node: String,

    /// Task creation timestamp
    pub created_at: u64,

    /// Last task update timestamp
    pub updated_at: u64,
}

impl TaskPacket {
    /// Creates a new task packet.
    pub fn new(
        task_id: u64,
        task_name: String,
        opcode: TaskOpcode,
        priority: TaskPriority,
        source_node: String,
    ) -> Self {
        let timestamp = current_timestamp();

        Self {
            protocol_version: String::from("0.1.0"),

            task_id,
            task_name,

            opcode,

            state: TaskState::Created,

            priority,

            assigned_node: None,

            source_node,

            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    /// Assigns task to orchestration node.
    pub fn assign_to(&mut self, node_id: String) {
        self.assigned_node = Some(node_id);
        self.state = TaskState::Assigned;
        self.updated_at = current_timestamp();
    }

    /// Marks task as executing.
    pub fn start_execution(&mut self) {
        self.state = TaskState::Executing;
        self.updated_at = current_timestamp();
    }

    /// Marks task as completed.
    pub fn complete(&mut self) {
        self.state = TaskState::Completed;
        self.updated_at = current_timestamp();
    }

    /// Marks task as failed.
    pub fn fail(&mut self) {
        self.state = TaskState::Failed;
        self.updated_at = current_timestamp();
    }

    /// Serializes task packet into JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes task packet from JSON.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Returns formatted task summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
TASK PACKET
========================================

Protocol Version : {}

Task ID           : {}
Task Name         : {}

Opcode            : {:?}
State             : {:?}
Priority          : {:?}

Assigned Node     : {}
Source Node       : {}

Created At        : {}
Updated At        : {}

",
            self.protocol_version,
            self.task_id,
            self.task_name,
            self.opcode,
            self.state,
            self.priority,
            self.assigned_node.as_deref().unwrap_or("UNASSIGNED"),
            self.source_node,
            self.created_at,
            self.updated_at,
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
