use plotlib::page::Page;
use plotlib::scatter::Scatter;
use plotlib::view::ContinuousView;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Morpheme {
    surface: String,
    pos: String,
    pos1: String,
    pos2: String,
    pos3: String,
    conj_rule: String,
    conj_type: String,
    base: String,
    yomi: String,
    pron: String,
}

impl Morpheme {
    fn from(surface: &str) -> Morpheme {
        Morpheme {
            surface: String::from(surface),
            pos: String::from("*"),
            pos1: String::from("*"),
            pos2: String::from("*"),
            pos3: String::from("*"),
            conj_rule: String::from("*"),
            conj_type: String::from("*"),
            base: String::from("*"),
            yomi: String::from("*"),
            pron: String::from("*"),
        }
    }

    fn from_text(text: &str) -> Option<Morpheme> {
        let v: Vec<_> = text.split("\t").collect();
        if v.len() < 2 {
            return None;
        }
        let mut m = Morpheme::from(&v[0]);
        let v: Vec<_> = v[1].split(",").collect();
        if v.len() < 9 {
            return None;
        }
        m.pos = v[0].to_string();
        m.pos1 = v[1].to_string();
        m.pos2 = v[2].to_string();
        m.pos3 = v[3].to_string();
        m.conj_rule = v[4].to_string();
        m.conj_type = v[5].to_string();
        m.base = v[6].to_string();
        m.yomi = v[7].to_string();
        m.pron = v[8].to_string();

        Some(m)
    }
}

impl fmt::Display for Morpheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Morpheme({}-{},{},{},{},{},{},{},{},{})",
            self.surface,
            self.pos,
            self.pos1,
            self.pos2,
            self.pos3,
            self.conj_rule,
            self.conj_type,
            self.base,
            self.yomi,
            self.pron
        )
    }
}

fn main() -> io::Result<()> {
    let f = BufReader::new(io::stdin());

    let mut neko: Vec<Vec<Morpheme>> = Vec::new();
    let mut sentence: Vec<Morpheme> = Vec::new();
    for line in f.lines() {
        let line = line?;
        if line.starts_with("EOS") {
            if sentence.len() > 0 {
                neko.push(sentence);
                sentence = Vec::new();
            }
        } else {
            if let Some(m) = Morpheme::from_text(&line) {
                sentence.push(m);
            }
        }
    }
    if sentence.len() > 0 {
        neko.push(sentence);
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for m in neko.iter().flat_map(|x| x.iter()) {
        *map.entry(m.surface.clone()).or_insert(0) += 1;
    }
    let mut v: Vec<f64> = map.values().map(|x| *x as f64).collect();
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let data: Vec<(f64, f64)> = v
        .iter()
        .enumerate()
        .map(|(i, x)| ((1.0 + (i as f64)).log10(), x.log10()))
        .collect();

    let s = Scatter::from_slice(&data);
    let view = ContinuousView::new().add(&s);

    Page::single(&view)
        .save("q39.svg")
        .expect("Image output failed.");

    Ok(())
}
