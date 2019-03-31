use std::fmt;

#[derive(Debug, Clone)]
pub struct Morpheme {
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
    pub fn from_text(text: &str) -> Option<Morpheme> {
        let v: Vec<_> = text.split("\t").collect();
        if v.len() < 2 {
            return None;
        }
        let surface = v[0];
        let v: Vec<_> = v[1].split(",").collect();
        if v.len() < 9 {
            return None;
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
