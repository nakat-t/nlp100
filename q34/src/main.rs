use std::collections::HashSet;
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

    let mut result: HashSet<String> = HashSet::new();
    for i in 0..neko.len() {
        for j in 2..neko[i].len() {
            let (m0, m1, m2) = (&neko[i][j - 2], &neko[i][j - 1], &neko[i][j]);
            if m0.pos == "名詞" && m1.surface == "の" && m2.pos == "名詞" {
                result.insert(format!("{}の{}", m0.surface, m2.surface));
            }
        }
    }

    for r in &result {
        println!("{}", r);
    }

    Ok(())
}
