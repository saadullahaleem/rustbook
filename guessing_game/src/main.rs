use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut lives = 4;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    while lives > 0 {

        println!("You have \x1b[93m{lives}\x1b[0m lives left!");
        println!("Guess the number or press q to quit");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "q" {
                    println!("You quit the game!");
                    break;
                } else {
                    println!("Please enter a number!");
                    println!();
                    continue;
                }
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives -= 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                lives -= 1;
            },
            Ordering::Equal => {
                println!("You guessed the correct number. You win!");
                break;
            },
        }

        if lives == 0 {
            println!("You have no more lives left! You lose!");
            println!("The secret number was: {}", secret_number);
        }
        println!();
    }
}