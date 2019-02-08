fn q01() -> String {
    [1, 3, 5, 7]
        .iter()
        .map(|n| "パタトクカシーー".chars().nth(*n as usize).unwrap())
        .collect()
}

fn main() {
    println!("{}", q01());
}
