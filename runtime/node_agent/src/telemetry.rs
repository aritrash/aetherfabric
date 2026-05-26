// runtime/node_agent/src/telemetry.rs

use serde::{Deserialize, Serialize};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::System;

/// Represents a telemetry snapshot collected from the local node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    /// CPU utilization percentage
    pub cpu_usage: f32,

    /// Memory usage in megabytes
    pub memory_usage_mb: u64,

    /// Available memory in megabytes
    pub available_memory_mb: u64,

    /// CPU temperature in Celsius
    pub cpu_temperature_c: f32,

    /// UNIX timestamp of telemetry collection
    pub timestamp: u64,
}

impl Telemetry {
    /// Collects live telemetry from the local Linux system.
    pub fn collect() -> Self {
        let mut system = System::new_all();

        system.refresh_all();

        // Average CPU usage across all cores
        let cpu_usage = system.global_cpu_info().cpu_usage();

        // Memory values from sysinfo are in bytes
        let memory_usage_mb = (system.used_memory() / 1024 / 1024) as u64;

        let available_memory_mb = (system.available_memory() / 1024 / 1024) as u64;

        // Read Raspberry Pi/Linux thermal sensor
        let cpu_temperature_c = read_cpu_temperature();

        // Current UNIX timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            cpu_usage,
            memory_usage_mb,
            available_memory_mb,
            cpu_temperature_c,
            timestamp,
        }
    }
}

/// Reads CPU temperature from Linux thermal interface.
///
/// Returns temperature in Celsius.
///
/// If temperature cannot be read, returns 0.0.
fn read_cpu_temperature() -> f32 {
    const THERMAL_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

    match fs::read_to_string(THERMAL_PATH) {
        Ok(temp_str) => temp_str
            .trim()
            .parse::<f32>()
            .map(|temp| temp / 1000.0)
            .unwrap_or(0.0),
        Err(_) => 0.0,
    }
}
