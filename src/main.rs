use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let min = 0;
    let max = 101;
    let mut tries = 5;
    let secret_number = rand::thread_rng().gen_range(min..max);
    
    println!("Welcome to the Guessing Game");
   
    while tries != 0 {
        let mut guess = String::new();
        println!("Please select a Number between {} and {}", min, (max - 1));
        println!("You have {} tries left", tries);
        io::stdin().read_line(&mut guess).expect("error reading input");
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&max) {
            Ordering::Less => match guess.cmp(&secret_number) {
                Ordering::Less => {println!("Less"); tries = tries - 1;},
                Ordering::Greater => {println!("Greater"); tries = tries - 1;},
                Ordering::Equal => {
                    println!("Equal");
                    break;
                }
            },
            Ordering::Greater => {
                println!("{} is over {}, you'll never win", guess, (max -1));
                tries = tries - 1;
                },
            Ordering::Equal => {
                println!("{} is over {}, you'll never win", guess, (max -1)); tries = tries - 1;},
        };
    };
    println!("You Lose!");
}
