use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 - 100.");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_n) => {
                println!("You guess: {}", guess);
                let guess: u32 = match guess.trim().parse() {
                    Ok(number) => number,
                    Err(_error) => {
                        println!("Your guess is not a valid number. Please try again!");
                        continue;
                    },
                };
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    },
                }
            },
            Err(error) => println!("Failed to read: {}", error)
        }
    }
}
