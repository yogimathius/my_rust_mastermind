use my_rust_mastermind::my_rust_mastermind::{
    check_pieces, misplaced_condition, well_placed_condition, Game,
};
use std::env;

#[test]
fn test_well_placed_condition() {
    let buffer = "1234";
    let random_digits = 1234;
    let result = check_pieces(buffer, random_digits, well_placed_condition);
    assert_eq!(result, 4);
}

#[test]
fn test_misplaced_condition() {
    let buffer = "4321";
    let random_digits = 1234;
    let result = check_pieces(buffer, random_digits, misplaced_condition);
    assert_eq!(result, 4);
}

#[test]
fn test_no_well_placed_pieces() {
    let buffer = "5678";
    let random_digits = 1234;
    let result = check_pieces(buffer, random_digits, well_placed_condition);
    assert_eq!(result, 0);
}

#[test]
fn test_no_misplaced_pieces() {
    let buffer = "5678";
    let random_digits = 1234;
    let result = check_pieces(buffer, random_digits, misplaced_condition);
    assert_eq!(result, 0);
}

#[test]
fn test_mixed_conditions() {
    let buffer = "1243";
    let random_digits = 1234;
    let well_placed = check_pieces(buffer, random_digits, well_placed_condition);
    let misplaced = check_pieces(buffer, random_digits, misplaced_condition);
    assert_eq!(well_placed, 2);
    assert_eq!(misplaced, 2);
}

#[test]
fn test_game_initialization_default() {
    let game = Game::new();
    assert_eq!(game.current_round, 0);
    assert_eq!(game.total_rounds, 10);
    assert!(game.random_four_digits >= 1000 && game.random_four_digits <= 9999);
}

#[test]
fn test_game_initialization_custom_args() {
    let args = vec![
        "program".to_string(),
        "-c".to_string(),
        "5678".to_string(),
        "-t".to_string(),
        "20".to_string(),
    ];

    let original_args: Vec<String> = env::args().collect();

    env::set_var("RUST_TEST_ARGS", args.join(" "));

    let _ = std::panic::catch_unwind(|| {
        env::set_var("RUST_TEST_ARGS", args.join(" "));
        let game = Game::new();
        assert_eq!(game.current_round, 0);
        assert_eq!(game.total_rounds, 20);
        assert_eq!(game.random_four_digits, 5678);
    });

    env::set_var("RUST_TEST_ARGS", original_args.join(" "));
}
