use failure;
use std::cmp;
use std::io;
use std::io::prelude::*;
use xpath_reader;

fn main() -> Result<(), failure::Error> {
    let mut xml = String::new();
    let _ = io::stdin().read_to_string(&mut xml)?;
    let r = xpath_reader::Reader::from_str(&xml, None)?;
    let mut id = 1;
    loop {
        let xpath_words = format!("//sentence[{}]/*/*/word", id);
        let result_words: Result<Vec<String>, _> = r.read(xpath_words.as_str());
        let words = match result_words {
            Ok(ref words) if !words.is_empty() => words,
            Ok(_) => break,
            Err(_) => break,
        };
        let xpath_lemmas = format!("//sentence[{}]/*/*/lemma", id);
        let result_lemmas: Result<Vec<String>, _> = r.read(xpath_lemmas.as_str());
        let lemmas = match result_lemmas {
            Ok(ref lemmas) if !lemmas.is_empty() => lemmas,
            Ok(_) => break,
            Err(_) => break,
        };
        let xpath_pos = format!("//sentence[{}]/*/*/POS", id);
        let result_pos: Result<Vec<String>, _> = r.read(xpath_pos.as_str());
        let pos = match result_pos {
            Ok(ref pos) if !pos.is_empty() => pos,
            Ok(_) => break,
            Err(_) => break,
        };
        let n_tokens = cmp::max(words.len(), cmp::max(lemmas.len(), pos.len()));
        for i in 0..n_tokens {
            let s = String::new();
            let word = words.get(i).unwrap_or(&s);
            let lemma = lemmas.get(i).unwrap_or(&s);
            let pos = pos.get(i).unwrap_or(&s);
            println!("{}\t{}\t{}", word, lemma, pos);
        }
        println!();
        id += 1;
    }
    Ok(())
}
