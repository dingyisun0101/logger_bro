/// ==============================================================================
/// examples/dummy_project/dummy_simulator.rs
/// Minimal dummy simulator for testing client-side reporting.
/// ==============================================================================

use std::thread;
use std::time::Duration;

use rand::Rng;

/// A single dummy simulator with a monotonically increasing step counter.
#[derive(Debug)]
pub struct DummySimulator {
    /// Current simulation step.
    pub step: usize,
}

impl DummySimulator {
    /// Create a new simulator starting at step 0.
    pub fn new() -> Self {
        Self { step: 0 }
    }

    /// Advance the simulator by one step after waiting 1-2 seconds.
    pub fn tick(&mut self) {
        let delay_secs = rand::thread_rng().gen_range(1..=2);
        thread::sleep(Duration::from_secs(delay_secs));
        self.step = self.step.saturating_add(1);
    }
}
