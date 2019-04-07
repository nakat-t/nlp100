use q43::*;
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
            let src = &s[i];
            let dst = src.dst.map(|d| &s[d]);
            let is_src_contains_noun = src.morphs.iter().any(|x| x.pos == "名詞");
            let is_dst_contains_verb =
                dst.map_or(false, |c| c.morphs.iter().any(|x| x.pos == "動詞"));
            if is_src_contains_noun && is_dst_contains_verb {
                println!(
                    "{}\t{}",
                    src.phrase_without_marks(),
                    dst.unwrap().phrase_without_marks()
                );
            }
        }
    }
}
