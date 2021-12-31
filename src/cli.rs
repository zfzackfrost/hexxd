pub use std::path::PathBuf;
pub use structopt::StructOpt;
use std::string::ToString;

fn validate_cols(v: String) -> Result<(), String> {
    let r = v.parse::<u32>();
    if let Ok(i) = r.clone() {
        if i <= 0 ||  i >= 256 {
            Err(String::from("`-c/--cols` must be between 1 and 256"))
        } else {
            Ok(())
        }
    } else {
        let err = r.unwrap_err();
        Err(err.to_string())
    }
}

fn validate_groupsize(v: String) -> Result<(), String> {
    let r = v.parse::<u32>();
    if let Ok(i) = r.clone() {
        if i >= 256 {
            Err(String::from("`-g/--groupsize` must be between 0 and 256"))
        } else {
            Ok(())
        }
    } else {
        let err = r.unwrap_err();
        Err(err.to_string())
    }
}

/// `xxd` replacement for Neovim
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "hexxd")]
pub struct Cli {
    /// Display debug and error information
    #[structopt(long)]
    pub debug: bool,

    /// Reverse operation: convert hexdump to binary
    #[structopt(short, long)]
    pub revert: bool,

    /// Switch to bits (binary digits) dump, rather than hexdump
    #[structopt(short, long)]
    pub bits: bool,

    /// Uppercase hex letters. Default is lower case
    #[structopt(short, long)]
    pub upper: bool,

    /// Number of bytes to display per row
    #[structopt(short, long, default_value = "16", validator = validate_cols)]
    pub cols: u8,

    /// Number of bytes to group together in hexdump display
    #[structopt(short, long, default_value = "2", validator = validate_groupsize)]
    pub groupsize: u8,

    /// Input file path. If omitted will use STDIN. Use "-" to specify STDIN explicitly.
    #[structopt(name = "INPUT", default_value = "-")]
    pub ipath: String,

    /// Output file path. If omitted will use STDOUT. Use "-" to specify STDOUT explicitly.
    #[structopt(name = "OUTPUT", default_value = "-")]
    pub opath: String,
}
