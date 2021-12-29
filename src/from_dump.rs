use lazy_static::lazy_static;
use std::io::prelude::*;

use crate::cli::Cli;
use regex::Regex;

fn strip_decoded(line: &str) -> &str {
    // TODO: Implement without regex
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s{2,}").unwrap();
    }
    let mut splitter = RE.splitn(line, 2);
    let s = splitter.next().unwrap();
    return s;
}

fn strip_index(line: &str) -> &str {
    if let Some((_, right)) = line.split_once(" ") {
        return right;
    }
    return line;
}

fn is_whitespace_or_empty(s: &str) -> bool {
    return s.is_empty() || s.chars().all(|c| c.is_whitespace());
}

pub fn undump_binary(mut w: Box<dyn Write>, _cli: Cli, mut r: Box<dyn Read>) {
    let data_str = {
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        s
    };
    let lines = data_str.split("\n");
    for l in lines {
        if is_whitespace_or_empty(l) {
            continue;
        }
        let s = strip_decoded(l);
        let s = strip_index(s);
        let s = s.replace(" ", "");
        let chars = s.chars();
        let chars: Vec<_> = chars.collect();
        let chunks = chars
            .chunks(2)
            .map(String::from_iter);
        for chunk in chunks {
            let byte = u8::from_str_radix(&chunk, 16).unwrap();
            w.write(&[byte]).unwrap();
        }
    }
}
