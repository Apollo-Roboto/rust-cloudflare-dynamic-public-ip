use colored::Colorize;
use log::{Level, Log, Metadata, Record};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub struct SimpleLogger;

pub static LOGGER: SimpleLogger = SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with(&CRATE_NAME.replace("-", "_"))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level_text = match record.level() {
            Level::Error => "ERROR".bright_red(),
            Level::Warn => "WARNING".yellow(),
            Level::Info => "INFO".green(),
            Level::Debug => "DEBUG".blue(),
            Level::Trace => "TRACE".cyan(),
        };

        let now = chrono::Local::now();

        for (i, line) in record.args().to_string().lines().enumerate() {
            if i == 0 {
                println!(
                    "{:<36} {:<8} {}",
                    now.to_rfc3339().black(),
                    level_text,
                    line
                );
            } else {
                println!("{:<36} {:<8} {}", "", "", line);
            }
        }
    }

    fn flush(&self) {}
}
