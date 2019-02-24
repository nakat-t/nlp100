use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let col1 = io::BufReader::new(File::open("col1.txt").unwrap());
    let col2 = io::BufReader::new(File::open("col2.txt").unwrap());
    let mut col1_2 = io::BufWriter::new(File::create("col1-2.txt").unwrap());

    let col1_lines = col1.lines().map(|l| l.unwrap());
    let col2_lines = col2.lines().map(|l| l.unwrap());
    for (c1, c2) in col1_lines.zip(col2_lines) {
        writeln!(col1_2, "{}\t{}", c1, c2).unwrap();
    }
}
