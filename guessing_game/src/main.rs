use std::io;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    let mut lives = 4;

    while lives > 0 {

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Guess the number or press q to quit");
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "q" {
            println!("You quit the game. SAD!");
            break;
        }

        if secret_number == guess.trim().parse::<u32>().unwrap() {
            println!("You guessed the correct number. You win!");
        } else {
            lives -= 1;
            guess = String::new();
            println!("The correct number is {secret_number}. You did not guess it!");
            println!("You have {lives} lives left!");
        }

        if lives == 0 {
            println!("You have no more lives left! You lose!");
        }
        println!();
    }
}