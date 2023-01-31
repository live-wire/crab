# RUST :crab:
---

- Following [this book](https://doc.rust-lang.org/book/title-page.html).

## Hello Rust
- Cargo package manager + build system
  - `cargo new <newproject>`
  - `cargo build` creates executable in `target/`
  - `cargo check` checks compilation errors without producing executables
  - `cargo run` from within a cargo repo, runs the executable

- **Variables** 
  - Immutable by default. `let a: u32 = 5;`
  - Redeclaration of same var names is allowed. `let a = "override previous value";`
  - Make Mutable. `let mut a = 5;` and now `a = 6;` is allowed.
  - Constants: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

- **Data types**
  - Int `i8`, ... `i128`, `u8` ... `u128`
    - Decimal `98_222`
    - Hex `0xff`
    - Octal `0o77`
    - Binary `0b1111_0000`
    - Byte (u8 only) `b'A'`
  - Float `f32` `3.0`
  - Boolean `bool` `true/false`
  - Character `char` `'😻'`
  - Compound types
    - tuple `let tup: (i32, f64, u8) = (500, 6.4, 1);`
      - Access elements like this `tup.0`, `tup.1`
      - Explode into variables like: `let (x, y, z) = tup;`
    - array `let a = [1, 2, 3, 4, 5];`
      - Access elements like `a[0]`.

- Valid program that panics when index is out of bounds
```
use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

- **Functions**
  - Most assignments are _statements_ (that do not return a value)
  - Calling a function is an _expression_. Calling a macro is an expression. A new scope block created with curly brackets is an expression.
```
// Sample expression
{
    let x = 3;
    x + 1
}

fn five() -> i32 {
    5
}
```

- **Control flow**
  - `let number = if condition { 5 } else { 6 };`


- **Loops**
```
let mut counter = 0;
// loop can also return a value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};

// while loops
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
```

- Loop over compound types:
```
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}

for number in (1..4).rev() {
    println!("{number}!");
}
```

## Ownership - what makes rust unique
- Rules:
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped. (when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable)
- All data stored on the stack must have a known, fixed size. 
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

```
let s1 = String::from("hello");
let s2 = s1;
```

- String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
- `move`: To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid.
- stack-only-data is `copied`. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
- Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the `Drop` trait.

## References
- Rust has a feature for using a value without transferring ownership, called references.
- The Rules of References:
  - At any given time, you can have either one mutable reference or any number of immutable references.
  - References must always be valid.
- Functions cannot return references of new variables (because the reference variable will go out of scope when the function ends).

```
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point (NLL-Non Lexical Lifetime)

    let r3 = &mut s; // no problem
    change(r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Slices
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
- A string slice (`&str`) is a reference to part of a String. (Immutable reference)
  
```
let s = String::from("hello");

let len = s.len();

let slice = &s[0..2];
let slice = &s[..2]; // same

let slice = &s[3..len];
let slice = &s[3..]; // same

let slice = &s[0..len];
let slice = &s[..]; // same
```

- Slices work on other types too: `let a = [1, 2, 3, 4, 5];` , `let slice = &a[1..3];` This slice has the type `&[i32]`
- The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.

