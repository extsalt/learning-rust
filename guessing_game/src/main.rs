extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    println!("Please enter your guess:");

    loop {
        let mut guess = String::new(); //create new string variable
        //read user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //parse string to appropriate numerical value
        let guess: u32 = guess.trim().parse()
            .expect("Invalid #");

        println!("You guessed: {}", guess);

        //generate random number
        let random = rand::thread_rng().gen_range(1, 100);

        println!("Secret Number: {}", random);

        //match for random number
        match guess.cmp(&random) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
