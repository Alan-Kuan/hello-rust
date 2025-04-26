use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret = rand::random_range(1..=100);

    println!("Guess a number between 1 - 100.");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush.");

        let mut guess = String::new();
        let bytes_read = io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        if bytes_read == 0 {
            break;
        }

        // shadowing guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number.");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
