// runtime/leadership/src/election.rs

use crate::leader::LeaderState;
use node_agent::node::Node;
use crate::scoring::LeadershipScore;

use std::time::{SystemTime, UNIX_EPOCH};

/// Responsible for evaluating nodes and selecting
/// the most suitable orchestration leader.
pub struct ElectionEngine;

impl ElectionEngine {
    /// Evaluates all nodes and elects the highest-scoring
    /// candidate as orchestration leader.
    ///
    /// Returns updated leadership state and
    /// calculated node scores.
    pub fn elect(nodes: &[Node]) -> (LeaderState, Vec<LeadershipScore>) {
        let mut scores: Vec<LeadershipScore> =
            nodes.iter().map(LeadershipScore::evaluate).collect();

        // Sort descending by score
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let mut leader_state = LeaderState::new();

        // Select highest scoring node
        if let Some(best_candidate) = scores.first() {
            let timestamp = current_timestamp();

            leader_state.update_leader(best_candidate.node_id.clone(), timestamp);
        }

        (leader_state, scores)
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
