use std::io::BufRead;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
        println!("{}", country.text);
    }
    else {
        println!("JSON entry 'イギリス' not found");
    }
}
