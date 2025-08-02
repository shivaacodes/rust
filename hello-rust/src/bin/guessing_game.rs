use std::io; // std i/o library

use std::cmp::Ordering; //Less , Greater, Equal

use rand::Rng; // random number generator crate

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=10); //(start..=end)

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // reusing 'guess' variable -> shadowing
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("TOOOO BIG"),
            Ordering::Equal => {
                println!("YoU WiN!!!");
                break; //exit loop
            }
        }
    }
}
