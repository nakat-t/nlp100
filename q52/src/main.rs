use failure;
use rust_stemmers;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);
    let r = io::BufReader::new(io::stdin());
    for line in r.lines() {
        let line = line?;
        println!("{}\t{}", line, stemmer.stem(&line));
    }
    Ok(())
}
