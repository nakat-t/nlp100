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

fn delete_file_template(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\[\[ファイル:([^|]+)\|([^|]+)\|([^|]+)\]\]").unwrap();
    }
    RE.replace_all(s, "$3 ($1)").to_string()
}

fn delete_lang_template(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{\{lang\|(en|fr)\|([^}]+)\}\}").unwrap();
    }
    RE.replace_all(s, "$2").to_string()
}

fn delete_ref(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?m)(<ref( [^/>]*)?/>|<ref( [^/>]*)?>[^<]*</ref>|<references />)")
                .unwrap();
    }
    RE.replace_all(s, "").to_string()
}

fn escape_pound(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"&pound;").unwrap();
    }
    RE.replace_all(s, "£").to_string()
}

fn escape_br(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<br\s*/>").unwrap();
    }
    RE.replace_all(s, "\n").to_string()
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
    let s = delete_internal_link(&s);
    let s = escape_br(&s);
    let s = escape_pound(&s);
    let s = delete_ref(&s);
    let s = delete_lang_template(&s);
    let s = delete_file_template(&s);
    s
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
        let re_attr = Regex::new(r"\|(.*?) = (.*)").unwrap();
        let mut infobox: HashMap<String, String> = HashMap::new();
        let mut attr = String::new();
        let infobox_lines = text
            .lines()
            .skip_while(|l| !re_begin.is_match(l))
            .take_while(|l| !re_end.is_match(l));
        for line in infobox_lines {
            if let Some(cap) = re_attr.captures(line) {
                attr = String::from(&cap[1]);
                infobox.insert(attr.clone(), cap[2].to_string());
            } else {
                infobox
                    .entry(attr.clone())
                    .and_modify(|e| *e += &format!("\n{}", line));
            }
        }
        for v in infobox.values_mut() {
            *v = delete_markup(v);
        }
        let image_file = infobox.get("国旗画像").unwrap();
        let req_url = format!("https://commons.wikimedia.org/w/api.php?action=query&titles=File:{}&prop=imageinfo&iiprop=url&format=json", image_file);
        let mut res = reqwest::get(&req_url).unwrap();
        let body = res.text().unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let file_url = &json["query"]["pages"]["347935"]["imageinfo"][0]["url"];
        println!("{}", file_url.as_str().unwrap());
    }
}
