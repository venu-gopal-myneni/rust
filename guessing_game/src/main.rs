
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Starting a Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number. You typed {} which can't be converted to a number", guess);
                continue;
            }

        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is {}.", secret_number);
                break;
            }
        }
    }

}
