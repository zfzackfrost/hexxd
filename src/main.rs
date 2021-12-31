use hexxd::cli::*;
use hexxd::to_dump::dump_binary;
use hexxd::from_dump::undump_binary;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let cli = Cli::from_args();
    
    let ifile = {
        let ipath = cli.clone().ipath;
        if ipath == "-" {
            None
        } else {
            Some(File::open(ipath).unwrap())
        }
    };
    let ofile = {
        let opath = cli.clone().opath;
        if opath == "-" {
            None
        } else {
            Some(File::create(opath).unwrap())
        }
    };


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
    

    let debug = cli.debug;
    let r = if !cli.revert {
        dump_binary(cli, writer, reader)
    } else {
        undump_binary(cli, writer, reader)
    };
    if let Err(err) = r {
        if debug {
            eprintln!("{}", err);
        }
    }
}
