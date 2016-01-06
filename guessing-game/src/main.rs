extern crate rand; //random crate from cargo

use std::io; //import io lib for input
use rand::Rng; // import rand lib (from cargo) to gen rand nums

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret number: {}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new(); //input string that can be modified (hence mut)

    //read a line from the user.

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
