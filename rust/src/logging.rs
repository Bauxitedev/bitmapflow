use chrono::prelude::*;
use gdnative::prelude::*;
use log::*;

struct GodotLogger;

static GODOT_LOGGER: GodotLogger = GodotLogger;

pub fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&GODOT_LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

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

//////////////////////////////////////////////////////////

type Base = Node;
//Base refers to the type Logger inherits from. In this case it's Node (because #[inherit(Node)])

#[derive(NativeClass)]
#[inherit(Base)]
pub struct Logger {}

impl Logger {
    fn new(_owner: &Base) -> Self {
        Logger {}
    }
}

#[methods]
impl Logger {
    //Allows logging from GDscript

    fn log(&self, level: Level, msg: String) {
        GODOT_LOGGER.log(
            &Record::builder()
                .level(level)
                .file(Some("(gdscript)"))
                .args(format_args!("{}", msg))
                .build(),
        );
    }
    #[export]
    fn info(&mut self, _owner: &Base, msg: String) {
        self.log(Level::Info, msg);
    }
    #[export]
    fn warn(&mut self, _owner: &Base, msg: String) {
        self.log(Level::Warn, msg);
    }
    #[export]
    fn error(&mut self, _owner: &Base, msg: String) {
        self.log(Level::Error, msg);
    }
}
