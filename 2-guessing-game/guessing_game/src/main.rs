use std::io; // io is not in the prelude
use std::cmp::Ordering;
use rand::Rng; // Rng trait must be in scope for us to use rng methods

fn main() {
    println!("Guess the number!");

    // rand::thread_rng() gives us a RNG that is local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // variable to store user input
        let mut guess = String::new(); // bound to a new, empty instance of String

        io::stdin()
            .read_line(&mut guess) // passing a ref of guess to .read_line() 
                                // refs are immutable by default so we have to write &mut guess rather than &guess
            .expect("Failed to read line");

        // .trim() eliminates whitespace at beginning and end
        // user has to press enter to satisfy read_line(), so guess will be 8\n
        // .trim() removes that

        // .parse() can parse a variety of number types, so we explicitly tell Rust the number type by annotating the type of guess to be u32

        // this u32 annotation means Rust will infer secret_number should be a u32 as well (jeffs note: wow this is cool, even after secret_number has been defined)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // guess: 50, secret_number: 38
        // then .cmp() will return Ordering::Greater, and match gets Ordering::Greater and starts checking each arm's pattern    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
