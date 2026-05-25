// runtime/protocol/src/main.rs

mod heartbeat;
mod leadership;
mod message;
mod task;
mod telemetry;

use heartbeat::{HeartbeatPacket, NodeStatus, TelemetryPacket as HeartbeatTelemetry};

use leadership::{LeadershipPacket, LeadershipTransition};

use message::RuntimeMessage;

use task::{TaskOpcode, TaskPacket, TaskPriority};

use telemetry::{TelemetryCategory, TelemetryMetric, TelemetryPacket};

use tracing::info;

fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    info!("========================================");
    info!("AetherFabric Protocol Runtime");
    info!("========================================");

    // -------------------------------------------------
    // HEARTBEAT PACKET
    // -------------------------------------------------

    let heartbeat_telemetry = HeartbeatTelemetry {
        cpu_usage: 21.5,
        cpu_temperature_c: 47.2,
        available_memory_mb: 3200,
        memory_usage_mb: 896,
    };

    let heartbeat = HeartbeatPacket::new(
        "PI-01".to_string(),
        NodeStatus::Active,
        heartbeat_telemetry,
        true,
    );

    info!("\n{}", heartbeat.summary());

    // -------------------------------------------------
    // LEADERSHIP PACKET
    // -------------------------------------------------

    let leadership = LeadershipPacket::new(
        "PI-01".to_string(),
        Some("PI-02".to_string()),
        LeadershipTransition::Reelection,
        4,
    );

    info!("\n{}", leadership.summary());

    // -------------------------------------------------
    // TASK PACKET
    // -------------------------------------------------

    let mut task = TaskPacket::new(
        1001,
        "Synthetic Inference Workload".to_string(),
        TaskOpcode::Inference,
        TaskPriority::High,
        "PI-01".to_string(),
    );

    task.assign_to("PI-03".to_string());

    task.start_execution();

    task.complete();

    info!("\n{}", task.summary());

    // -------------------------------------------------
    // TELEMETRY PACKET
    // -------------------------------------------------

    let mut telemetry = TelemetryPacket::new("PI-03".to_string());

    telemetry.add_metric(TelemetryMetric::new(
        "cpu_usage".to_string(),
        TelemetryCategory::Compute,
        42.6,
        "%".to_string(),
    ));

    telemetry.add_metric(TelemetryMetric::new(
        "cpu_temperature".to_string(),
        TelemetryCategory::Thermal,
        58.4,
        "C".to_string(),
    ));

    telemetry.add_metric(TelemetryMetric::new(
        "power_draw".to_string(),
        TelemetryCategory::Power,
        18.2,
        "W".to_string(),
    ));

    info!("\n{}", telemetry.summary());

    // -------------------------------------------------
    // RUNTIME MESSAGE WRAPPING
    // -------------------------------------------------

    let heartbeat_message = RuntimeMessage::Heartbeat(heartbeat.clone());

    let leadership_message = RuntimeMessage::Leadership(leadership.clone());

    let task_message = RuntimeMessage::Task(task.clone());

    info!("\n{}", heartbeat_message.summary());

    info!("\n{}", leadership_message.summary());

    info!("\n{}", task_message.summary());

    // -------------------------------------------------
    // JSON SERIALIZATION TESTS
    // -------------------------------------------------

    info!("========================================");
    info!("JSON SERIALIZATION TEST");
    info!("========================================");

    if let Ok(json) = heartbeat_message.to_json() {
        info!("Serialized HEARTBEAT message:\n{}", json);

        let deserialized: RuntimeMessage = RuntimeMessage::from_json(&json).unwrap();

        info!(
            "\nDeserialized Message Type: {}",
            deserialized.message_type()
        );
    }

    info!("========================================");
    info!("Protocol Runtime Complete");
    info!("========================================");
}
