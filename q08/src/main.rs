use std::char;

fn cipher(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
                char::from_u32(219 - c as u32).unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let s = "I am an NLPer";
    println!("original: {}", s);
    println!("encode  : {}", cipher(s));
    println!("decode  : {}", cipher(&cipher(s)));
}
