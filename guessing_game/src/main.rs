//  imports
use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // println! is macro , ! denotes it's macro

    let secret_number = rand::thread_rng().gen_range(1..101); // creating random number using rand  external crates

    println!("The secret number is: {}", secret_number);

    loop { // more like while(true) 
        println!("Please input your guess.");

        let mut guess = String::new(); // mut symbolize is mutable, default is immutable
        io::stdin()
            .read_line(&mut guess) // & is reference , like c and reference also immutable
            .expect("Failed to read line"); // getting input with fail exception

        let guess: u32 = match guess.trim().parse() { // creating same name ?? rust allows shadowing.
            Ok(num)=> num,
            Err(_)=> continue,
        }; 

        println!("You guessed: {}", guess); // {} is a placeholder , like c %s , %d

        match guess.cmp(&secret_number) {
            // match is more like switch??
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            },

        }
    }
}
