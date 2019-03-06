use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Serialize, Deserialize, Debug)]
struct JaWikiCountry {
    text: String,
    title: String,
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut map = HashMap::new();
    for l in handle.lines() {
        let entry: JaWikiCountry = serde_json::from_str(&l.unwrap()).expect("Invalid JSON format");
        map.entry(entry.title.clone()).or_insert(entry);
    }
    if let Some(country) = map.get("イギリス") {
        for line in country.text.lines() {
            if line.starts_with("[[Category:") {
                println!("{}", line);
            }
        }
    }
}
