# logger_bro

## Introduction
This Rust program defines a **lightweight logging utility** with custom formatting:

- **Log Levels**: Three severity levels (`INFO`, `WARN`, `CRIT`), each automatically colored (`red` for level, `green` for object tag).  
- **Custom Object Tag**: Each log message includes a user-defined `obj` string, centered to a fixed width for clean alignment.  
- **Formatted Output**: Messages are written in the style

```
[LEVEL] (obj): message
```

  with a consistent gap after the colon.  
- **Unicode Support**: Object names are centered by display width, handling wide characters gracefully.  
- **Convenient Macros**: `info!`, `warn!`, and `crit!` macros provide shorthand for logging messages without manually calling `format_args!`.  
- **Thread Safety**: The logger wraps its output writer (`stdout` by default) in a `Mutex`, making it safe for concurrent use.

This makes it easy to track structured logs in Rust projects with **aligned, color-coded, and thread-safe output**.



## Instructions
### Creating an Instance
#### For Usage in the Same File
```rust
let Logger = Logger::new(obj_txt_width)
```

#### For Usage across Files
```rust
pub static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new(obj_txt_width));
```
### Logging a Message
```rust
use logger_bro::{info, crit, warn};
use super::{your_file}::LOGGER;    // If it is defined elsewhere

info!(
    &LOGGER,
    "obj",
    "msg",
    args,
);
