use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello! Welcome to the guessing game.");
    println!("I have chosen a random number between 1 and 100.");
    println!("Please enter a guess.");

    let chosen = rand::thread_rng().gen_range(1, 101);
    let mut num_tries:i32 = 0;

    loop {
        num_tries += 1;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("You entered an invalid guess.");

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Please enter a valid guess!");
                continue},
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&chosen) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Perfect, you guessed it! \
                 Took you only {} attempts", num_tries);
                break;
            }
        };
    }
}