use q40::Morpheme;
use std::io;
use std::io::BufReader;

fn read_sentence_from_cabocha_lattice<T: io::BufRead>(r: &mut T) -> io::Result<Vec<Morpheme>> {
    let mut sentence = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = r.read_line(&mut line)?;
        if num_bytes == 0 || line.starts_with("EOS") {
            return Ok(sentence);
        }
        if line.starts_with("* ") {
            continue;
        }
        let crlf: &[_] = &['\r', '\n'];
        if let Some(m) = Morpheme::from_text(&line.trim_end_matches(crlf)) {
            sentence.push(m);
        }
    }
}

fn main() {
    let mut r = BufReader::new(io::stdin());
    let _ = read_sentence_from_cabocha_lattice(&mut r);
    let _ = read_sentence_from_cabocha_lattice(&mut r);
    let v = read_sentence_from_cabocha_lattice(&mut r).expect("failed to read cabocha lattice");
    for m in &v {
        println!("{}", m);
    }
}
