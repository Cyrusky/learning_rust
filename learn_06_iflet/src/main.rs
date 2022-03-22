fn main() {
    let v = Some(0i8);
    if let Some(3) = v {
        println!("Success")
    } else if let Some(4) = v {
        println!("Error")
    } else if let Some(5) = v {
        println!("5")
    } else {
        println!("2")
    }
}
