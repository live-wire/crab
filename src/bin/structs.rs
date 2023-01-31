#![allow(unused)]

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
