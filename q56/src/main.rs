use failure::Error;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use xpath_reader::Reader;

#[derive(Debug)]
struct Coreference {
    sentence: u32,
    start: u32,
    end: u32,
    text: String,
    rep_text: String,
}

fn main() -> Result<(), Error> {
    let mut xml = String::new();
    io::stdin().read_to_string(&mut xml)?;
    let reader = Reader::from_str(&xml, None)?;

    let mut corefs = HashMap::new();
    let r = reader.with_nodeset_eval("/root/document/coreference/coreference")?;
    let coref_nodeset = r.anchor_nodeset().document_order();
    for node in coref_nodeset.iter() {
        let r = Reader::from_node(*node, None);
        let rep_text: String = r.read("mention[@representative=\"true\"]/text")?;
        let r = r.with_nodeset_eval("mention")?;
        let mention_nodeset = r.anchor_nodeset().document_order();
        for node in mention_nodeset.iter() {
            let r = Reader::from_node(*node, None);
            let rep: Option<bool> = r.read("@representative")?;
            if rep.is_some() {
                continue;
            }
            let sentence: u32 = r.read("sentence")?;
            let start: u32 = r.read("start")?;
            let end: u32 = r.read("end")?;
            let text: String = r.read("text")?;
            corefs.entry((sentence, start)).or_insert(Coreference {
                sentence,
                start,
                end,
                text,
                rep_text: rep_text.clone(),
            });
        }
    }

    let r = reader.with_nodeset_eval("/root/document/sentences/sentence")?;
    let sentence_nodeset = r.anchor_nodeset().document_order();
    for (i, node) in sentence_nodeset.iter().enumerate() {
        let sentence_id = (i + 1) as u32;
        let r = Reader::from_node(*node, None);
        let words: Vec<String> = r.read("tokens/token/word")?;
        let mut skip = None;
        for (i, word) in words.iter().enumerate() {
            let token_id = (i + 1) as u32;
            match skip {
                Some(skip) if skip < token_id => continue,
                _ => skip = None,
            };
            if let Some(coref) = corefs.get(&(sentence_id, token_id)) {
                print!("<{}> ({}) ", coref.rep_text, coref.text);
                skip = Some(coref.end);
            } else {
                print!("{} ", word);
            }
        }
        println!();
    }
    Ok(())
}
