use failure;
use regex;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let re = regex::Regex::new(r"\s+")?;
    let r = io::BufReader::new(io::stdin());
    for line in r.lines() {
        let line = line?;
        for word in re.split(&line) {
            println!("{}", word);
        }
        println!("");
    }
    Ok(())
}
