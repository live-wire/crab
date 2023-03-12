# RUST :crab:
---

> Following [this book](https://doc.rust-lang.org/book/title-page.html).

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

## Structs
- Just like structs in other languages to group different types of fields together.
- The entire instance of the struct has to be marked `mut` to make any field mutable.
- Structs can be accompanied by an `impl` block that houses all methods and functions related to that struct. `Self` refers to the original struct inside this block. Usually you can have a function in here that returns a new instance (like a constructor).
- Add `#[derive(Debug)]` before your struct if you want to `dbg!(...)` it or `println!("{:?}", ...)` it.
- Like tuples, structs can also just contain types and be inline: `struct Color(i32, i32, i32);`
- Extend a struct using values from another struct like this `User{ x: 12, ..extend_with }`
```
#[derive(Debug)]
struct User {
    name: String,
    age: usize,
}
fn email(user: &User) -> String {
    let mut x = user.name.clone();
    x.push_str("@gmail.com");
    x
}
impl User {
    fn email_method(&self) -> String {
        let mut x: String = self.name.clone();
        x.push_str("+method@gmail.com");
        x
    }
    fn new_user(name: String, age: usize) -> Self {
        User { name, age }
    }
}
fn main() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let mut user1: User = User {
        name: String::from("John"),
        age: 12,
    };
    user1.age = 13;
    let user2: User = User {
        name: String::from("Doe"),
        ..user1
    };
    let user3: User = User::new_user(String::from("Wick"), 39);
    let color1: Color = Color(255, 0, 255);
    dbg!(&color1);
    println!(
        "User1 {:?}, User2 {:?}, user3 {:?}, Color1 {:?}",
        user1.email_method(),
        email(&user2),
        user3,
        color1
    );
}
```

## Enums & Pattern Matching
- Enums like structs can have fields of a variety of types. Like nested struct like types.
- `Option<T>` enum is a great null checker generic enum (Just like in Scala). It forces you to worry about null checking. It can be `Some<T>` or `None`.
- `match {}` must always be exhaustive. Very similar to scala.
- Integers can also be range matched. Valid case example: `1..=20 => do_something`
- If a match only has one check, it can be made less verbose using `if let`:
```
let new_action: Option<Action> = Some(Action::Quit);
// THIS IS THE SAME AS
match new_action {
    Some(ref a) => println!("{:?}", a),
    _ => (),
};
// THIS
if let Some(b) = new_action {
    println!("{:?}", b)
}
```

## Packages, Crates and Modules
- Module < Crate < Package
- Crates can be one of these:
  - library - root @ `src/lib.rs`
  - binary - root @ `src/main.rs`
- Package must contain at least one crate (library or binary). It can contain as many binary crates as you like but just one library crate. Put all binary crate files in `src/bin/` to mark them as binary crates.
- Modules are private by default. Fields in a struct in a module are private by default. Also functions are private by default.
- Marking an enum `pub` is enough to make all its fields `pub` unlike in structs.
- Scope and privacy is controlled by modules. Code in modules is private by default.
- Alias for imports `use std::io::Result as IoResult;`
- Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. `pub use crate::some1::nested::modulex` in `src/lib.rs` will result it being available for external code to use it as `crate::modulex`.
- It is possible to import multiple things inline like `use std::{cmp::Ordering, io};`. For bringing a child and self in one line eg: `std::io` and `std::io::Write` use `use std::io::{self, Write};`.
- There is always the glob operator `use std::collections::*;`.
- Splitting modules in files. Declare module once (in one file) in `src/lib.rs` is enough. Example:
```
src/lib.rs ->
mod also_lib;
public use crate::also_lib::exported_module;

src/also_lib.rs ->
pub mod exported_module;

src/also_lib/exported_module.rs ->
pub mod exported_module {
    pub fn exported_fn() {}
}
```

- Then :point_up: in `src/main.rs` you can `use cratename::exported_module;`. Slightly tricky, look at [this cheatsheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) if you get confused.

## Collections
- These store data on the Heap. And the size can't be known at compile time.
- Vectors `Vec<T>`
  - `let v: Vec<i32> = Vec::new();`
  - or the macro `vec!` as `let v = vec![1,2,3];`
  - items can be iterated and updated using dereferencing(`*`) like this: `for i in &mut v { *i += 10 }`
  - When a vector goes out of scope, so do its elements.
- Strings
  - use `format!("{s1}-{s2}")` to concatenate strings. (Uses references, so doesn't take ownership)
  - methods like `contains` and `replace` exist
  - CanNOT reference by index. and `.len` is also unreliable.
  - Be explicit about you want to iterate over chars or bytes `for c in "str".chars() {}` or `for c in "str".bytes()`. Unicode scalar values may be made up of more than one byte.
- HashMaps
  - `let mut map = collections::HashMap::new();`
  - Insert and get like `map.insert(String::from("a"), 10);` get returns an Option. So `.unwrap()` it.
  - `map.entry(String::from("c")).or_insert(30);`
  - Uses SipHash by default as its hashing function.

## Error handling
- Types of errors in Rust:
  - Recoverable `Result<T, E>` yields an `Ok<T>` or `Err<E>`.
  - Non recoverable `panic!` - example: index out of bounds.
- You can deal with recoverable errors using pattern matching or with closures:
```
// Pattern match
let file_open = File::open("README.md");
match file_open {
    Ok(f) => println!("FOUND FILE {:?}", f),
    Err(e) => match e.kind() {
        ErrorKind::NotFound => panic!("CREATE FILE {:?}", e),
        x => panic!("SOME ERROR {:?}", x),
    },
}

// Closure
let file_open2 = File::open("README.md");
let f = file_open2.unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        panic!("CREATE FILE {:?}", error);
    } else {
        panic!("SOME ERROR {:?}", error);
    }
});
println!("FOUND FILE {:?}", f);
```

- If you want to panic on error, there are better shortcuts like: `.unwrap()` or `.expect("Error message")`.
- Error propagation shortcut: The amazing `?` operator can be used to return an error back to the parent function. (Of course the parent function should also return a `Result<T,E>`). It also works with `Option<T>`. Example:
```
fn read_username_from_file(filepath: String) -> Result<String, io::Error> {
    let fo = File::open(filepath);
    let mut ff = match fo {
        Ok(fi) => fi,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match ff.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// IS THE SAME AS THIS

fn read_username_from_file_clean(filepath: String) -> Result<String, io::Error> {
    let mut username: String = String::new();
    File::open(filepath)?.read_to_string(&mut username)?;
    Ok(username)
}
```

## Generics
---
- Specify type `<T>` in angular braces and restrict it to an interface like this: `<T: std::fmt::Display>`. Restrict to multiple traits separated by a +: `<T: std::cmp::PartialOrd + std::fmt::Display>`.
- Put it next to structs/enums/fn/methods etc.
```
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
```

- This implements the `distance_from_origin` only when T is an f32 else this function will not be available.
- *Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. So, the generated code is just as fast.

## Traits
---
- Like GoLang, traits are just groups of functions. (Can also have default implementations).
- You can implement a trait for a type as:
```
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// tweet.summarize(); is available
```

- To use the trait implementation functions, you need to import the trait explicitly as well. Like: `use aggregator::{Summary, Tweet};`
- You can implement an imported trait on a Type that your crate defines. (Not when both are imported. Duh).
- Traits can also have default implementations but you still need to explicitly define the impl for a type and import both trait and type to use it.
```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary for Tweet {}
```

- You can then ofcourse use traits in function definitions:
```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// IS THE SAME AS
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## In Practice
---
- Make sure add dependencies from `crates.io` to your Cargo.toml.
- Random number in a range: `let random_num: i32 = rand::thread_rng().gen_range(1..100);`
- 