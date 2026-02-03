/// ==============================================================================
/// src/task_group.rs
/// Trait for task groups that orchestrate parallel execution.
/// ==============================================================================

use std::thread;

use crate::{ClientStore, Task};
#[cfg(feature = "tui")]
use crate::Runtime;

/// A group/cluster that owns the parallel mission runner.
///
/// Implementors decide how tasks are constructed and configured, and
/// can spawn threads or async tasks as needed.
pub trait TaskGroup {
    /// The task type executed in each worker thread.
    type Task: Task;

    /// How many tasks should be launched.
    fn task_count(&self) -> usize;

    /// Build the task for the given index.
    fn build_task(&self, index: usize) -> Self::Task;

    /// Configure task metadata (label/total/etc.) after construction.
    fn configure_task(&self, _index: usize, _task: &mut Self::Task) {}

    /// Optional label for logging when the runner doesn't read it from the task.
    fn label_for(&self, _index: usize) -> Option<String> {
        None
    }

    /// Optional total units for logging when the runner doesn't read it from the task.
    fn total_for(&self, _index: usize) -> Option<u64> {
        None
    }

    /// Steps for a given task index (defaults to the group-wide `steps` passed to `run`).
    fn steps_for(&self, default_steps: usize, _index: usize) -> usize {
        default_steps
    }

    /// Target FPS for the TUI runtime.
    fn runtime_fps(&self) -> u64 {
        20
    }

    /// Whether pressing `q` quits the runtime.
    fn runtime_quit_on_q(&self) -> bool {
        true
    }

    /// Optional project label for the runtime header.
    fn runtime_project_label(&self) -> Option<String> {
        None
    }

    /// Default runner that handles logging and (optionally) the TUI.
    ///
    /// Implementors only provide task construction/configuration; the
    /// execution and reporting is handled here.
    fn run(&self, steps: usize) -> Result<(), Box<dyn std::error::Error>> {
        let (reporter, mut store) = ClientStore::new();
        let task_count = self.task_count();
        let mut handles = Vec::with_capacity(task_count);

        for i in 0..task_count {
            let reporter = reporter.clone();
            let label = self
                .label_for(i)
                .unwrap_or_else(|| format!("task-{i}"));
            let task_steps = self.steps_for(steps, i);
            let total = self
                .total_for(i)
                .or(Some(task_steps as u64));

            let mut task = self.build_task(i);
            self.configure_task(i, &mut task);
            task.set_metadata(label.clone(), total);

            let handle = thread::spawn(move || {
                let client = reporter.start(label, total);
                if let Ok(client) = client {
                    for _ in 0..task_steps {
                        task.tick();
                        if let Some(current) = task.current() {
                            let _ = client.set_current(current);
                        }
                    }
                    let _ = client.set_current(task_steps as u64);
                    let _ = client.complete();
                }
            });

            handles.push(handle);
        }

        #[cfg(feature = "tui")]
        {
            let mut runtime = Runtime::new(self.runtime_fps());
            runtime = runtime.quit_on_q(self.runtime_quit_on_q());
            if let Some(label) = self.runtime_project_label() {
                runtime = runtime.project_label(label);
            }
            runtime.run(&mut store)?;
        }

        for handle in handles {
            let _ = handle.join();
        }

        Ok(())
    }
}
