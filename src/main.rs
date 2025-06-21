use std::io;
use rand::Rng;

fn main(){
    println!("Welcome the Guessing Game!");

    let secret_num = rand::thread_rng().gen_range(1..=100); 

    println!("Enter the value");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    println!("the entered value is {}", guess);


}