use kdf::parse_kdf_document;
use log::{info, Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::path::PathBuf;
use structopt::StructOpt;

/// CLI options
#[derive(StructOpt, Debug)]
#[structopt(name = "kdf2pdf")]
struct Opt {
    /// Path to KDF file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output path
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// Prompt before automatically upgrading KDF document
    #[structopt(short = "p", long)]
    upgrade_prompt: bool,
}

struct Logger;

impl log::Log for Logger {
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

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

/// Parses a KDF file and exports it to PDF
pub fn main() {
    // Init logging
    match init() {
        Ok(()) => (),
        Err(_e) => println!("Failed to init logger"),
    }

    let args = Opt::from_args();
    let doc = parse_kdf_document(args.input);
    info!("{:?}", doc);
}
