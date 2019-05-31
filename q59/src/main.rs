use failure::Error;
use std::io;
use std::io::prelude::*;
use xpath_reader::Reader;

fn parse_sexpr(s: &str) {
    let s = s.replace("(", " ( ").replace(")", " ) ");
    let v: Vec<_> = s.split_whitespace().collect();
    for i in 0..v.len() {
        if v[i] != "NP" {
            continue;
        }
        let mut nest = 1;
        let mut tokens = Vec::new();
        for j in (i + 1)..v.len() {
            match v[j] {
                "(" => nest += 1,
                ")" => nest -= 1,
                _ if v[j - 1] != "(" => tokens.push(v[j]),
                _ => (),
            };
            if nest == 0 {
                break;
            }
        }
        println!("{}", tokens.join(" "));
    }
}

fn main() -> Result<(), Error> {
    let mut xml = String::new();
    io::stdin().read_to_string(&mut xml)?;
    let reader = Reader::from_str(&xml, None)?;

    let sentences = reader.with_nodeset_eval("/root/document/sentences/sentence")?;
    for node in &sentences.anchor_nodeset().document_order() {
        let r = Reader::from_node(*node, None);
        let parse: String = r.read("parse")?;
        parse_sexpr(&parse);
    }
    Ok(())
}
