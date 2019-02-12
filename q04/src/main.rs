use std::collections::HashMap;

fn q04() -> HashMap<String, i32> {
    let mut ret = HashMap::new();
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let words = s.split(|c| c == ' ' || c == '.' || c == ',').filter(|x| !x.is_empty());
    for x in words.enumerate() {
        if [1, 5, 6, 7, 8, 9, 15, 16, 19].iter().any(|&y| y == x.0 + 1) {
            ret.insert(String::from(&x.1[..1]), x.0 as i32);
        }
        else {
            ret.insert(String::from(&x.1[..2]), x.0 as i32);
        }
    }
    ret
}

fn main() {
    println!("{:?}", q04());
}
