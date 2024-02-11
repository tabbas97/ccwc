use std::io::{self, IsTerminal};
use std::io::prelude::*;
use std::fs::File;

fn count_bytes<

fn main() {
    // Check if stdin is empty
    let mut stdin = io::stdin();
    let mut line = String::new();
    let is_empty = stdin.read_line(&mut line).unwrap() == 0;
    
    if is_empty {
        println!("stdin is empty");
    } else {
        
    }

    // let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("{}", io::stdin().is_terminal());

    // println!("You guessed: {}", guess);
}
