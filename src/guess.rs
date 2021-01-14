use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        print!("Please input your guess:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: u32 = match input.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("Your guessed: {}", input);
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}