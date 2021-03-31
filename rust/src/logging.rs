use chrono::prelude::*;
use gdnative::*;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
struct GodotLogger;

impl Log for GodotLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        //Format it as:
        //[2017-11-09 02:12:24 DEBUG my_app] this is a debug message

        if self.enabled(record.metadata()) {
            let now = Local::now();
            let fn_name = record.file().unwrap_or("(unknown file)");
            let line = record
                .line()
                .map(|line| format!(":{}", line))
                .unwrap_or_default();
            let msg = format!(
                "[{} {} {}{}] {}",
                now.format("%F %T"),
                record.level(),
                fn_name,
                line,
                record.args()
            );

            //TODO temp disabled this, since it makes the log files bloated (it duplicates stack traces)
            /*
            match record.level() {
                Level::Error => {
                    godot_error!("{}", msg);
                }
                Level::Warn => {
                    godot_warn!("{}", msg);
                }
                _ => {
                    godot_print!("{}", msg);
                }
            }
            */

            godot_print!("{}", msg);
        }
    }

    fn flush(&self) {}
}

static GODOT_LOGGER: GodotLogger = GodotLogger;

pub fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&GODOT_LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
