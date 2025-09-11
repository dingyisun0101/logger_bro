use owo_colors::OwoColorize;
use std::fmt::Arguments;
use std::io::{self, Write};
use std::sync::Mutex;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy, Debug)]
pub enum Level { Info, Warn, Crit }

impl Level {
    fn as_str(self) -> &'static str {
        match self { Level::Info => "INFO", Level::Warn => "WARN", Level::Crit => "CRIT" }
    }
}

pub struct Logger {
    obj_width: usize,
    writer: Mutex<Box<dyn Write + Send>>,
    gap_after_colon: usize,
}

impl Logger {
    pub fn new(obj_width: usize) -> Self {
        Self {
            obj_width,
            writer: Mutex::new(Box::new(io::stdout())),
            gap_after_colon: 4,
        }
    }

    pub fn log(&self, level: Level, obj: &str, args: Arguments) {
        let level_tag = level.as_str();
        let obj_padded = pad_obj(obj, self.obj_width);

        let lvl_out = level_tag.red().to_string();
        let obj_out = obj_padded.green().to_string();

        let mut guard = self.writer.lock().unwrap();
        let _ = write!(
            &mut *guard,
            "[{}] [{}]:{}{msg}\n",
            lvl_out,
            obj_out,
            " ".repeat(self.gap_after_colon),
            msg = format!("{}", args)
        );
        let _ = guard.flush();
    }

    pub fn info(&self, obj: &str, args: Arguments) { self.log(Level::Info, obj, args) }
    pub fn warn(&self, obj: &str, args: Arguments) { self.log(Level::Warn, obj, args) }
    pub fn crit(&self, obj: &str, args: Arguments) { self.log(Level::Crit, obj, args) }
}

fn pad_obj(obj: &str, width_cols: usize) -> String {
    let mut s = obj.to_string();
    while UnicodeWidthStr::width(s.as_str()) > width_cols {
        s.pop();
    }
    let w = UnicodeWidthStr::width(s.as_str());
    if w < width_cols {
        s.push_str(&" ".repeat(width_cols - w));
    }
    s
}

#[macro_export] macro_rules! info  { ($lg:expr, $obj:expr, $($arg:tt)*) => { $lg.info($obj, format_args!($($arg)*)) } }
#[macro_export] macro_rules! warn  { ($lg:expr, $obj:expr, $($arg:tt)*) => { $lg.warn($obj, format_args!($($arg)*)) } }
#[macro_export] macro_rules! crit  { ($lg:expr, $obj:expr, $($arg:tt)*) => { $lg.crit($obj, format_args!($($arg)*)) } }
