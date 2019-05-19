use failure;
use std::io;
use std::io::prelude::*;
use xpath_reader;

fn main() -> Result<(), failure::Error> {
    let mut xml = String::new();
    let _ = io::stdin().read_to_string(&mut xml)?;
    let r = xpath_reader::Reader::from_str(&xml, None)?;
    let mut id = 1;
    loop {
        let xpath = format!("//sentence[@id={}]/*/*/word", id);
        let result: Result<Vec<String>, _> = r.read(xpath.as_str());
        let words = match result {
            Ok(ref words) if words.len() > 0 => words,
            Ok(_) => break,
            Err(_) => break,
        };
        for word in words {
            println!("{}", word);
        }
        println!("");
        id += 1;
    }
    Ok(())
}
