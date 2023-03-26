use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // mutable

    io::stdin()
        .read_line(&mut guess)  // references are immutable by default, so &mut to make it mutable
        .expect("Failed to read line!");  // like golang, this is error handling. by default return Result and err

    println!("You guessed: {guess}");
}
