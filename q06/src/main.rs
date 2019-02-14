use q06::ngram::*;
use std::collections::HashSet;

fn main() {
    let x: HashSet<_> = ngram(2, "paraparaparadise").iter().cloned().collect();
    let y: HashSet<_> = ngram(2, "paragraph").iter().cloned().collect();
    println!("x | y: {:?}", x.union(&y));
    println!("x & y: {:?}", x.intersection(&y));
    println!("x - y: {:?}", x.difference(&y));
    println!("y - x: {:?}", y.difference(&x));
    println!("x contains 'se': {:?}", x.contains(&String::from("se")));
    println!("y contains 'se': {:?}", y.contains(&String::from("se")));
}
