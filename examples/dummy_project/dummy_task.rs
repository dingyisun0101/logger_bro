/// ==============================================================================
/// examples/dummy_project/dummy_task.rs
/// Dummy task spec wrapper.
/// ==============================================================================

use std::thread;
use std::time::Duration;

use rand::Rng;

/// A single dummy task that defines workload and total iterations.
#[derive(Debug)]
pub struct DummyTask {
    pub label: String,
    pub total_iters: u64,
}

impl DummyTask {
    /// Create a dummy task with label and total iterations.
    pub fn new<L>(label: L, total_iters: u64) -> Self
    where
        L: Into<String>,
    {
        Self {
            label: label.into(),
            total_iters,
        }
    }

    /// Do one unit of work.
    pub fn step(&mut self) {
        let delay_secs = rand::rng().random_range(1..=2);
        thread::sleep(Duration::from_secs(delay_secs));
    }
}

impl logger_bro::Task for DummyTask {
    fn label(&self) -> &str {
        &self.label
    }

    fn total_iters(&self) -> u64 {
        self.total_iters
    }

    fn workload_per_iter(&mut self) {
        self.step();
    }
}
