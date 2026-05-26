// runtime/leadership/src/scoring.rs

use node_agent::node::Node;

use serde::{Deserialize, Serialize};

/// Represents the evaluated orchestration score
/// for a node inside the AetherFabric runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipScore {
    /// Node identifier
    pub node_id: String,

    /// Final weighted leadership score
    pub score: f32,

    /// Individual scoring factors
    pub thermal_factor: f32,
    pub cpu_factor: f32,
    pub memory_factor: f32,
}

impl LeadershipScore {
    /// Creates a leadership score from a node snapshot.
    pub fn evaluate(node: &Node) -> Self {
        // -----------------------------
        // Thermal Factor
        // Lower temperatures are better
        // -----------------------------
        let thermal_factor = (100.0 - node.telemetry.cpu_temperature_c).clamp(0.0_f32, 100.0_f32);

        // -----------------------------
        // CPU Factor
        // Lower CPU usage is better
        // -----------------------------
        let cpu_factor = (100.0 - node.telemetry.cpu_usage).clamp(0.0_f32, 100.0_f32);

        // -----------------------------
        // Memory Factor
        // Normalize available memory
        // -----------------------------
        let memory_factor =
            (node.telemetry.available_memory_mb as f32 / 4096.0).clamp(0.0, 1.0) * 100.0;

        // -----------------------------
        // Weighted Leadership Score
        // -----------------------------
        let score = (thermal_factor * 0.4) + (cpu_factor * 0.4) + (memory_factor * 0.2);

        Self {
            node_id: node.id.clone(),
            score,

            thermal_factor,
            cpu_factor,
            memory_factor,
        }
    }

    /// Returns formatted score summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Leadership Score
-------------------------
Node ID         : {}
Final Score     : {:.2}

Thermal Factor  : {:.2}
CPU Factor      : {:.2}
Memory Factor   : {:.2}
",
            self.node_id, self.score, self.thermal_factor, self.cpu_factor, self.memory_factor,
        )
    }
}
