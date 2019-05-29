use failure::Error;
use std::io;
use std::io::prelude::*;
use xpath_reader::Reader;

fn main() -> Result<(), Error> {
    let mut xml = String::new();
    io::stdin().read_to_string(&mut xml)?;
    let reader = Reader::from_str(&xml, None)?;

    let sentences = reader.with_nodeset_eval("/root/document/sentences/sentence")?;
    for node in &sentences.anchor_nodeset().document_order() {
        let r = Reader::from_node(*node, None);
        let deps = r.with_nodeset_eval("dependencies[@type=\"collapsed-dependencies\"]/dep")?;
        for node in &deps.anchor_nodeset().document_order() {
            let r = Reader::from_node(*node, None);
            let dep1_type: String = r.read("@type")?;
            if dep1_type == "nsubj" {
                let dep1_governor_idx: u32 = r.read("governor/@idx")?;
                let dep1_dependent: String = r.read("dependent")?;
                for node2 in &deps.anchor_nodeset().document_order() {
                    let r = Reader::from_node(*node2, None);
                    let dep2_type: String = r.read("@type")?;
                    if dep2_type == "dobj" {
                        let dep2_governor_idx: u32 = r.read("governor/@idx")?;
                        if dep1_governor_idx == dep2_governor_idx {
                            let pred: String = r.read("governor")?;
                            let nsubj = &dep1_dependent;
                            let dobj: String = r.read("dependent")?;
                            println!("{}\t{}\t{}", nsubj, pred, dobj);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
