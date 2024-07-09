use rand::random;
use std::env;

mod my_rust_mastermind;

use my_rust_mastermind::{get_from_user, misplaced_pieces, parse_args, well_placed_pieces};

fn main() {
    let mut current_round = 0;
    let mut total_rounds = 10;

    let mut random_four_digits = random::<u32>() % 9000 + 1000;
    let args: Vec<String> = env::args().collect();
    println!("random_four_digits: {:?}", random_four_digits);
    if !args.is_empty() {
        (total_rounds, random_four_digits) = parse_args(args, random_four_digits);
    }

    println!("Will you find the secret code?\n");
    println!("Please enter a valid guess\n");
    let mut word_guessed = false;

    while !word_guessed && current_round < total_rounds {
        println!("---");
        println!("Round {}\n", current_round);

        let buf = get_from_user();
        if buf.is_empty() {
            println!("exit");
            return;
        }

        if buf.len() != 4 || buf.chars().any(|c| !c.is_digit(10)) {
            println!("Wrong input!\n");
        } else {
            println!("random_four_digits: {:?}", random_four_digits);
            let well_placed = well_placed_pieces(&buf, random_four_digits);
            if well_placed == 4 {
                println!("Congratz! You did it!");
                word_guessed = true;
                return;
            }
            let misplaced = misplaced_pieces(&buf, random_four_digits);
            println!("Well placed pieces: {:?}\n", well_placed);
            println!("Misplaced pieces: {:?}\n", misplaced);
            current_round += 1;
        }
    }
}
