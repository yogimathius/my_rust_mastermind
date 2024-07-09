use std::io;

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
