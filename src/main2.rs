use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn  main() {
    
    println!("Welcome to the guessing game");
    
    let _secret_key = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("enter your guess");
        let mut _guess = String::new();

        io::stdin().read_line(&mut _guess)
                    .expect("Failed to read line");

        let _guess : u32 = match _guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
            
        };

        println!("you guessed {}", _guess);

        match _guess.cmp(&_secret_key) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Yatta u guessed it right");
                break;
            }
            
        }
    }


    
}