use hexxd::cli::*;
use hexxd::to_dump::dump_binary;
use hexxd::from_dump::undump_binary;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let cli = Cli::from_args();

    let ifile = cli.clone().ipath.map(|p| File::open(p).unwrap());
    let ofile = cli.clone().opath.map(|p| File::create(p).unwrap());

    let reader: Box<dyn Read> = if let Some(ifile) = ifile {
        Box::new(ifile)
    } else {
        Box::new(std::io::stdin())
    };

    let writer: Box<dyn Write> = if let Some(ofile) = ofile {
        Box::new(ofile)
    } else {
        Box::new(std::io::stdout())
    };
    
    if !cli.revert {
        dump_binary(cli, writer, reader);
    } else {
        undump_binary(cli, writer, reader);
    }
}
