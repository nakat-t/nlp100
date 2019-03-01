use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines().map(|l| l.unwrap());
    let col1s = lines.map(|l| l.split_whitespace().nth(0).unwrap().to_string());
    let mut col1s: Vec<String> = col1s.collect();
    col1s.sort();
    col1s.dedup();
    for l in col1s {
        println!("{}", l);
    }
}
