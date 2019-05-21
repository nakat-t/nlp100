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
        let xpath = format!("//sentence[{}]/tokens/token/word", id);
        let result: Result<Vec<String>, _> = r.read(xpath.as_str());
        match result {
            Ok(ref v) if !v.is_empty() => v,
            Ok(_) => break,
            Err(_) => break,
        };
        let xpath = format!("//sentence[{}]/tokens/token[NER=\"PERSON\"]/word", id);
        let result: Result<Vec<String>, _> = r.read(xpath.as_str());
        if let Ok(person_words) = result {
            for person in person_words {
                println!("{}", person);
            }
        }
        id += 1;
    }
    Ok(())
}
