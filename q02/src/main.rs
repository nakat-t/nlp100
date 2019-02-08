fn q02() -> String {
    "パトカー".chars()
        .zip("タクシー".chars())
        .map(|x| vec![x.0, x.1])
        .flatten()
        .collect()
}

fn main() {
    println!("{}", q02());
}
