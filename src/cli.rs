pub use clap::Parser;

fn parse_cols(v: &str) -> Result<u8, String> {
    let r = v.parse::<i64>();
    if let Ok(i) = r.clone() {
        if !(1..256).contains(&i) {
            Err(String::from(
                "`-c/--cols` must be between 1 and 255, inclusive",
            ))
        } else {
            Ok(i as u8)
        }
    } else {
        let err = r.unwrap_err();
        Err(err.to_string())
    }
}

fn parse_groupsize(v: &str) -> Result<u8, String> {
    let r = v.parse::<i64>();
    if let Ok(i) = r.clone() {
        if !(0..256).contains(&i) {
            Err(String::from(
                "`-g/--groupsize` must be between 0 and 255, inclusive",
            ))
        } else {
            Ok(i as u8)
        }
    } else {
        let err = r.unwrap_err();
        Err(err.to_string())
    }
}

/// `xxd` replacement for Neovim
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Display debug and error information
    #[clap(long)]
    pub debug: bool,

    /// Reverse operation: convert hexdump to binary
    #[clap(short, long)]
    pub revert: bool,

    /// Switch to bits (binary digits) dump, rather than hexdump
    #[clap(short, long)]
    pub bits: bool,

    /// Uppercase hex letters. Default is lower case
    #[clap(short, long)]
    pub upper: bool,

    /// Number of bytes to display per row
    #[clap(short, long, default_value = "16", value_parser = parse_cols)]
    pub cols: u8,

    /// Number of bytes to group together in hexdump display
    #[clap(short, long, default_value = "2", value_parser = parse_groupsize)]
    pub groupsize: u8,

    /// Input file path. If omitted will use STDIN. Use "-" to specify STDIN explicitly.
    #[clap(name = "INPUT", default_value = "-")]
    pub ipath: String,

    /// Output file path. If omitted will use STDOUT. Use "-" to specify STDOUT explicitly.
    #[clap(name = "OUTPUT", default_value = "-")]
    pub opath: String,
}
