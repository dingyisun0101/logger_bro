# logger_bro

## Use Example
Create an instance:
    pub static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new(obj_txt_width));

Use inside another .rs:
    use logger_bro::{info, ...};
    use super::config::{LOGGER};
    
    info!(
        &LOGGER,
        "obj",
        "msg",
        args,
    );
