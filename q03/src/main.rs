fn q03() -> Vec<i32> {
    "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."
        .split(|c| c == ' ' || c == '.' || c == ',')
        .filter(|x| !x.is_empty())
        .map(|x| x.len() as i32)
        .collect()
}

fn main() {
    println!("{:?}", q03());
}
