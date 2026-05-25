// simulations/local_cluster/src/node.rs

use tracing::{info, warn};

/// Represents simulated distributed node.
///
/// This abstraction models:
/// - Raspberry Pi orchestration nodes
/// - runtime lifecycle
/// - telemetry evolution
/// - leadership participation
///
/// IMPORTANT:
/// This is NOT the production runtime node.
///
/// It exists purely for:
/// - localhost simulation
/// - orchestration experimentation
/// - distributed systems validation
#[derive(Debug, Clone)]
pub struct SimulatedNode {
    /// Unique node identifier
    pub node_id: String,

    /// Local simulation port
    pub port: u16,

    /// Current CPU usage percentage
    pub cpu_usage: f32,

    /// Current node temperature
    pub temperature: f32,

    /// Current memory usage percentage
    pub memory_usage: f32,

    /// Whether node is online
    pub online: bool,

    /// Whether node acts as cluster leader
    pub is_leader: bool,

    /// Runtime boot timestamp
    pub boot_timestamp: u64,

    /// Simulated heartbeat counter
    pub heartbeat_counter: u64,

    /// Last time node was seen online
    pub last_seen_timestamp: u64,
}

impl SimulatedNode {
    /// Creates simulated node.
    pub fn new(
        node_id: String,
        port: u16,
        current_time_secs: u64,
    ) -> Self {
        Self {
            node_id,

            port,

            cpu_usage: 0.0,

            temperature: 0.0,

            memory_usage: 0.0,

            online: false,

            is_leader: false,

            boot_timestamp:
                current_time_secs,

            heartbeat_counter: 0,
            
            last_seen_timestamp: current_time_secs,
        }
    }

    /// Starts simulated node runtime.
    pub fn start(&mut self, current_time_secs: u64) {
        if self.online {
            warn!(
                "Node [{}] already online",
                self.node_id
            );

            return;
        }

        self.online = true;

        self.boot_timestamp =
            current_time_secs;

        info!(
            "Node [{}] started on port [{}]",
            self.node_id,
            self.port
        );
    }

    /// Stops simulated node runtime.
    pub fn stop(&mut self) {
        if !self.online {
            warn!(
                "Node [{}] already offline",
                self.node_id
            );

            return;
        }

        self.online = false;

        self.is_leader = false;

        info!(
            "Node [{}] stopped",
            self.node_id
        );
    }

    /// Simulates heartbeat emission.
    pub fn heartbeat(
        &mut self,
        current_time_secs: u64,
    ) {
        if !self.online {
            warn!(
                "Offline node [{}] cannot emit heartbeat",
                self.node_id
            );

            return;
        }

        self.heartbeat_counter += 1;
        self.last_seen_timestamp = current_time_secs;

        info!(
            "[{}] Heartbeat #{}",
            self.node_id,
            self.heartbeat_counter
        );
    }

    /// Simulates telemetry evolution.
    ///
    /// Future versions may:
    /// - use stochastic telemetry
    /// - replay benchmark traces
    /// - emulate workload pressure
    pub fn tick(
        &mut self,
        current_time_secs: u64,
    ) {
        if !self.online {
            return;
        }

        // -------------------------------------------------
        // Simulated workload progression
        // -------------------------------------------------

        self.cpu_usage += 2.4;

        if self.cpu_usage > 100.0 {
            self.cpu_usage = 10.0;
        }

        self.memory_usage =
            25.0 + (self.cpu_usage * 0.45);

        self.temperature =
            35.0 + (self.cpu_usage * 0.58);

        self.heartbeat(current_time_secs);
    }

    /// Returns formatted runtime summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
SIMULATED NODE
========================================

Node ID           : {}
Port              : {}

Online            : {}
Leader            : {}

CPU Usage         : {:.2}%
Memory Usage      : {:.2}%
Temperature       : {:.2}C

Heartbeats Sent   : {}
Last Seen Time    : {}

Boot Timestamp    : {}

",
            self.node_id,
            self.port,

            self.online,
            self.is_leader,

            self.cpu_usage,
            self.memory_usage,
            self.temperature,

            self.heartbeat_counter,
            self.last_seen_timestamp,

            self.boot_timestamp,
        )
    }
}