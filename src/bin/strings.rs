fn main() {
    let mut s = String::from("नमस्ते");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point (NLL-Non Lexical Lifetime)

    let r3 = &mut s; // no problem
    change(r3);
    println!("{}", r3);
    println!("{}", s.contains("lo"));

    let mut i: i32 = 0;
    for c in s.chars() {
        i += 1;
        println!("CHAR {c}")
    }
    println!("{}", i);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
