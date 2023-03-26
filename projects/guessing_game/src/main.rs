use std::io;
use std::cmp::Ordering;
use rand::Rng;

// without looping
/*
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();  // mutable

    io::stdin()
        .read_line(&mut guess)  // references are immutable by default, so &mut to make it mutable
        .expect("Failed to read line!");  // like golang, this is error handling. by default return Result and err

    // even though there is variable guess before this, rust has a feature called `shadowing`
    // Shadowing allows to reuse guess variable to create two unique variables
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    /*
    without code above wiill produce error

    |
22  |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
    |                 |
    |                 arguments to this method are incorrect
    |
    = note: expected reference `&String`
              found reference `&{integer}`
    */

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
*/

// with looping
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // mutable

        io::stdin()
            .read_line(&mut guess)  // references are immutable by default, so &mut to make it mutable
            .expect("Failed to read line!");  // like golang, this is error handling. by default return Result and err

        // even though there is variable guess before this, rust has a feature called `shadowing`
        // Shadowing allows to reuse guess variable to create two unique variables
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /*
        without code above wiill produce error

        |
    22  |     match guess.cmp(&secret_number) {
        |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
        |                 |
        |                 arguments to this method are incorrect
        |
        = note: expected reference `&String`
                found reference `&{integer}`
        */

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
