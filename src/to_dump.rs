use crate::cli::Cli;
use std::io::prelude::*;

fn write_index(w: &mut Box<dyn Write>, upper: bool, idx: usize) {
    // Write the row index
    if upper {
        // Write the row index as uppercase hex number
        write!(w, "{:08X}: ", idx).unwrap();
    } else {
        // Write the row index as lowercase hex number
        write!(w, "{:08x}: ", idx).unwrap();
    }
}
fn write_text(w: &mut Box<dyn Write>, bin: Vec<u8>) {
    // Decode byte Vec to String
    let s = String::from_utf8(bin).unwrap();

    // Replace certain characters with "."
    let s = s.replace("\n", ".");
    let s = s.replace("\t", ".");
    let s = s.replace("\r", ".");
    let s = s.replace("\0", ".");

    // Write space followed by the decoded text
    write!(w, " {}", s).unwrap();
}

pub fn dump_binary(mut w: Box<dyn Write>, cli: Cli, binary: Vec<u8>) {
    let upper = cli.upper;
    // let bits = cli.bits;
    let cols = cli.cols;
    let gs = if cli.groupsize == 0 { 1 } else { cli.groupsize };
    let gspace = if cli.groupsize == 0 { "" } else { " " };

    // Create iterator for rows/columns/groups
    let rows = binary.chunks(cols.into()).map(|x| x.chunks(gs.into()));
    let last_ri = rows.clone().count() - 1;

    // The number of bytes written for the first row
    //    Used to align the decoded text of the last row
    let mut first_row_n_bytes: usize = 0;

    // The number of separator spaces written for the first row
    //    Used to align the decoded text of the last row
    let mut first_row_n_spaces: usize = 0;

    for (ri, col) in rows.enumerate() {
        // Write the row  index using (ri * cols) for the value
        write_index(&mut w, upper, ri * (cols as usize));

        let mut row_chars = Vec::new();
        let mut current_row_n_spaces: usize = 0;
        for group in col {
            for b in group {
                // Count bytes in first row
                if ri == 0 {
                    first_row_n_bytes += 1;
                }
                
                // Write the byte as a 2-digit hex number
                if upper {
                    // Write as uppercase hex
                    write!(w, "{:02X}", b).unwrap();
                } else {
                    // Write as lowercase hex
                    write!(w, "{:02x}", b).unwrap();
                }

                // Keep track of all of the bytes written for this row
                row_chars.push(*b);
            }

            // Write group spaces, if any
            write!(w, "{}", gspace).unwrap();

            // Count the number of spaces added to this row for alignment
            current_row_n_spaces += 1;

            // Count the number of spaces added to the first row for alignment
            if ri == 0 {
                first_row_n_spaces += 1;
            }
        }

        // Add spaces for alignment of decoded text
        if ri == last_ri {
            // The actual number of characters written for the current row
            let current_row_n_chars = (row_chars.len() * 2) + (current_row_n_spaces * gspace.len());

            // The actual number of characters written for the first row
            let first_row_n_chars = (first_row_n_bytes * 2) + (first_row_n_spaces * gspace.len());

            // Calculate the number of spaces needed to align text
            let extra_spaces = first_row_n_chars - current_row_n_chars;

            // Write extra spaces, if needed
            if extra_spaces > 0 {
                for _ in 0..extra_spaces {
                    write!(w, " ").unwrap();
                }
            }
        }
        // Write decoded text at end of row
        write_text(&mut w, row_chars);

        // Write newline at end of row
        writeln!(w).unwrap();
    }
}
