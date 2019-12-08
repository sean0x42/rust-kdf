use log::{Level, Metadata, Record};

/// A logger designed for use with the various KDF binaries.
pub struct BinaryLogger;

impl log::Log for BinaryLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
