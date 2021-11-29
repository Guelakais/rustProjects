use std::io; //std provides us with features including the abillity to accept user input
use std::cmp::Ordering;
use rand::Rng;

#[macro_use]
extern crate fstrings;

fn main() {
    let secretNumber = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Guess the number!");
        
        let mut stGuess = String::new(); //let does create a variable stGuess, which is 'mut'able;
        //the String::new() function is here to accepts string user inputs
        io::stdin()
        .read_line(&mut stGuess)
        .expect("Failed to read line");
        //This line reads the given input, by handling Potential Failure with the .except() function
        let stGuess: u32 = match stGuess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println_f!("Please input your guess.\nYou guessed: {stGuess}");
        
        match stGuess.cmp(&secretNumber) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
