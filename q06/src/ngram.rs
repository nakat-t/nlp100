pub fn ngram(n: usize, s: &str) -> Vec<String> {
    let char_len = s.chars().count();
    let mut v = Vec::new();
    for i in 0..char_len {
        let iter = s.chars().skip(i).take(n);
        let elem: String = iter.collect();
        v.push(elem);
    }
    v
}
