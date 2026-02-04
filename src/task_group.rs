/// ==============================================================================
/// src/task_group.rs
/// Task group runner for parallel execution and reporting.
/// ==============================================================================

use std::thread;

use crate::{ClientStore, Task};
#[cfg(feature = "tui")]
use crate::Runtime;

/// A group of tasks executed in parallel.
///
/// Implement this on a custom class that manages tasks in a unique way.
/// The caller only needs to return the vector of tasks; the runtime
/// handles reporting and the TUI when `launch()` is called.
pub trait TaskGroup: Send + 'static {
    /// The task type executed by this group.
    type Task: Task;

    /// Return the tasks managed by this group.
    fn tasks(self) -> Vec<Self::Task>;

    /// Launch all tasks until their total iteration limit is hit.
    fn launch(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        Self: Sized,
    {
        launch_tasks(self.tasks(), 20, None)
    }
}

/// Helper that launches a vector of tasks with standard TUI behavior.
pub fn launch_tasks<T: Task>(
    tasks: Vec<T>,
    runtime_fps: u64,
    runtime_project_label: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let runtime_quit_on_q = true;
    let (reporter, mut store) = ClientStore::new();
    let mut handles = Vec::with_capacity(tasks.len());

    for mut task in tasks {
        let reporter = reporter.clone();
        let label = task.label().to_string();
        let total_iters = task.total_iters();
        let total = Some(total_iters);

        let handle = thread::spawn(move || {
            let client = reporter.start(label, total);
            if let Ok(client) = client {
                for step in 0..total_iters {
                    task.workload_per_iter();
                    let _ = client.set_current(step.saturating_add(1));
                }
                let _ = client.set_current(total_iters);
                let _ = client.complete();
            }
        });

        handles.push(handle);
    }

    #[cfg(feature = "tui")]
    {
        let mut runtime = Runtime::new(runtime_fps);
        runtime = runtime.quit_on_q(runtime_quit_on_q);
        if let Some(label) = runtime_project_label {
            runtime = runtime.project_label(label);
        }
        runtime.run(&mut store)?;
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}
