use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let start = 1;
    let end = 100;
    let secret_number = rand::thread_rng().gen_range(start..=end);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input the number from {} to {}: ", start, end);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

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
