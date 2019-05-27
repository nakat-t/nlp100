use failure::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use xpath_reader::Reader;

fn write_dot(path: &str, deps: &[(String, String)]) -> Result<(), Error> {
    let path = Path::new(path);
    let mut w = io::BufWriter::new(File::create(&path)?);
    writeln!(w, "digraph \"{}\" {{", path.display())?;
    writeln!(w, "\tnode [ fontname = \"Meiryo\" ];")?;
    for dep in deps {
        writeln!(w, "\t\"{}\" -> \"{}\"", dep.0, dep.1)?;
    }
    writeln!(w, "}}")?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut xml = String::new();
    io::stdin().read_to_string(&mut xml)?;
    let reader = Reader::from_str(&xml, None)?;

    let r = reader.with_nodeset_eval("/root/document/sentences/sentence")?;
    let sentence_nodeset = r.anchor_nodeset().document_order();
    for (i, node) in sentence_nodeset.iter().enumerate() {
        let mut deps = Vec::new();
        let r = Reader::from_node(*node, None);
        let r = r.with_nodeset_eval("dependencies[@type=\"collapsed-dependencies\"]/dep")?;
        let deps_nodeset = r.anchor_nodeset().document_order();
        for node in &deps_nodeset {
            let r = Reader::from_node(*node, None);
            let governor: String = r.read("governor")?;
            let governor_idx: u32 = r.read("governor/@idx")?;
            let governor_str = format!("{}:{}", governor_idx, governor);
            let dependent: String = r.read("dependent")?;
            let dependent_idx: u32 = r.read("dependent/@idx")?;
            let dependent_str = format!("{}:{}", dependent_idx, dependent);
            deps.push((governor_str, dependent_str));
        }
        let filename = format!("out/sentence{}.dot", i + 1);
        write_dot(&filename, &deps)?;
    }
    Ok(())
}
