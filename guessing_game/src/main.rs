use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..100);
        println!("secret number is {secret_number}");
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read the Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("guessed right!!!");
                break;
            }
            Ordering::Greater => println!("guessed small"),
            Ordering::Less => println!("guessed big"),
        }
    }
}
