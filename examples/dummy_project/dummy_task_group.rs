/// ==============================================================================
/// examples/dummy_project/dummy_task_group.rs
/// Dummy task group that owns a vector of dummy tasks.
/// ==============================================================================

use super::dummy_task::DummyTask;

/// A task group that launches multiple dummy tasks in parallel threads.
#[derive(Debug)]
pub struct DummyTaskGroup {
    tasks: Vec<DummyTask>,
}

impl DummyTaskGroup {
    /// Create a task group with `count` tasks and a fixed iteration total.
    pub fn new(count: usize, total_iters: u64) -> Self {
        let tasks = (0..count)
            .map(|i| DummyTask::new(format!("task-{i}"), total_iters))
            .collect();
        Self { tasks }
    }
}

impl logger_bro::TaskGroup for DummyTaskGroup {
    type Task = DummyTask;

    fn tasks(self) -> Vec<Self::Task> {
        self.tasks
    }
}
