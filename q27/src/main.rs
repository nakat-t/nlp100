use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Serialize, Deserialize, Debug)]
struct JaWikiCountryJson {
    text: String,
    title: String,
}

fn delete_strong(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"'''?").unwrap();
    }
    RE.replace_all(s, "").to_string()
}

fn delete_internal_link(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[\[([^|\]]+)(?:\|([^|\]]+))?\]\]").unwrap();
    }
    RE.replace_all(s, |caps: &Captures| match caps.get(2) {
        Some(cap) => cap.as_str().to_string(),
        None => caps[1].to_string(),
    })
    .to_string()
}

fn delete_markup(s: &str) -> String {
    let s = delete_strong(s);
    delete_internal_link(&s)
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut map = HashMap::new();
    for l in handle.lines() {
        let json: JaWikiCountryJson =
            serde_json::from_str(&l.unwrap()).expect("Invalid JSON format");
        map.entry(json.title).or_insert(json.text);
    }
    if let Some(text) = map.get("イギリス") {
        let re_begin = Regex::new(r"\A\{\{基礎情報").unwrap();
        let re_end = Regex::new(r"\A\}\}").unwrap();
        let re_attr = Regex::new(r"\|(\S+)\s*=\s*(.*)").unwrap();
        let mut infobox: HashMap<String, String> = HashMap::new();
        let mut attr = String::new();
        let infobox_lines = text
            .lines()
            .skip_while(|l| !re_begin.is_match(l))
            .take_while(|l| !re_end.is_match(l));
        for line in infobox_lines {
            if let Some(cap) = re_attr.captures(line) {
                attr = String::from(&cap[1]);
                infobox.insert(attr.clone(), delete_markup(&cap[2]));
            } else {
                infobox
                    .entry(attr.clone())
                    .and_modify(|e| *e += &format!("\n{}", delete_markup(line)));
            }
        }
        println!("{:#?}", infobox);
    }
}
