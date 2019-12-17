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
        KDF::New { path, title, force } => handle_new(path, title, force),
        KDF::Update { .. } => handle_update(args),
    };
}

/// A command for creating a new KDF document
fn handle_new(path: Option<PathBuf>, title: Option<String>, _force: bool) {
    let doc_path = path.unwrap_or(PathBuf::from(r"Untitled.kdf"));
    let doc = Document::new(title.unwrap_or(String::from("Untitled Document"));
    // TODO: Convert document struct into .tar.gz archive and attempt to save to fs.
    info!("{:?}", doc_path);
    info!("{:?}", doc);
}

/// A command for updating a KDF document
fn handle_update(args: KDF) {
    info!("Update doc: {:?}", args);
}
