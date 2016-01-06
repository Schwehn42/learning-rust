use std::io; //import io lib for input

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new(); //input string that can be modified (hence mut)

    //read a line from the user.

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
