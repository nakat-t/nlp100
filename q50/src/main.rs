use failure;
use regex;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), failure::Error> {
    let re = regex::Regex::new(r"([.;:?!])\s+([A-Z])")?;
    let r = io::BufReader::new(io::stdin());
    for line in r.lines() {
        let line = line?;
        let sentences = re.replace(&line, |caps: &regex::Captures| {
            format!("{}\n{}", &caps[1], &caps[2])
        });
        println!("{}", sentences);
    }
    Ok(())
}
