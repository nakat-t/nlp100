use regex::Regex;
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
        let re_begin = Regex::new(r"\A\{\{基礎情報").unwrap();
        let re_end = Regex::new(r"\A\}\}").unwrap();
        let re_entry = Regex::new(r"\|(\S+)\s*=\s*(.*)").unwrap();
        let mut in_basic_info = false;
        let mut basic_info: HashMap<String, String> = HashMap::new();
        let mut entry = String::new();
        for line in country.text.lines() {
            if re_begin.is_match(line) {
                in_basic_info = true;
            }
            if re_end.is_match(line) {
                in_basic_info = false;
            }
            if in_basic_info {
                if let Some(cap) = re_entry.captures(line) {
                    entry = String::from(&cap[1]);
                    basic_info
                        .entry(entry.clone())
                        .or_insert(String::from(&cap[2]));
                } else {
                    basic_info.entry(entry.clone()).and_modify(|e| {
                        *e += "\n";
                        *e += line;
                    });
                }
            }
        }
        println!("{:#?}", basic_info);
    }
}
