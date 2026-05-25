// simulations/local_cluster/src/health.rs

/// Represents the overall survivability state of the simulated cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterHealth {
    /// Ideal state: valid online leader within safety thermal thresholds.
    Healthy,
    
    /// Emergency state: valid leader does not exist, but surviving nodes do. 
    /// Least-bad node temporarily elected to preserve continuity.
    Degraded,
    
    /// Catastrophic state: online nodes exist but none are eligible for any fallback.
    Critical,
    
    /// Terminal state: no online nodes remain.
    Dead,
}
