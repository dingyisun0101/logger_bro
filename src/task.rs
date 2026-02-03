/// ==============================================================================
/// src/task.rs
/// Minimal task trait for integrating logger_bro with custom simulators.
/// ==============================================================================

/// A single unit of work that can be stepped forward.
///
/// This trait intentionally avoids `new()` so tasks can be constructed
/// with arbitrary domain-specific inputs. Use `set_metadata` to configure
/// logging-related metadata after construction.
pub trait Task: Send + 'static {
    /// Advance the task by one tick/step.
    fn tick(&mut self);

    /// Provide a human-readable label and total units of work, or `None` for unknown.
    fn set_metadata(&mut self, label: String, total: Option<u64>) {
        let _ = label;
        let _ = total;
    }

    /// Optionally expose current progress.
    ///
    /// Return `None` when progress is observed externally (e.g. a probe).
    fn current(&self) -> Option<u64> {
        None
    }
}
