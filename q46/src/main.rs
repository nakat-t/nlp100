use q46::*;
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
                let mut particle_term: Vec<(String, String)> = Vec::new();
                for src in chunk.srcs.iter().map(|&x| &s[x]) {
                    let last_morph = &src.morphs[src.morphs.len() - 1];
                    if last_morph.pos == "助詞" {
                        particle_term.push((last_morph.base.clone(), src.phrase()));
                    }
                }
                if particle_term.len() > 0 {
                    particle_term.sort_by(|a, b| (a.0).cmp(&b.0));
                    let particles: Vec<_> = particle_term.iter().map(|x| x.0.clone()).collect();
                    let terms: Vec<_> = particle_term.iter().map(|x| x.1.clone()).collect();
                    println!("{}\t{}\t{}", m.base, particles.join(" "), terms.join(" "));
                }
            }
        }
    }
}
