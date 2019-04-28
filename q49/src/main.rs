use q49::*;
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

fn get_dsts(x: usize, chunks: &Vec<Chunk>) -> Vec<usize> {
    let mut i = x;
    let mut v = vec![x];
    while let Some(dst) = chunks[i].dst {
        i = dst;
        v.push(i);
    }
    v
}

fn get_branches(vx: &Vec<usize>, vy: &Vec<usize>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let xlen = vx.len();
    let ylen = vy.len();
    let mut n = 0;
    while n < xlen && n < ylen {
        if vx[xlen - n - 1] != vy[ylen - n - 1] {
            break;
        }
        n += 1;
    }
    let vc: Vec<usize> = vx[xlen - n..xlen].iter().cloned().collect();
    let vx: Vec<usize> = vx[0..xlen - n].iter().cloned().collect();
    let vy: Vec<usize> = vy[0..ylen - n].iter().cloned().collect();
    (vx, vy, vc)
}

fn path_string(x: usize, y: usize, chunks: &Vec<Chunk>) -> Option<String> {
    let len = chunks.len();
    assert!(x < len && y < len);
    let vx = get_dsts(x, chunks);
    let vy = get_dsts(y, chunks);
    let (vx, vy, vc) = get_branches(&vx, &vy);
    if vc.len() == 0 {
        return None;
    } else if vy.len() == 0 {
        let mut ret: String = vx
            .iter()
            .map(|&x| chunks[x].phrase_without_marks())
            .collect::<Vec<String>>()
            .join(" -> ");
        if let Some(&i) = vc.iter().next() {
            ret.push_str(" -> ");
            ret.push_str(&chunks[i].phrase_without_marks());
        }
        return Some(ret);
    } else {
        let xs: String = vx
            .iter()
            .map(|&i| chunks[i].phrase_without_marks())
            .collect::<Vec<String>>()
            .join(" -> ");
        let ys: String = vy
            .iter()
            .map(|&i| chunks[i].phrase_without_marks())
            .collect::<Vec<String>>()
            .join(" -> ");
        let cs: String = vc
            .iter()
            .map(|&i| chunks[i].phrase_without_marks())
            .collect::<Vec<String>>()
            .join(" -> ");
        return Some(format!("{} | {} | {}", xs, ys, cs));
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
            for j in i + 1..chunks.len() {
                let ci = &chunks[i];
                let cj = &chunks[j];
                let ci_noun = ci.morphs.iter().find(|&x| x.pos == "名詞");
                let cj_noun = cj.morphs.iter().find(|&x| x.pos == "名詞");
                if ci_noun.is_some() && cj_noun.is_some() {
                    if let Some(s) = path_string(i, j, &chunks) {
                        let x = &ci_noun.unwrap().surface;
                        let y = &cj_noun.unwrap().surface;
                        println!("{}", s.replace(x, "X").replace(y, "Y"));
                    }
                }
            }
        }
    }
}
