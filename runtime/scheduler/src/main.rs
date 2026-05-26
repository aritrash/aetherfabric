// runtime/scheduler/src/main.rs

mod policy;
mod queue;
mod scheduler;
mod task;

use node_agent::node::Node;

use scheduler::Scheduler;
use task::{Task, TaskPriority};

use tracing::info;

fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    info!("========================================");
    info!("AetherFabric Scheduler Runtime");
    info!("========================================");

    // -------------------------------------------------
    // Simulated Fabric Nodes
    // -------------------------------------------------

    let mut pi_01 = Node::new("PI-01".to_string());
    let mut pi_02 = Node::new("PI-02".to_string());
    let mut pi_03 = Node::new("PI-03".to_string());

    // -------------------------------------------------
    // Simulated Telemetry
    // -------------------------------------------------

    pi_01.telemetry.cpu_usage = 24.0;
    pi_01.telemetry.cpu_temperature_c = 46.0;
    pi_01.telemetry.available_memory_mb = 3200;

    pi_02.telemetry.cpu_usage = 73.0;
    pi_02.telemetry.cpu_temperature_c = 71.0;
    pi_02.telemetry.available_memory_mb = 1800;

    pi_03.telemetry.cpu_usage = 14.0;
    pi_03.telemetry.cpu_temperature_c = 42.0;
    pi_03.telemetry.available_memory_mb = 3600;

    // Update node runtime states
    pi_01.evaluate_status();
    pi_02.evaluate_status();
    pi_03.evaluate_status();

    // -------------------------------------------------
    // Build Fabric
    // -------------------------------------------------

    let fabric = vec![pi_01, pi_02, pi_03];

    // -------------------------------------------------
    // Create Scheduler
    // -------------------------------------------------

    let mut scheduler = Scheduler::new(fabric);

    info!("\n{}", scheduler.summary());

    // -------------------------------------------------
    // Create Simulated Tasks
    // -------------------------------------------------

    let task_01 = Task::new(
        1001,
        "Thermal Telemetry Aggregation".to_string(),
        TaskPriority::Normal,
    );

    let task_02 = Task::new(
        1002,
        "Synthetic Matrix Workload".to_string(),
        TaskPriority::High,
    );

    let task_03 = Task::new(
        1003,
        "Accelerator Dispatch Simulation".to_string(),
        TaskPriority::Critical,
    );

    // -------------------------------------------------
    // Submit Tasks
    // -------------------------------------------------

    scheduler.submit_task(task_01);
    scheduler.submit_task(task_02);
    scheduler.submit_task(task_03);

    info!("========================================");
    info!("Starting Scheduler Cycles");
    info!("========================================");

    // -------------------------------------------------
    // Simulated Scheduler Ticks
    // -------------------------------------------------

    scheduler.tick();
    scheduler.tick();
    scheduler.tick();

    info!("========================================");
    info!("Scheduler Execution Complete");
    info!("========================================");
}
