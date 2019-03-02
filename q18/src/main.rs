use std::io::BufRead;

fn column(s: &str, n: usize) -> Option<String> {
    s.split_whitespace().nth(n).map(|x| x.to_string())
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut lines: Vec<String> = handle.lines().map(|l| l.unwrap()).collect();
    lines.sort_by(|x, y| column(y, 2).partial_cmp(&column(x, 2)).unwrap());
    for l in lines {
        println!("{}", l);
    }
}
