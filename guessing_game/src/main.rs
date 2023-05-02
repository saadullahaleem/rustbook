use std::io;

fn main() {
    let mut guess = String::new();
    let mut choice = String::from("y");

    while choice.trim() == "y" {
        println!("Guess the number!");
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        println!("Do you want to play again? (y/n)");
        choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if choice.trim() == "y" {
            println!("You chose to play again! Nice!");
        } else {
            println!("You chose to quit!, SAD!");
        }
        println!();
    }
}