use crate::cli::Cli;
use crate::error::HexxdError;
use std::io::prelude::*;

fn strip_decoded(line: &str) -> &str {
    // Last char index of hex dump
    let mut dump_end: usize = 0;

    // Keep track of previous two characters during iteration
    let mut last_chars: [char; 2] = ['\0', '\0'];

    // Find end of hex dump region in line string
    //    Hex dump region is bounded on the right
    //    by two or more spaces
    for (i, c) in line.chars().enumerate() {
        // Two spaces in a row?
        if last_chars.iter().all(|c| *c == ' ') {
            // If yes, we have gone past the end of the dump
            // Backtrack by two characters
            dump_end = i - 2;

            // Done!
            break;
        }

        // Keep track of previous characters
        last_chars[i % 2] = c;
    }

    // Return a new slice from the start to the end of the dump region
    return &line[0..dump_end];
}

fn strip_index(line: &str) -> &str {
    // Split at first space...
    if let Some((_, right)) = line.split_once(" ") {
        // Return right hand side
        return right;
    }
    // Fail silently
    return line;
}

// Check if string contains only whitespace characters or is empty
fn is_whitespace_or_empty(s: &str) -> bool {
    return s.is_empty() || s.chars().all(|c| c.is_whitespace());
}

pub fn undump_binary(_cli: Cli, mut w: Box<dyn Write>, mut r: Box<dyn Read>) -> Result<(), HexxdError> {
    // Read all input into a string
    let data_str = {
        let mut s = String::new();
        r.read_to_string(&mut s).map_err(|x| HexxdError::from(x))?;
        s
    };

    // Split data string by newlines
    let lines = data_str.split("\n");

    // Loop over data string lines
    for l in lines {
        // Skip empty/whitespace lines
        if is_whitespace_or_empty(l) {
            continue;
        }

        //------------- Isolate Hex String --------------//

        // Remove the decoded text from the end of the line
        let s = strip_decoded(l);
        // Remove the index from the start of the line
        let s = strip_index(s);

        // Remove spaces from hex dump
        let s = s.replace(" ", "");

        //------------- Hex String to Bytes -------------//

        // Create iterator for every two characters in the hex string
        // NOTE: Evey character is a hex digit from the dump
        let chars = s.chars();
        let chars: Vec<_> = chars.collect();
        let chunks = chars.chunks(2).map(String::from_iter);

        // Loop over every two characters in the hex string
        for chunk in chunks {
            // Create byte (u8) from two hex digits
            let byte = u8::from_str_radix(&chunk, 16).map_err(|x| HexxdError::from(x))?;

            // Write byte to output
            w.write(&[byte]).map_err(|x| HexxdError::from(x))?;
        }
    }
    Ok(())
}
