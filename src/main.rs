use hexxd::cli::*;
use hexxd::from_dump::undump_binary;
use hexxd::to_dump::dump_binary;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Parse command line arguments into struct
    let cli = Cli::from_args();

    // File to read from, if any
    let ifile = {
        let ipath = cli.clone().ipath;
        if ipath == "-" {
            // If INPUT is "-", don't read from file
            None
        } else {
            // If INPUT is not "-", read from file
            Some(File::open(ipath).unwrap())
        }
    };

    // File to write to, if any
    let ofile = {
        let opath = cli.clone().opath;
        if opath == "-" {
            // If OUTPUT is "-", don't write to file
            None
        } else {
            // If OUTPUT is not "-", write to file
            Some(File::create(opath).unwrap())
        }
    };

    // Reader for input
    let reader: Box<dyn Read> = if let Some(ifile) = ifile {
        // If INPUT was a path, read from file
        Box::new(ifile)
    } else {
        // If INPUT was not provided or equals "-", read from STDIN
        Box::new(std::io::stdin())
    };

    // Writer for output
    let writer: Box<dyn Write> = if let Some(ofile) = ofile {
        // If OUTPUT was a path, write to file
        Box::new(ofile)
    } else {
        // If OUTPUT was not provided or equals "-", write to STDOUT
        Box::new(std::io::stdout())
    };

    // Cache `debug` field of Cli
    let debug = cli.debug;

    // Dump or Undump from input to output
    let r = if !cli.revert {
        // If "-r/--revert" flag is NOT given,
        // dump from input to output
        dump_binary(cli, writer, reader)
    } else {
        // If "-r/--revert" flag is given,
        // undump from input to output
        undump_binary(cli, writer, reader)
    };

    // Check for error...
    if let Err(err) = r {
        // Print error messages if `--debug` flag is given
        // Otherwise, silently ignore errors
        if debug {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
