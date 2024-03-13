use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

fn main() {
    println!("Welcome to the world!");

    let user = User{
        manager: Box::from(Option::from(User {
            manager: Box::from(None),
            username: "blackbeard".to_string(),
            email: "blackbeard@yonkou.com".to_string(),
            sign_in_count: 0,
        })),
        username: "aokiji".to_string(),
        email: "kuzan@punkhazard.com".to_string(),
        sign_in_count: 0,
    };
    println!("user is {:?}", user);
    user.print_email();
    exit(0);
    loop {
        println!("Select one of the options to do something");
        println!("1. Play the guessing game");
        // TODO add more options here
        println!("0. Crash and burn");


        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let guess: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess {
            0 => {
                println!("Sayonara ...");
                break;
            }
            1 => play_guessing_game(),
            _ => {
                println!("Invalid option...");
                continue;
            }
        }
    }
}

#[derive(Debug)]

struct User {
    manager: Box<Option<User>>,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User{
    fn print_email(self){
        println!("User email is {}",self.email)
    }
}

fn play_guessing_game() {
    println!("*****************************");
    println!("Welcome to the guessing game");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("*****************************");
                break;
            }
        }
    }
}
