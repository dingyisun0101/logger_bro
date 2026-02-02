# logger_bro

A small Rust library for tracking multiple task-like "clients" and rendering a simple TUI dashboard with aligned progress, status coloring, and timing info.

**Features**
- Client/reporting API designed for multi-threaded producers
- In-memory store that merges partial updates safely
- Optional TUI runtime (enabled by default) with live progress, last-update timing, and project header

**Install**
```toml
[dependencies]
logger_bro = "0.3"
```

Disable the TUI feature if you only want the core reporting types:
```toml
[dependencies]
logger_bro = { version = "0.3", default-features = false }
```

**Core Usage**
```rust
use logger_bro::prelude::client::*;

let (reporter, mut store) = ClientStore::new();

let handle = reporter.start("worker-1", Some(100))?;
handle.set_current(1)?;
handle.set_current(2)?;
handle.complete()?;

store.drain();
let snapshot = store.snapshot();
println!("tracked: {}", snapshot.len());
# Ok::<(), ReportError>(())
```

**TUI Runtime**
```rust
use logger_bro::prelude::client::*;
use logger_bro::prelude::runtime::*;

let (reporter, mut store) = ClientStore::new();

std::thread::spawn(move || {
    let handle = reporter.start("sim-1", Some(20)).unwrap();
    for i in 1..=20 {
        handle.set_current(i).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    handle.complete().unwrap();
});

let mut runtime = Runtime::new(20).project_label("Dummy Project");
runtime.run(&mut store)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Controls:
- `q` begins quit confirmation
- `y` confirms quit
- `n` or `Esc` cancels quit

**Examples**
```bash
cargo run --example dummy_project
```

**Notes**
- Status colors: Completed is green, Canceled/Failed are red
- Each client row shows time since last update
- The project header shows elapsed time since the runtime started
