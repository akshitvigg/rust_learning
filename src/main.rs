use std::io;
use rand::Rng;

fn main(){
    println!("Welcome the Guessing Game!");

    let _secret_key = rand::thread_rng().gen_range(1..=100);
    println!("the secret key is {}", _secret_key);

    println!("Enter the value");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    println!("the entered value is {}", guess);


}