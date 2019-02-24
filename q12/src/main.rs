use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let col1_path = Path::new("col1.txt");
    let col2_path = Path::new("col2.txt");

    let mut col1 = io::BufWriter::new(match File::create(&col1_path) {
        Err(why) => panic!(
            "couldn't create {}: {}",
            col1_path.display(),
            Error::description(&why)
        ),
        Ok(file) => file,
    });
    let mut col2 = io::BufWriter::new(match File::create(&col2_path) {
        Err(why) => panic!(
            "couldn't create {}: {}",
            col2_path.display(),
            Error::description(&why)
        ),
        Ok(file) => file,
    });

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let cols: Vec<String> = line.split_whitespace().map(|c| c.to_string()).collect();
                writeln!(col1, "{}", cols[0])
                    .expect(&format!("write failed to {}", col1_path.display()));
                writeln!(col2, "{}", cols[1])
                    .expect(&format!("write failed to {}", col2_path.display()));
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
}
