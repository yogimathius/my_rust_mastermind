use rand::random;
use std::env;
use std::io;

pub struct Game {
    pub current_round: u32,
    pub total_rounds: u32,
    pub random_four_digits: u32,
}

impl Game {
    pub fn new() -> Game {
        let current_round = 0;
        let mut total_rounds = 10;
        let mut random_four_digits = random::<u32>() % 9000 + 1000;

        let args: Vec<String> = env::args().collect();
        if !args.is_empty() {
            (total_rounds, random_four_digits) = parse_args(args, random_four_digits);
        }
        Game {
            current_round,
            total_rounds,
            random_four_digits,
        }
    }
    pub fn set_random_four_digits(&mut self, random_four_digits: u32) {
        self.random_four_digits = random_four_digits;
    }
    pub fn play(&mut self) {
        println!("Will you find the secret code?\n");
        println!("Please enter a valid guess\n");
        let mut word_guessed = false;
        while !word_guessed && self.current_round < self.total_rounds {
            println!("---");
            println!("Round {}\n", self.current_round);

            let buf = get_from_user();
            if buf.is_empty() {
                println!("exit");
                return;
            }

            if buf.len() != 4 || buf.chars().any(|c| !c.is_digit(10)) {
                println!("Wrong input!\n");
            } else {
                let well_placed = well_placed_pieces(&buf, self.random_four_digits);
                if well_placed == 4 {
                    println!("Congratz! You did it!");
                    word_guessed = true;
                } else {
                    let misplaced = misplaced_pieces(&buf, self.random_four_digits);
                    println!("Well placed pieces: {:?}\n", well_placed);
                    println!("Misplaced pieces: {:?}\n", misplaced);
                    self.current_round += 1;
                }
            }
        }
    }
}

pub fn get_from_user() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
pub fn well_placed_pieces(buffer: &str, random_digits: u32) -> u32 {
    let mut well_placed = 0;
    let formatted_str = format!("{:04}", random_digits);
    let formatted_chars: Vec<char> = formatted_str.chars().collect();
    buffer.chars().enumerate().for_each(|(i, c)| {
        if c == formatted_chars[i] {
            well_placed += 1;
        }
    });
    return well_placed;
}

pub fn misplaced_pieces(buffer: &str, random_digits: u32) -> u32 {
    let mut misplaced = 0;
    let formatted_str = format!("{:04}", random_digits);
    let formatted_chars: Vec<char> = formatted_str.chars().collect();
    buffer.chars().enumerate().for_each(|(i, c)| {
        if c != formatted_chars[i] && formatted_chars.contains(&c) {
            misplaced += 1;
        }
    });
    return misplaced;
}

pub fn parse_args(args: Vec<String>, random_four_digits: u32) -> (u32, u32) {
    let mut random_digits = random_four_digits;
    let mut total_rounds = 10;
    let mut i = 0;
    for arg in args.clone() {
        if arg == "-c" {
            random_digits = args[i + 1].parse::<u32>().unwrap();
        }
        if arg == "-t" {
            total_rounds = args[i + 1].parse::<u32>().unwrap();
        }
        i += 1;
    }
    return (total_rounds, random_digits);
}
