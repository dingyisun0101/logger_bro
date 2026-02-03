/// ==============================================================================
/// examples/dummy_project_2/dummy_simulator.rs
/// Dummy simulator where progress is stored internally and read externally.
/// ==============================================================================

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;

use rand::Rng;

/// Shared progress probe exposed outside the simulator thread.
#[derive(Debug)]
pub struct ProgressProbe {
    step: AtomicUsize,
    done: AtomicBool,
}

impl ProgressProbe {
    pub fn new() -> Self {
        Self {
            step: AtomicUsize::new(0),
            done: AtomicBool::new(false),
        }
    }

    pub fn current(&self) -> usize {
        self.step.load(Ordering::SeqCst)
    }

    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::SeqCst)
    }

    fn advance(&self) {
        self.step.fetch_add(1, Ordering::SeqCst);
    }

    fn mark_done(&self) {
        self.done.store(true, Ordering::SeqCst);
    }
}

/// A single dummy simulator that only mutates internal progress.
#[derive(Debug)]
pub struct DummySimulator {
    probe: Arc<ProgressProbe>,
}

impl DummySimulator {
    /// Create a simulator that reports progress only via the shared probe.
    pub fn new(probe: Arc<ProgressProbe>) -> Self {
        Self { probe }
    }

    /// Advance the simulator by one step after waiting 1-2 seconds.
    pub fn tick(&self) {
        let delay_secs = rand::rng().random_range(1..=2);
        thread::sleep(Duration::from_secs(delay_secs));
        self.probe.advance();
    }

    /// Run the simulator for `steps` ticks and then mark completion.
    pub fn run(&self, steps: usize) {
        for _ in 0..steps {
            self.tick();
        }
        self.probe.mark_done();
    }
}

/// Create a new shared progress probe.
pub fn new_probe() -> Arc<ProgressProbe> {
    Arc::new(ProgressProbe::new())
}
