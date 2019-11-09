use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut tries = 0;

    loop {
        println!("Guess a number\n");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Please input your Guess!\n");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if tries == 5 {
            println!("\nSorry, you had your chance!\n");
            break;
        }    

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                tries += 1;
                num
            },
            Err(_) => {
                tries += 1;
                continue //in the case of an invalid inout do not throw an error
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too Big!\n"),
            Ordering::Equal => {
                println!("Yes, you win!\n");
                break;
            }
        }

        println!("The secret number is: {}", secret_number);
        println!("You guessed: {}", guess);

        println!("===================================\n");
    }
}
