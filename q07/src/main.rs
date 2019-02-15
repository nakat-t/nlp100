use std::any::Any;
use std::fmt::Display;

fn q07<T1: Any + Display, T2: Any + Display, T3: Any + Display>(x: T1, y: T2, z: T3) -> String {
    format!("{}時の{}は{}", x, y, z)
}

fn main() {
    println!("{}", q07(12, "気温", 22.4));
}
