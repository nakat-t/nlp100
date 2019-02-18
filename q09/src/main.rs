use rand::seq::SliceRandom;
use rand::thread_rng;

fn q09(s: &str) -> String {
    let words = s.split_whitespace();
    let mut ret: Vec<String> = Vec::new();
    for w in words {
        if w.len() <= 4 {
            ret.push(String::from(w));
        } else {
            let first = w.chars().next().unwrap();
            let last = w.chars().last().unwrap();
            let mut middle: Vec<char> = w.chars().skip(1).take(w.len() - 2).collect();
            middle.shuffle(&mut thread_rng());
            let mut wr: String = String::new();
            wr.push(first);
            wr.push_str(&middle.iter().collect::<String>());
            wr.push(last);
            ret.push(wr);
        }
    }
    ret.join(" ")
}

fn main() {
    println!("{}", q09("I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind ."));
}
