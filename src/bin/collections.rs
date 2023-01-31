#![allow(unused)]

use std::collections;
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    match v.get(100) {
        Some(a) => println!("{a}"),
        _ => (),
    }

    // WILL PANIC because &v will create an immutable reference
    // let does_not_exist = &v[10];
    // println!("{does_not_exist}");

    for i in &mut v {
        *i += 10;
        println!("{i}");
    }

    let mut map = collections::HashMap::new();
    map.insert(String::from("a"), 10);
    map.insert(String::from("b"), 20);
    map.entry(String::from("c")).or_insert(30);
    for (key, value) in &mut map {
        println!("{key} - {value}");
    }
    println!("{}", map.get("a").unwrap());
}
