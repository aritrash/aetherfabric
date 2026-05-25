// runtime/scheduler/src/scheduler.rs

use crate::queue::TaskQueue;
use crate::task::{Task, TaskState};

use node_agent::node::Node;

use tracing::{info, warn};

/// Core orchestration scheduler for the
/// AetherFabric runtime.
///
/// Responsibilities:
/// - task queue management
/// - workload assignment
/// - node selection
/// - task lifecycle tracking
pub struct Scheduler {
    /// Central task queue
    pub queue: TaskQueue,

    /// Known orchestration nodes
    pub nodes: Vec<Node>,
}

impl Scheduler {
    /// Creates a new scheduler instance.
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            queue: TaskQueue::new(),
            nodes,
        }
    }

    /// Adds a task into the scheduling queue.
    pub fn submit_task(&mut self, task: Task) {
        info!("Submitting task [{}] into scheduler queue", task.id);

        self.queue.enqueue(task);
    }

    /// Executes one scheduler cycle.
    ///
    /// Current behavior:
    /// - dequeue task
    /// - select best node
    /// - assign workload
    /// - simulate execution lifecycle
    pub fn tick(&mut self) {
        let mut task = match self.queue.dequeue() {
            Some(task) => task,
            None => {
                info!("Scheduler queue empty");
                return;
            }
        };

        let target_node = match self.select_best_node() {
            Some(node) => node,
            None => {
                warn!("No valid orchestration nodes available");
                return;
            }
        };

        // Assign task
        task.assign_to(target_node.id.clone());

        info!("Task [{}] assigned to node [{}]", task.id, target_node.id);

        // Simulated execution lifecycle
        self.execute_task(&mut task);
    }

    /// Selects best available node for workload assignment.
    ///
    /// Initial policy:
    /// lowest CPU utilization wins.
    fn select_best_node(&self) -> Option<&Node> {
        self.nodes.iter().min_by(|a, b| {
            a.telemetry
                .cpu_usage
                .partial_cmp(&b.telemetry.cpu_usage)
                .unwrap()
        })
    }

    /// Simulates task execution lifecycle.
    ///
    /// Real distributed execution will later:
    /// - dispatch over network
    /// - synchronize runtime state
    /// - communicate with accelerators
    fn execute_task(&self, task: &mut Task) {
        task.start_execution();

        info!(
            "Task [{}] executing on [{}]",
            task.id,
            task.assigned_node.as_deref().unwrap_or("UNKNOWN")
        );

        // Placeholder execution simulation
        //
        // Future:
        // actual workload dispatch
        // distributed execution
        // accelerator integration
        task.complete();

        info!("Task [{}] completed successfully", task.id);
    }

    /// Returns scheduler queue depth.
    pub fn queue_depth(&self) -> usize {
        self.queue.len()
    }

    /// Returns formatted scheduler summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
SCHEDULER STATE
========================================

Known Nodes : {}
Queue Depth : {}

",
            self.nodes.len(),
            self.queue_depth(),
        )
    }
}
