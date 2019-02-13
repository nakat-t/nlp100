fn n_gram_words(n: u32, words: Vec<&str>) -> Vec<Vec<String>> {
    let mut v = Vec::new();
    for i in 0..words.len() {
        let iter = words.iter().skip(i).take(n as usize);
        let mut elem = Vec::new();
        for x in iter {
            elem.push(String::from(*x));
        }
        v.push(elem);
    }
    v
}

fn n_gram(n: u32, chars: Vec<char>) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..chars.len() {
        let iter = chars.iter().skip(i).take(n as usize);
        let elem: String = iter.collect();
        v.push(elem);
    }
    v
}

fn main() {
    println!(
        "{:?}",
        n_gram_words(2, "I am an NLPer".split(|c| c == ' ').collect())
    );
    println!("{:?}", n_gram(2, "I am an NLPer".chars().collect()));
    println!("{:?}", n_gram(2, "こんにちは".chars().collect()));
}
