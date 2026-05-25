// runtime/scheduler/src/policy.rs

use node_agent::node::Node;

/// Represents available workload scheduling policies
/// inside the AetherFabric runtime.
///
/// Policies define HOW orchestration decisions are made.
///
/// This separation allows:
/// - runtime experimentation
/// - adaptive orchestration research
/// - thermal-aware scheduling
/// - future AI-assisted orchestration
#[derive(Debug, Clone)]
pub enum SchedulingPolicy {
    /// Select node with lowest CPU usage
    LowestCpuUsage,

    /// Select node with lowest temperature
    LowestTemperature,

    /// Select node with highest available memory
    HighestAvailableMemory,

    /// Balanced weighted scheduling
    ///
    /// Combines:
    /// - thermal headroom
    /// - CPU availability
    /// - available memory
    Balanced,
}

impl SchedulingPolicy {
    /// Returns human-readable policy name.
    pub fn name(&self) -> &'static str {
        match self {
            SchedulingPolicy::LowestCpuUsage => "LOWEST_CPU_USAGE",

            SchedulingPolicy::LowestTemperature => "LOWEST_TEMPERATURE",

            SchedulingPolicy::HighestAvailableMemory => "HIGHEST_AVAILABLE_MEMORY",

            SchedulingPolicy::Balanced => "BALANCED",
        }
    }

    /// Selects the optimal node according
    /// to the current policy.
    pub fn select_node<'a>(&self, nodes: &'a [Node]) -> Option<&'a Node> {
        match self {
            SchedulingPolicy::LowestCpuUsage => nodes.iter().min_by(|a, b| {
                a.telemetry
                    .cpu_usage
                    .partial_cmp(&b.telemetry.cpu_usage)
                    .unwrap()
            }),

            SchedulingPolicy::LowestTemperature => nodes.iter().min_by(|a, b| {
                a.telemetry
                    .cpu_temperature_c
                    .partial_cmp(&b.telemetry.cpu_temperature_c)
                    .unwrap()
            }),

            SchedulingPolicy::HighestAvailableMemory => nodes.iter().max_by(|a, b| {
                a.telemetry
                    .available_memory_mb
                    .cmp(&b.telemetry.available_memory_mb)
            }),

            SchedulingPolicy::Balanced => nodes.iter().max_by(|a, b| {
                let score_a = balanced_score(a);

                let score_b = balanced_score(b);

                score_a.partial_cmp(&score_b).unwrap()
            }),
        }
    }
}

/// Balanced orchestration score.
///
/// Current weights:
/// - CPU availability: 40%
/// - thermal headroom: 40%
/// - available memory: 20%
fn balanced_score(node: &Node) -> f32 {
    let cpu_factor = (100.0 - node.telemetry.cpu_usage).clamp(0.0, 100.0);

    let thermal_factor = (100.0 - node.telemetry.cpu_temperature_c).clamp(0.0, 100.0);

    let memory_factor =
        ((node.telemetry.available_memory_mb as f32) / 4096.0).clamp(0.0, 1.0) * 100.0;

    (cpu_factor * 0.4) + (thermal_factor * 0.4) + (memory_factor * 0.2)
}
