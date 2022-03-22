fn main() {
    let a = Some(5);
    let b: Option<i32> = None;
    match a {
        None => println!("Test"),
        Some(a) => println!("{}", a),
    }
    match b {
        None => println!("Test"),
        Some(b) => println!("{}", b),
    }
}
