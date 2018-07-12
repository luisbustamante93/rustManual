extern crate rand; //Importing libs

use std::io; // Standard input output library
use rand::Rng; //Call of functions


fn main(){
    println!("*****************");
    println!("GUESS THE NUMBER!");
    println!("*****************");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let x = 5;
    let y = 10;

    println!("Please input your guess");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");        

    println!("You guessed: {}", guess);
    println!("Values: X = {} and Y = {}", x, y)
}
