use rand::Rng;
use std::cmp::Ordering;
use std::io;
use structopt::StructOpt;

/// Set Difficulty via CLI
#[derive(StructOpt)]
struct Cli {
    /// Difficulty of the Game: "easy", "medium", "hard"
    #[structopt(default_value = "easy", long)]
    difficulty: String,
}

#[test]
fn test_print_difficulty_empty() {
    let mut result = Vec::new();
    print_difficulty("", &mut result);
    assert_eq!(result, b"Playing on: \n");
}

#[test]
fn test_print_difficulty() {
    let mut result = Vec::new();
    print_difficulty("easy", &mut result);
    assert_eq!(result, b"Playing on: easy\n");
}

fn print_difficulty(difficulty: &str, mut writer: impl std::io::Write) {
    writeln!(writer, "Playing on: {}", difficulty);
}

fn main() {
    println!("Welcome to the Guessing Game");

    let args = Cli::from_args();

    let difficulty = match &args.difficulty as &str {
        "easy" => {
            print_difficulty(&args.difficulty, &mut std::io::stdout());
            (10, 101)
        }
        "medium" => {
            print_difficulty(&args.difficulty, &mut std::io::stdout());
            (5, 201)
        }
        "hard" => {
            print_difficulty(&args.difficulty, &mut std::io::stdout());
            (3, 301)
        }
        _ => {
            print_difficulty("easy", &mut std::io::stdout());
            (10, 101)
        }
    };

    let min = 0;
    let (mut tries, max) = difficulty;

    let secret_number = rand::thread_rng().gen_range(min..max);

    while tries != 0 {
        let mut guess = String::new();
        println!("Please select a Number between {} and {}", min, (max - 1));
        println!("You have {} tries left", tries);
        io::stdin()
            .read_line(&mut guess)
            .expect("error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&max) {
            Ordering::Less => match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Need to Guess Higher!");
                    tries = tries - 1;
                }
                Ordering::Greater => {
                    println!("Guess Lower!");
                    tries = tries - 1;
                }
                Ordering::Equal => {
                    println!("You Win!");
                    std::process::exit(0);
                }
            },
            Ordering::Greater => {
                println!("{} is over {}, you'll never win", guess, (max - 1));
                tries = tries - 1;
            }
            Ordering::Equal => {
                println!("{} is over {}, you'll never win", guess, (max - 1));
                tries = tries - 1;
            }
        };
    }
    println!("You Lose!");
    println!("The Correct Answer: {}", secret_number);
}
