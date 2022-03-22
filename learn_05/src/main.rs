struct User {
    name: String,
    age: i32,
}

fn main() {
    let user1 = User {
        name: String::from("Test"),
        age: 32,
    };
    let user2 = User {
        name: String::from("Test2"),
        ..user1
    };
    let name = user1.name;
    println!("{}, {}", name, user1.age);
}
