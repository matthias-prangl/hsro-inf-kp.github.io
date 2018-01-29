extern crate rand;

use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn main() {
    //Generate the secret number
    let secret = rand::thread_rng().gen_range(0, 100);
    
    //TODO: Read a number from stdin

    //TODO: Parse the number from string to u32

    //TODO: Match the input number by applying cmp and output an appropiate message
}
