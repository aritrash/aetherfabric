// simulations/local_cluster/src/clock.rs

/// Represents a deterministic logical clock for simulation.
#[derive(Debug, Clone)]
pub struct SimulationClock {
    pub tick: u64,
    pub current_time_secs: u64,
}

impl SimulationClock {
    pub fn new() -> Self {
        Self {
            tick: 0,
            current_time_secs: 0,
        }
    }

    pub fn advance(
        &mut self,
        seconds: u64,
    ) {
        self.tick += 1;
        self.current_time_secs += seconds;
    }
}
