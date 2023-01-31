fn string_reference() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point (NLL-Non Lexical Lifetime)

    let r3 = &mut s; // no problem
    change(r3);
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
