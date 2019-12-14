use kdf::{helpers::logging::BinaryLogger, parse_kdf_document};
use log::{info, LevelFilter};
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

static LOGGER: BinaryLogger = BinaryLogger;

/// An enum containing possible CSS output modes.
#[derive(Debug)]
enum StyleOutputMode {
    /// Save CSS to an external stylesheet.
    External,

    /// Save CSS styles inline. Not recommended.
    Inline,

    /// Save CSS styles within the <head> of the HTML document.
    Head,
}

impl FromStr for StyleOutputMode {
    type Err = String;

    /// Create from string
    fn from_str(src: &str) -> Result<StyleOutputMode, Self::Err> {
        match src.to_lowercase().as_str() {
            "external" => Ok(StyleOutputMode::External),
            "inline" => Ok(StyleOutputMode::Inline),
            "head" => Ok(StyleOutputMode::Head),
            _ => Err(format!("Unknown mode '{}'", src)),
        }
    }
}

/// CLI options for kdf2web binary
#[derive(StructOpt, Debug)]
#[structopt(name = "kdf2web")]
struct Opt {
    /// Path to KDF file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output path
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// Set style output mode.
    #[structopt(short, long, default_value = "head")]
    style_mode: StyleOutputMode,
}

/// Parse and exports a KDF file as HTML/CSS.
pub fn main() {
    // Init logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    let args = Opt::from_args();
    info!("{:?}", args);

    let document = parse_kdf_document(args.input);
    info!("{:?}", document);
}
