// runtime/scheduler/src/task.rs

use serde::{Deserialize, Serialize};

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the lifecycle state of a task
/// inside the AetherFabric orchestration runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    /// Task has been created but not queued
    Created,

    /// Task is waiting in scheduler queue
    Queued,

    /// Task has been assigned to a node
    Assigned,

    /// Task is currently executing
    Executing,

    /// Task completed successfully
    Completed,

    /// Task execution failed
    Failed,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            TaskState::Created => "CREATED",
            TaskState::Queued => "QUEUED",
            TaskState::Assigned => "ASSIGNED",
            TaskState::Executing => "EXECUTING",
            TaskState::Completed => "COMPLETED",
            TaskState::Failed => "FAILED",
        };

        write!(f, "{state}")
    }
}

/// Represents orchestration priority.
///
/// Higher priority tasks may later receive:
/// - faster scheduling
/// - thermal preference
/// - execution guarantees
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let priority = match self {
            TaskPriority::Low => "LOW",
            TaskPriority::Normal => "NORMAL",
            TaskPriority::High => "HIGH",
            TaskPriority::Critical => "CRITICAL",
        };

        write!(f, "{priority}")
    }
}

/// Represents a distributed orchestration workload.
///
/// Tasks are the atomic scheduling units
/// inside the AetherFabric runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier
    pub id: u64,

    /// Human-readable task name
    pub name: String,

    /// Current task lifecycle state
    pub state: TaskState,

    /// Task priority level
    pub priority: TaskPriority,

    /// Node currently assigned to execute task
    pub assigned_node: Option<String>,

    /// Task creation timestamp
    pub created_at: u64,
}

impl Task {
    /// Creates a new task instance.
    pub fn new(id: u64, name: String, priority: TaskPriority) -> Self {
        Self {
            id,
            name,
            state: TaskState::Created,
            priority,
            assigned_node: None,
            created_at: current_timestamp(),
        }
    }

    /// Assigns task to a node.
    pub fn assign_to(&mut self, node_id: String) {
        self.assigned_node = Some(node_id);
        self.state = TaskState::Assigned;
    }

    /// Marks task as queued.
    pub fn queue(&mut self) {
        self.state = TaskState::Queued;
    }

    /// Marks task as executing.
    pub fn start_execution(&mut self) {
        self.state = TaskState::Executing;
    }

    /// Marks task as completed.
    pub fn complete(&mut self) {
        self.state = TaskState::Completed;
    }

    /// Marks task as failed.
    pub fn fail(&mut self) {
        self.state = TaskState::Failed;
    }

    /// Returns formatted task summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Task ID         : {}
Task Name       : {}
Task State      : {}
Priority        : {}
Assigned Node   : {}
Created At      : {}
",
            self.id,
            self.name,
            self.state,
            self.priority,
            self.assigned_node.as_deref().unwrap_or("UNASSIGNED"),
            self.created_at,
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
