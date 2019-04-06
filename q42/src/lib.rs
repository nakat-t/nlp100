use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Morpheme {
    pub surface: String,
    pub pos: String,
    pub pos1: String,
    pub pos2: String,
    pub pos3: String,
    pub conj_rule: String,
    pub conj_type: String,
    pub base: String,
    pub yomi: String,
    pub pron: String,
}

impl Morpheme {
    pub fn from_text(text: &str) -> Option<Morpheme> {
        let v: Vec<_> = text.split("\t").collect();
        if v.len() < 2 {
            return None;
        }
        let surface = v[0];
        let mut v: Vec<_> = v[1].split(",").collect();
        if v.len() < 9 {
            for _ in v.len()..9 {
                v.push("");
            }
        }
        Some(Morpheme {
            surface: surface.to_string(),
            pos: v[0].to_string(),
            pos1: v[1].to_string(),
            pos2: v[2].to_string(),
            pos3: v[3].to_string(),
            conj_rule: v[4].to_string(),
            conj_type: v[5].to_string(),
            base: v[6].to_string(),
            yomi: v[7].to_string(),
            pron: v[8].to_string(),
        })
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

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: usize,
    pub morphs: Vec<Morpheme>,
    pub dst: Option<usize>,
    pub srcs: Vec<usize>,
    pub score: f64,
    pub head: usize,
    pub func: usize,
}

impl Chunk {
    pub fn from_lattice_text(lines: Vec<String>) -> Option<Chunk> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\A\* (\d+) (-?\d+)D (\d+)/(\d+) (-?\d+\.\d+)").unwrap();
        }
        if lines.len() < 2 {
            return None;
        }
        let chunk_line = &lines[0];
        let morph_lines = &lines[1..];
        let cap = match RE.captures_iter(&chunk_line).next() {
            Some(cap) => cap,
            None => {
                return None;
            }
        };
        let id: usize = match &cap[1].parse::<usize>() {
            Ok(n) => *n,
            Err(_) => {
                return None;
            }
        };
        let dst: Option<usize> = match &cap[2].parse::<i32>() {
            Ok(n) if *n < 0 => None,
            Ok(n) => Some(*n as usize),
            Err(_) => {
                return None;
            }
        };
        let head: usize = match &cap[3].parse::<usize>() {
            Ok(n) => *n,
            Err(_) => {
                return None;
            }
        };
        let func: usize = match &cap[4].parse::<usize>() {
            Ok(n) => *n,
            Err(_) => {
                return None;
            }
        };
        let score = match &cap[5].parse::<f64>() {
            Ok(n) => *n,
            Err(_) => {
                return None;
            }
        };
        let mut chunk = Chunk {
            id: id,
            morphs: Vec::new(),
            dst: dst,
            srcs: Vec::new(),
            score: score,
            head: head,
            func: func,
        };
        for line in morph_lines {
            let morph = match Morpheme::from_text(line) {
                Some(m) => m,
                None => {
                    return None;
                }
            };
            chunk.morphs.push(morph);
        }
        Some(chunk)
    }

    pub fn phrase(&self) -> String {
        let mut s = String::new();
        for m in &self.morphs {
            s.push_str(&m.surface);
        }
        s
    }

    pub fn phrase_without_marks(&self) -> String {
        let mut s = String::new();
        for m in &self.morphs {
            if m.pos != "記号" {
                s.push_str(&m.surface);
            }
        }
        s
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Chunk({} {}D {}/{} {} srcs: {:?})",
            self.id,
            match self.dst {
                Some(n) => n as i64,
                None => -1,
            },
            self.head,
            self.func,
            self.score,
            self.srcs
        )?;
        for m in self.morphs.iter() {
            writeln!(f, "\t{}", m)?;
        }
        Ok(())
    }
}
