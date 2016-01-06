extern crate rand; //random crate from cargo

use std::io; //import io lib for input
use std::cmp::Ordering; //be able to compare input with secet num
use rand::Rng; // import rand lib (from cargo) to gen rand nums

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret number: {}", secret_number);

    //loop this forever until win
    loop {
        println!("Please input your guess");

        let mut guess = String::new(); //input string that can be modified (hence mut)

        //read a line from the user.
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        //convert string to unsigned int 32bit.
        //take away its mutability
        let guess: u32 = guess.trim().parse()
                .ok()
                .expect("Please type a number!");

        println!("You guessed: {}", guess);

        //compare guess with secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //break loop
            }
        }
    }
}
