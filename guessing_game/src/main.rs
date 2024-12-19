use rand::Rng;
use std::io;
use std::cmp::Ordering;

const LOW_NUMBER: u8 = 0;
const HIGH_NUMBER: u8 = 100;

fn main() {
    println!("Guess a number between {LOW_NUMBER} and {HIGH_NUMBER}");
    let number = rand::thread_rng().gen_range(LOW_NUMBER..=HIGH_NUMBER);

    loop {
        println!();
        let guess: String = input("Please input your guess");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number must be between {LOW_NUMBER} and {HIGH_NUMBER}");
                continue
            },
        };
        if guess < LOW_NUMBER || guess > HIGH_NUMBER {
            println!("Number must be between {LOW_NUMBER} and {HIGH_NUMBER}");
            continue;
        }
        println!("\nYou guessed {guess}");
        match guess.cmp(&number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("{number} is correct - you win!");
                break;
            }
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}
