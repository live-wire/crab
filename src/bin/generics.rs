#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn pp<T: std::fmt::Display>(s: T) {
    println!("{}", s);
}

fn cmp<T: std::cmp::PartialOrd + std::fmt::Display>(a: T, b: T) -> bool {
    match a > b {
        true => return true,
        false => (),
    }
    println!("{}", a);
    false
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p2 = Point { x: 5.0, y: 10.0 };
    println!("p.dfo = {}", p2.distance_from_origin());
    pp(p.x.to_string());
    println!("{}", cmp(15, 9));
}
