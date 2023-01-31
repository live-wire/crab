#![allow(unused)]

#[derive(Debug)]
enum Action {
    Move { x: i32, y: i32 },
    Quit,
    Describe(String),
}
fn main() {
    let movenow: Action = Action::Move { x: 12, y: 33 };
    let s = do_it(&movenow);
    println!("{}", s);

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

    let i: i32 = 5;
    match i {
        1..=20 => println!("WINNER"),
        _ => (),
    }
}
fn do_it(action: &Action) -> String {
    match action {
        Action::Move { x, y } => std::fmt::format(format_args!("||{:?}||", action)),
        Action::Quit => String::from("QUIT"),
        _ => String::from("DONE"),
    }
}
