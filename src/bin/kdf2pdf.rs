use kdf::helpers::logging::BinaryLogger;
use kdf::parse_kdf_document;
use log::{info, LevelFilter};
use std::path::PathBuf;
use structopt::StructOpt;

static LOGGER: BinaryLogger = BinaryLogger;

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

/// Parses a KDF file and exports it to PDF
pub fn main() {
    // Init logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    let args = Opt::from_args();
    let doc = parse_kdf_document(args.input);
    info!("{:?}", doc);
}
