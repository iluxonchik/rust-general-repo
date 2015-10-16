extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // create an empty string
        io::stdin().read_line(&mut guess)
                .ok()
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // trim - eliminate any whitespace
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try higher!"),
            Ordering::Greater => println!("Try lower!"),
            Ordering::Equal => { 
                println!("Good job! You've gussed it!");
                break;
            }
        }
    }
}
