mod game_lib;

use game_lib::game::GameBoard;
use game_lib::game::GameStatus;
use game_lib::players::Player;
use std::io;
use std::io::stdout;
use std::io::Write;

const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");
const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");
const INPUT_QUESTION_1: &str = 
    "Which game do you want to play? Tic-Tac-Toe or Connect-4? Enter a 1, 2, or 0 to exit: ";
const INPUT_QUESTION_2: &str = 
    "Would you like to play again? Enter a Y or N: ";
const PLAYER_1_INPUT: &str = 
    "Please input player 1 name: ";
const PLAYER_2_INPUT: &str = 
    "Please input player 2 name: ";

// test application
fn main() {
    println!("Software Name: {SOFTWARE_NAME}");
    println!("Software Version: {SOFTWARE_VERSION}\n");

    let mut player_1 = Player::new("".to_string(), "X");
    let mut player_2 = Player::new("".to_string(), "O");
    let mut user_input = get_int_input(INPUT_QUESTION_1);

    //play_game(& mut player_1, & mut player_2, "Tic-Tac-Toe", 5); // should crash

    while user_input  != 0 {
        match user_input {
            1 => {
                play_game(& mut player_1, & mut player_2, "Tic-Tac-Toe", 3);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            2 => {
                play_game(& mut player_1, & mut player_2, "Connect-4", 4);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            _ => {
                user_input = get_int_input(INPUT_QUESTION_1);
            }
        }
    }
}

fn play_game<'a>(player_1: &'a mut Player, player_2: &'a mut Player, game_name: &'a str, grid_size: usize) {
    println!("You selected {game_name}.");

    let player_1_name = get_str_input(PLAYER_1_INPUT, 3);
    let player_2_name = get_str_input(PLAYER_2_INPUT, 3);

    player_1.change_username(player_1_name);
    player_2.change_username(player_2_name);

    let mut another_round: String = String::new();

    while another_round.trim().to_uppercase() != "N" {
        let mut game = GameBoard::new(game_name, grid_size, player_1, player_2);

        print!("\nWelcome to {game_name}\n", game_name = game.name);
        print!("{game}");
        stdout().flush().unwrap();

        println!("{player_1} | {player_2}");
        println!("\nCurrent Player: {name}", name = game.current_player.username);

        let mut selected_cell = get_int_input("Select an open cell: ");
        game.play_move(selected_cell);

        while game.game_status == GameStatus::Continue {
            eprint!("\n{game}");
            println!("{player_1} | {player_2}");
            println!("\nCurrent Player: {name}", name = game.current_player.username);

            // debug
            //println!("Testing Logic: {:?}", game.game_status);

            selected_cell = get_int_input("Select an open cell: ");
            game.play_move(selected_cell);
        }

        println!("\n{game}");
        println!("{player_1} | {player_2}");

        match game.game_status {
            GameStatus::Won(player) => {
                println!("\nCongratulations {name}! You Won!", name = player.username);

                if player_1.sprite == player.sprite {
                    player_1.update_wins(1);
                } else {
                    player_2.update_wins(1);
                }
            }
            GameStatus::Tie => {
                println!("\nThere was no winner. It was a tie");
            }
            GameStatus::Continue => continue,
        }

        another_round = get_str_input(INPUT_QUESTION_2, 1);
    }

    println!("\nScoreboard: ");
    println!("{player_1} | {player_2}\n");
}

fn get_str_input(message: & str, max_character_input: usize) -> String {
    //println!("{message}");

    let mut user_input = String::new();

    // debug
    //println!("Testing new to me methods: {}", user_input.trim().chars().count());

    while user_input.trim().chars().count() != max_character_input {
        eprint!("\n{message}");

        io::stdin()
            .read_line(& mut user_input)
            .expect("There was an issue getting your name");

        if user_input.trim().chars().count() < max_character_input || user_input.trim().chars().count() > max_character_input {
            eprintln!("Error: You need to enter at least {max_character_input} characters.");

            // debug
            //eprintln!("Testing new to me methods: {}", user_input.trim().chars().count());

            user_input = String::new(); // try again
        }
    }

    // debug
    //println!("Testing new to me methods: {}", user_input.trim().chars().count());

    user_input.trim().to_string().to_uppercase()
}

fn get_int_input(message: & str) -> usize {
    eprint!("{message}");

    let mut user_input = String::new();
    
    io::stdin()
        .read_line(& mut user_input)
        .expect("There was an issue getting user input.");

    let user_input = match user_input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(error) => {
            eprintln!("Error: {error}.\n");

            255
        }
    };

    user_input
}