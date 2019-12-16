use kdf::{document::Document, helpers::logging::BinaryLogger};
use log::{info, LevelFilter};
use std::path::PathBuf;
use structopt::StructOpt;

static LOGGER: BinaryLogger = BinaryLogger;

/// CLI options for KDF command
#[derive(StructOpt, Debug)]
#[structopt(
    name = "kdf",
    about = "Generic KDF utility.",
    author = "Sean Bailey <hello@seanbailey.io>"
)]
enum KDF {
    /// Create a new KDF document from the command line
    New {
        /// Overwite existing files
        #[structopt(short, long)]
        force: bool,

        /// Optional path for new document
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,

        /// Document title
        title: Option<String>,
    },

    /// Update a document to the latest version of KDF
    Update {
        /// Path to KDF document
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn main() {
    // Init logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    // Parse args and delegate to a command handler
    let args = KDF::from_args();
    match args {
        KDF::New { path, title, force } => new_kdf_command(path, title, force),
        KDF::Update { .. } => update_kdf_command(args),
    };
}

/// A command for creating a new KDF document
fn new_kdf_command(path: Option<PathBuf>, title: Option<String>, _force: bool) {
    let doc_path = path.unwrap_or(PathBuf::from(r"Unnamed.kdf"));
    let doc_title = title.unwrap_or(String::from("Unnamed Document"));
    info!("{:?}", doc_path);
    info!("{:?}", doc_title);
}

/// A command for updating a KDF document
fn update_kdf_command(args: KDF) {
    info!("Update doc: {:?}", args);
}
