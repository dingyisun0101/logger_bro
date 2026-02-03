/// ==============================================================================
/// examples/dummy_project_2/dummy_task_group.rs
/// Dummy task group where progress is observed externally.
/// ==============================================================================

use logger_bro::TaskGroup;

use super::dummy_task::DummyTask;

/// A task group that runs multiple dummy tasks in parallel threads.
#[derive(Debug)]
pub struct DummyTaskGroup {
    /// Number of tasks to spawn.
    pub count: usize,
}

impl DummyTaskGroup {
    /// Create a task group with `count` tasks.
    pub fn new(count: usize) -> Self {
        Self { count }
    }

}

impl TaskGroup for DummyTaskGroup {
    type Task = DummyTask;

    fn task_count(&self) -> usize {
        self.count
    }

    fn build_task(&self, _index: usize) -> Self::Task {
        DummyTask::new()
    }

    fn label_for(&self, index: usize) -> Option<String> {
        Some(format!("task-{index}"))
    }
}
