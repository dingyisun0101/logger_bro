# logger_bro

## Use Example

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
