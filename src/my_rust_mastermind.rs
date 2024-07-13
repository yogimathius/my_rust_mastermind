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
        let mut game = Game {
            current_round: 0,
            total_rounds: 10,
            random_four_digits: random::<u32>() % 9000 + 1000,
        };
        game.parse_args();

        game
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

            match buf.len() {
                4 if buf.chars().all(|c| c.is_digit(10)) => {
                    let well_placed =
                        check_pieces(&buf, self.random_four_digits, well_placed_condition);
                    if well_placed == 4 {
                        println!("Congratz! You did it!");
                        word_guessed = true;
                    } else {
                        let misplaced =
                            check_pieces(&buf, self.random_four_digits, misplaced_condition);
                        println!("Well placed pieces: {:?}\n", well_placed);
                        println!("Misplaced pieces: {:?}\n", misplaced);
                        self.current_round += 1;
                    }
                }
                _ => {
                    println!("Wrong input!\n");
                }
            }
        }
    }

    fn parse_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        let mut args_iter = args.iter().peekable();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-c" => {
                    if let Some(val) = args_iter.next() {
                        self.random_four_digits =
                            val.parse::<u32>().unwrap_or(self.random_four_digits);
                    }
                }
                "-t" => {
                    if let Some(val) = args_iter.next() {
                        self.total_rounds = val.parse::<u32>().unwrap_or(self.total_rounds);
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_from_user() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {
            return input.trim().to_string();
        }
        Err(err) => {
            println!("error: {}", err);
            return "".to_string();
        }
    }
}

pub fn well_placed_condition(c: char, formatted_char: &Vec<char>, i: usize) -> bool {
    c == formatted_char[i]
}

pub fn misplaced_condition(c: char, formatted_char: &Vec<char>, i: usize) -> bool {
    formatted_char.contains(&c)
        && c != formatted_char[i]
        && formatted_char.iter().filter(|&x| *x == c).count() == 1
}

pub fn check_pieces<F>(buffer: &str, random_digits: u32, condition: F) -> u32
where
    F: Fn(char, &Vec<char>, usize) -> bool,
{
    let formatted_str = format!("{:04}", random_digits);
    let formatted_chars: Vec<char> = formatted_str.chars().collect();
    buffer
        .chars()
        .enumerate()
        .filter(|&(i, c)| condition(c, &formatted_chars, i))
        .count() as u32
}
