# logger_bro

A small Rust library for running scientific workloads with a built-in TUI dashboard.

**Install**
```toml
[dependencies]
logger_bro = "0.7.0"
```

**Usage**
```rust
use logger_bro::{Task, TaskGroup};

struct SimTask {
    label: String,
    total_iters: u64,
}

impl Task for SimTask {
    fn label(&self) -> &str {
        &self.label
    }

    fn total_iters(&self) -> u64 {
        self.total_iters
    }

    fn workload_per_iter(&mut self) {
        // real scientific work here
    }
}

struct SimGroup {
    tasks: Vec<SimTask>,
}

impl TaskGroup for SimGroup {
    type Task = SimTask;

    fn tasks(self) -> Vec<Self::Task> {
        self.tasks
    }
}

SimGroup { tasks: vec![] }.launch()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Controls:
- `q` begins quit confirmation
- `y` confirms quit
- `n` or `Esc` cancels quit
After confirmation, the process exits.
