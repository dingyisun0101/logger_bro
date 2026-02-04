/// ==============================================================================
/// src/task.rs
/// Task trait for workload-driven scientific objects.
/// ==============================================================================

/// A single unit of work executed by the runner.
///
/// Implement this on your scientific object. The launcher will call
/// `workload_per_iter` for each tick and report progress using
/// `total_iters`.
pub trait Task: Send + 'static {
    /// Human-readable label for the task.
    fn label(&self) -> &str;

    /// Total iterations for this task.
    fn total_iters(&self) -> u64;

    /// Perform one unit of work.
    fn workload_per_iter(&mut self);
}
