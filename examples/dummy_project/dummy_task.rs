/// ==============================================================================
/// examples/dummy_project_2/dummy_task.rs
/// Dummy task where progress is stored internally and read externally.
/// ==============================================================================

use std::thread;
use std::time::Duration;

use rand::Rng;

use logger_bro::Task;

/// A single dummy task that only mutates internal progress.
#[derive(Debug)]
pub struct DummyTask {
    step: u64,
    label: Option<String>,
    total: Option<u64>,
}

impl DummyTask {
    /// Create a task that reports progress via its internal state.
    pub fn new() -> Self {
        Self {
            step: 0,
            label: None,
            total: None,
        }
    }

    /// Advance the task by one step after waiting 1-2 seconds.
    pub fn step_once(&mut self) {
        let delay_secs = rand::rng().random_range(1..=2);
        thread::sleep(Duration::from_secs(delay_secs));
        self.step = self.step.saturating_add(1);
    }
}

impl Task for DummyTask {
    fn tick(&mut self) {
        self.step_once();
    }

    fn set_metadata(&mut self, label: String, total: Option<u64>) {
        self.label = Some(label);
        self.total = total;
    }

    fn current(&self) -> Option<u64> {
        Some(self.step)
    }
}
