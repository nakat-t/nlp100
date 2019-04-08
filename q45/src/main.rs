use q45::*;
use std::io;
use std::io::BufReader;

fn read_sentence_from_cabocha_lattice<T: io::BufRead>(
    r: &mut T,
) -> io::Result<(usize, Vec<Chunk>)> {
    let mut read_bytes = 0usize;
    let mut sentence = Vec::new();
    let mut chunks_lines: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = r.read_line(&mut line)?;
        read_bytes += num_bytes;
        let crlf: &[_] = &['\r', '\n'];
        line.truncate(line.trim_end_matches(crlf).len());
        if num_bytes == 0 || line.starts_with("EOS") {
            if chunks_lines.len() >= 2 {
                if let Some(chunk) = Chunk::from_lattice_text(chunks_lines) {
                    sentence.push(chunk);
                }
            }
            for i in 0..sentence.len() {
                let src = &sentence[i];
                if let Some(dst) = src.dst {
                    if dst < sentence.len() {
                        sentence[dst].srcs.push(i);
                    }
                }
            }
            return Ok((read_bytes, sentence));
        }
        if line.starts_with("* ") {
            if chunks_lines.len() >= 2 {
                if let Some(chunk) = Chunk::from_lattice_text(chunks_lines) {
                    sentence.push(chunk);
                }
            }
            chunks_lines = Vec::new();
            chunks_lines.push(line);
            continue;
        } else {
            chunks_lines.push(line);
        }
    }
}

fn main() {
    let mut r = BufReader::new(io::stdin());
    loop {
        let s = match read_sentence_from_cabocha_lattice(&mut r) {
            Ok((n, _)) if n == 0 => break,
            Ok((_, s)) => s,
            Err(_) => break,
        };
        for i in 0..s.len() {
            let chunk = &s[i];
            if let Some(m) = chunk.morphs.iter().find(|&x| x.pos == "動詞") {
                let mut postpositions: Vec<String> = Vec::new();
                for src in chunk.srcs.iter().map(|&x| &s[x]) {
                    for pp in src.morphs.iter().filter(|x| x.pos == "助詞") {
                        postpositions.push(pp.base.clone());
                    }
                }
                if postpositions.len() > 0 {
                    postpositions.sort();
                    println!("{}\t{}", m.base, postpositions.join(" "));
                }
            }
        }
    }
}
