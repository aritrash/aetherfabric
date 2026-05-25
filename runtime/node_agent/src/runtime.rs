// runtime/node_agent/src/runtime.rs

use crate::node::Node;

use anyhow::Result;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

/// Core AetherFabric node runtime.
///
/// Responsible for:
/// - maintaining local node state
/// - collecting telemetry
/// - evaluating runtime status
/// - driving orchestration heartbeat loops
pub struct Runtime {
    /// Local node instance
    pub local_node: Node,

    /// Runtime loop interval in seconds
    pub heartbeat_interval_secs: u64,
}

impl Runtime {
    /// Creates a new runtime instance.
    pub fn new(node_id: String) -> Self {
        Self {
            local_node: Node::new(node_id),
            heartbeat_interval_secs: 1,
        }
    }

    /// Starts the runtime event loop.
    pub async fn start(&mut self) -> Result<()> {
        info!("========================================");
        info!("AetherFabric Runtime Initializing");
        info!("========================================");

        loop {
            self.tick().await?;

            sleep(Duration::from_secs(
                self.heartbeat_interval_secs,
            ))
            .await;
        }
    }

    /// Executes one runtime cycle.
    async fn tick(&mut self) -> Result<()> {
        // Refresh node telemetry
        self.local_node.update_telemetry();

        // Recalculate runtime state
        self.local_node.evaluate_status();

        // Emit runtime logs
        self.log_runtime_state();

        Ok(())
    }

    /// Logs current runtime state.
    fn log_runtime_state(&self) {
        info!("----------------------------------------");
        info!("NODE RUNTIME STATE");
        info!("----------------------------------------");

        info!("\n{}", self.local_node.summary());

        // Emit warnings for critical conditions
        if self.local_node.telemetry.cpu_temperature_c >= 80.0 {
            warn!(
                "Thermal critical threshold exceeded: {:.2} °C",
                self.local_node.telemetry.cpu_temperature_c
            );
        }

        if self.local_node.telemetry.cpu_usage >= 90.0 {
            warn!(
                "Node CPU usage critically high: {:.2}%",
                self.local_node.telemetry.cpu_usage
            );
        }
    }
}