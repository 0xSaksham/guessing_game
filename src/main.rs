use std::io;

fn main(){
    println!("Guess the Number!");

    println!("Please Input Your Guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You Guessed: {}",guess);

}