use kdf::parse_kdf_document;
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

/// Parses a KDF file and exports it to PDF
pub fn main() {
    let args = Opt::from_args();
    parse_kdf_document(args.input)
}
