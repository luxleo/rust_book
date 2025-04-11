use std::cmp::Ordering;
use std::io;
use rand::Rng;
pub fn run_game(){
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..101);
    println!("The secret number is {secret_number}");
    loop {
        let mut guess = String::new(); // mut to make variable ( default is immutable)
        io::stdin().read_line(&mut guess).expect("Failed to read line"); // Enum With Ok, Err
        println!("you guessed: {}", guess);

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    };
}