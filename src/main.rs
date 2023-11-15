use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the Number!");

    println!("Please Input Your Guess: ");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The Secret Number is: {secret_number}");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess:u32 = guess.trim().parse().expect("Please type a Number!");

    println!("You Guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!"),
        
    }

}