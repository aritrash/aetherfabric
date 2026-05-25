// runtime/ui/src/state.rs

use egui::Color32;

/// Represents current UI page/view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPage {
    Home,
    Pi,
    Esp,
    Network,
    Cooling,
    Settings,
}

/// Represents system-wide UI status.
///
/// Used for:
/// - footer indicators
/// - orchestration visibility
/// - runtime observability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemHealth {
    Healthy,
    Warning,
    Critical,
    Offline,
}

impl SystemHealth {
    /// Returns health color.
    pub fn color(&self) -> Color32 {
        match self {
            SystemHealth::Healthy => Color32::from_rgb(0, 255, 120),

            SystemHealth::Warning => Color32::from_rgb(255, 180, 0),

            SystemHealth::Critical => Color32::from_rgb(255, 60, 60),

            SystemHealth::Offline => Color32::from_rgb(120, 120, 120),
        }
    }
}

/// Represents a Raspberry Pi runtime node.
#[derive(Debug, Clone)]
pub struct PiNodeState {
    pub name: String,

    pub cpu_usage: f32,

    pub ram_usage: f32,

    pub temperature: f32,

    pub online: bool,

    pub accent: Color32,
}

impl PiNodeState {
    /// Creates new Pi node state.
    pub fn new(name: &str, accent: Color32) -> Self {
        Self {
            name: name.to_string(),

            cpu_usage: 0.0,

            ram_usage: 0.0,

            temperature: 0.0,

            online: true,

            accent,
        }
    }
}

/// Represents ESP accelerator state.
#[derive(Debug, Clone)]
pub struct EspNodeState {
    pub name: String,

    pub utilization: f32,

    pub online: bool,
}

impl EspNodeState {
    /// Creates new ESP state.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),

            utilization: 0.0,

            online: true,
        }
    }
}

/// Represents TEC cooling state.
#[derive(Debug, Clone)]
pub struct CoolingState {
    pub tec_name: String,

    pub power_level: f32,

    pub enabled: bool,
}

impl CoolingState {
    /// Creates cooling subsystem state.
    pub fn new(tec_name: &str) -> Self {
        Self {
            tec_name: tec_name.to_string(),

            power_level: 0.0,

            enabled: false,
        }
    }
}

/// Represents cluster-wide runtime metrics.
#[derive(Debug, Clone)]
pub struct ClusterState {
    pub task_queue_usage: f32,

    pub workload_distribution: f32,

    pub avg_temperature: f32,

    pub uptime_seconds: u64,

    pub health: SystemHealth,
}

impl ClusterState {
    /// Creates default cluster state.
    pub fn new() -> Self {
        Self {
            task_queue_usage: 0.0,

            workload_distribution: 0.0,

            avg_temperature: 0.0,

            uptime_seconds: 0,

            health: SystemHealth::Healthy,
        }
    }
}

/// Global UI state container.
///
/// IMPORTANT:
/// This state ONLY mirrors orchestration data.
///
/// It should NEVER:
/// - own orchestration logic
/// - manage scheduling
/// - control leadership
///
/// UI is strictly observational.
#[derive(Debug)]
pub struct UiState {
    /// Active page/view
    pub current_page: UiPage,

    /// Pi cluster state
    pub pi_nodes: Vec<PiNodeState>,

    /// ESP accelerator state
    pub esp_nodes: Vec<EspNodeState>,

    /// Cooling subsystem state
    pub cooling_nodes: Vec<CoolingState>,

    /// Cluster-wide runtime state
    pub cluster: ClusterState,
}

impl UiState {
    /// Creates initial UI state.
    pub fn new() -> Self {
        Self {
            current_page: UiPage::Home,

            // -------------------------------------------------
            // Simulated Pi Nodes
            // -------------------------------------------------
            pi_nodes: vec![
                PiNodeState {
                    name: "PI-1".to_string(),

                    cpu_usage: 0.58,

                    ram_usage: 0.72,

                    temperature: 52.0,

                    online: true,

                    accent: Color32::from_rgb(0, 255, 120),
                },
                PiNodeState {
                    name: "PI-2".to_string(),

                    cpu_usage: 0.53,

                    ram_usage: 0.50,

                    temperature: 48.0,

                    online: true,

                    accent: Color32::from_rgb(0, 170, 255),
                },
                PiNodeState {
                    name: "PI-3".to_string(),

                    cpu_usage: 0.78,

                    ram_usage: 0.89,

                    temperature: 60.0,

                    online: true,

                    accent: Color32::from_rgb(255, 170, 0),
                },
            ],

            // -------------------------------------------------
            // Simulated ESP Nodes
            // -------------------------------------------------
            esp_nodes: vec![
                EspNodeState {
                    name: "ESP1".to_string(),

                    utilization: 0.28,

                    online: true,
                },
                EspNodeState {
                    name: "ESP2".to_string(),

                    utilization: 0.52,

                    online: true,
                },
                EspNodeState {
                    name: "ESP3".to_string(),

                    utilization: 0.80,

                    online: true,
                },
            ],

            // -------------------------------------------------
            // Cooling Subsystem
            // -------------------------------------------------
            cooling_nodes: vec![
                CoolingState {
                    tec_name: "TEC1".to_string(),

                    power_level: 0.0,

                    enabled: false,
                },
                CoolingState {
                    tec_name: "TEC2".to_string(),

                    power_level: 0.42,

                    enabled: true,
                },
            ],

            // -------------------------------------------------
            // Cluster State
            // -------------------------------------------------
            cluster: ClusterState {
                task_queue_usage: 0.21,

                workload_distribution: 0.53,

                avg_temperature: 53.0,

                uptime_seconds: 8678,

                health: SystemHealth::Healthy,
            },
        }
    }

    /// Returns active online Pi count.
    pub fn online_pi_count(&self) -> usize {
        self.pi_nodes.iter().filter(|node| node.online).count()
    }

    /// Returns active online ESP count.
    pub fn online_esp_count(&self) -> usize {
        self.esp_nodes.iter().filter(|node| node.online).count()
    }

    /// Returns formatted uptime string.
    pub fn uptime_string(&self) -> String {
        let hours = self.cluster.uptime_seconds / 3600;

        let minutes = (self.cluster.uptime_seconds % 3600) / 60;

        let seconds = self.cluster.uptime_seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
