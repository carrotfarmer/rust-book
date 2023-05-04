use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    println!("Guess the number!");
    let rand_num = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("\nYou guessed: {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("{}", "too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
            Ordering::Greater => println!("{}", "too big".red()),
        };
    }
}
