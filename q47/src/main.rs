use q47::*;
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
        let chunks = match read_sentence_from_cabocha_lattice(&mut r) {
            Ok((n, _)) if n == 0 => break,
            Ok((_, chunks)) => chunks,
            Err(_) => break,
        };
        for i in 0..chunks.len() {
            let chunk = &chunks[i];
            let mut pred = None;
            let mut pred_src_idx = None;
            let mut particle_term: Vec<(String, String)> = Vec::new();
            if let Some(m) = chunk.morphs.iter().find(|&x| x.pos == "動詞") {
                for j in &chunk.srcs {
                    let src = &chunks[*j];
                    if src.morphs.len() != 2 {
                        continue;
                    }
                    let first = &src.morphs[0];
                    let last = &src.morphs[1];
                    if first.pos == "名詞"
                        && first.pos1 == "サ変接続"
                        && last.pos == "助詞"
                        && last.base == "を"
                    {
                        pred = Some(format!("{}{}", src.phrase(), m.base));
                        pred_src_idx = Some(*j);
                    }
                }
                if pred.is_some() {
                    for j in &chunk.srcs {
                        if pred_src_idx.unwrap() == *j {
                            continue;
                        }
                        let src = &chunks[*j];
                        let last_morph = &src.morphs[src.morphs.len() - 1];
                        if last_morph.pos == "助詞" {
                            particle_term.push((last_morph.base.clone(), src.phrase()));
                        }
                    }
                }
                if pred.is_some() {
                    particle_term.sort_by(|a, b| (a.0).cmp(&b.0));
                    let particles: Vec<_> = particle_term.iter().map(|x| x.0.clone()).collect();
                    let terms: Vec<_> = particle_term.iter().map(|x| x.1.clone()).collect();
                    println!(
                        "{}\t{}\t{}",
                        pred.unwrap(),
                        particles.join(" "),
                        terms.join(" ")
                    );
                }
            }
        }
    }
}
