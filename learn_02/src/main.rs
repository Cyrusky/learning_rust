use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    const start: i32 = 1;
    const end: i32 = 100;
    let secret_number = rand::thread_rng().gen_range(start..=end);
    loop {
        println!("Please input the number from {} to {}: ", start, end);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You input a wrong number that can not be parsed, please retry.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Congratulations ! You got the nubmer!");
                break;
            }
        }
    }
}
