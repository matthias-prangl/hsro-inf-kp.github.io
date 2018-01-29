extern crate rand;

use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(0, 100);
    
    loop {
        println!("Your guess:");
        let mut guess = String::new();

        match stdin().read_line(&mut guess) {
            Err(_) => panic!("Failed!"),
            _ => ()
        };
        //Alternative: io::stdin().read_line(&mut guess).expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Yay! You win!");
                break;
            }
        }
    }
}
