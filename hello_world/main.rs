
use std::io;

fn main() {
    println!("Starting a Guessing Game!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);

}
