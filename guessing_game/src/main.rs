use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get the user input");
        // let guess: u32 = guess.trim().parse().expect("Failed to pass");
        println!("Your guess {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please enter a valid input");
                continue;
            }
        }; 

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won !!");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Small"),
        }
    }
}
