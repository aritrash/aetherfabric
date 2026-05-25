// runtime/scheduler/src/queue.rs

use crate::task::{Task, TaskState};

use std::collections::VecDeque;

/// Represents the central scheduler task queue
/// for the AetherFabric runtime.
///
/// Initially implemented as a simple FIFO queue.
/// More advanced queueing strategies may later include:
/// - priority scheduling
/// - thermal-aware ordering
/// - workload class separation
/// - adaptive orchestration policies
#[derive(Debug)]
pub struct TaskQueue {
    /// Internal task queue
    queue: VecDeque<Task>,
}

impl TaskQueue {
    /// Creates an empty task queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Pushes a task into the scheduler queue.
    pub fn enqueue(&mut self, mut task: Task) {
        task.queue();
        self.queue.push_back(task);
    }

    /// Removes and returns the next task.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }

    /// Returns immutable reference to next task
    /// without removing it.
    pub fn peek(&self) -> Option<&Task> {
        self.queue.front()
    }

    /// Returns total queued task count.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Clears all queued tasks.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Returns immutable iterator over tasks.
    pub fn iter(&self) -> impl Iterator<Item = &Task> {
        self.queue.iter()
    }

    /// Removes all completed or failed tasks.
    ///
    /// Useful later for:
    /// - queue cleanup
    /// - memory management
    /// - runtime maintenance
    pub fn cleanup_finished(&mut self) {
        self.queue
            .retain(|task| task.state != TaskState::Completed && task.state != TaskState::Failed);
    }

    /// Returns formatted queue summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");
        output.push_str("TASK QUEUE STATE\n");
        output.push_str("========================================\n");

        output.push_str(&format!("Queued Tasks: {}\n\n", self.len()));

        for task in self.queue.iter() {
            output.push_str(&task.summary());
            output.push('\n');
        }

        output
    }
}
