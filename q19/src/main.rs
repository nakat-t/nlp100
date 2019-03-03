use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut lines: Vec<Vec<String>> = handle
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    let mut map = HashMap::new();
    for l in lines.iter() {
        let entry = map.entry(l[0].clone()).or_insert(0);
        *entry += 1;
    }
    lines.sort_by(|x, y| {
        let xe = map.get(&x[0]).unwrap();
        let ye = map.get(&y[0]).unwrap();
        (*ye)
            .partial_cmp(xe)
            .unwrap()
            .then(x[0].partial_cmp(&y[0]).unwrap())
    });
    for l in lines.iter() {
        println!("{}", l.join("\t"));
    }
}
