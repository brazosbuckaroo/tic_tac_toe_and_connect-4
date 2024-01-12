mod game_lib;

use game_lib::game::GameBoard;
use game_lib::game::GameStatus;
use game_lib::players::Player;
use colored::Colorize;
use std::io;

const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");
const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");
const INPUT_QUESTION_1: &str = 
    "Which game do you want to play? Tic-Tac-Toe or Connect-4? Enter a 1, 2, or 3 to get new player names, or 0 to exit: ";
const INPUT_QUESTION_2: &str = 
    "Would you like to play again? Enter a Y or N: ";
const PLAYER_1_INPUT: &str = 
    "Please input player 1 name: ";
const PLAYER_2_INPUT: &str = 
    "Please input player 2 name: ";

// test application
fn main() {
    println!("{first_part} {soft_name}", 
        first_part = "Software Name:".underline().bold(), 
        soft_name = SOFTWARE_NAME.italic().bold());
    println!("{first_part} {soft_ver}", 
        first_part = "Software Version:".underline().bold(), 
        soft_ver = SOFTWARE_VERSION.italic().bold());

    let mut user_input = get_int_input(INPUT_QUESTION_1);
    let mut player_1 = Player::new("".to_string(), "X");
    let mut player_2 = Player::new("".to_string(), "O");

    while user_input  != 0 {
        match user_input {
            1 => {
                if matches!(player_1.username.as_str(), "") && matches!(player_2.username.as_str(), "") {
                    let player_1_name = get_str_input(PLAYER_1_INPUT, 3);
                    let player_2_name = get_str_input(PLAYER_2_INPUT, 3);
            
                    player_1.change_username(player_1_name);
                    player_2.change_username(player_2_name);
                }

                play_game(& mut player_1, & mut player_2, "Tic-Tac-Toe", 3);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            2 => {
                if matches!(player_1.username.as_str(), "") && matches!(player_2.username.as_str(), "") {
                    let player_1_name = get_str_input(PLAYER_1_INPUT, 3);
                    let player_2_name = get_str_input(PLAYER_2_INPUT, 3);
            
                    player_1.change_username(player_1_name);
                    player_2.change_username(player_2_name);
                }

                play_game(& mut player_1, & mut player_2, "Connect-4", 4);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            3 => {
                let player_1_name = get_str_input(PLAYER_1_INPUT, 3);
                let player_2_name = get_str_input(PLAYER_2_INPUT, 3);

                player_1.clear_wins();
                player_1.change_username(player_1_name);
                player_2.clear_wins();
                player_2.change_username(player_2_name);

                eprintln!("\n{message}", message = "Success in changing the players name's".green().bold());

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            _ => {
                eprintln!("{error}", error = "Please enter a '1', '2', '3', or '0' to exit\n".red().bold());

                user_input = get_int_input(INPUT_QUESTION_1);
            }
        }
    }
}

fn play_game<'a>(player_1: &'a mut Player, player_2: &'a mut Player, game_name: &'a str, grid_size: usize) {
    println!("\nYou selected {game_name}", game_name = game_name.bold());

    let mut another_round = String::from("Y"); // assume they want to play once

    while matches!(another_round.as_str(), "Y") {
        let mut game = GameBoard::new(game_name, grid_size, player_1, player_2);

        println!("\nWelcome to {game_name}", game_name = game.name);
        println!("{game}");
        println!("\nCurrent Player: {name}", name = game.current_player.username);

        let mut selected_cell = get_int_input("Select an open cell: ");
        game.play_move(selected_cell);

        while game.game_status == GameStatus::Continue {
            eprint!("\n{game}");
            println!("\nCurrent Player: {name}", name = game.current_player.username);

            // debug
            //println!("Testing Logic: {:?}", game.game_status);

            if grid_size == 4 {
                selected_cell = get_int_input("Select a column: ");
            } else {
                selected_cell = get_int_input("Select an open cell: ");
            }

            game.play_move(selected_cell);
        }

        println!("\n{game}");

        match game.game_status {
            GameStatus::Won(player) => {
                println!("{first_part} {name} {last_part}", 
                    first_part = "Congratulation".green(), 
                    name = player.username.green().bold(),
                    last_part = "! You Won\n". green());

                if player_1.sprite == player.sprite {
                    player_1.update_wins(1);
                } else {
                    player_2.update_wins(1);
                }
            }
            GameStatus::Tie => {
                println!("{tie}", tie = "There was no winner. It was a tie".cyan());
            }
            GameStatus::Continue => continue,
        }

        println!("\n{header}", header = "Scoreboard:".underline().bold());
        println!("{player_1} | {player_2}\n");

        another_round = get_str_input(INPUT_QUESTION_2, 1);

        // filter out user input to just 
        while !(matches!(another_round.as_str(), "Y" | "N")) {
            eprintln!("{error}", error = "Please enter a 'y' or 'n'".red().bold());

            another_round = get_str_input(INPUT_QUESTION_2, 1);
        }
        
        println!("");
    }
}

fn get_str_input(message: & str, max_character_input: usize) -> String {
    let mut user_input = String::new();

    // debug
    //println!("Testing new to me methods: {}", user_input.trim().chars().count());

    while user_input.trim().chars().count() != max_character_input {
        eprint!("\n{message}");

        io::stdin()
            .read_line(& mut user_input)
            .expect("There was an issue getting your name");

        if user_input.trim().chars().count() < max_character_input || user_input.trim().chars().count() > max_character_input {
            eprint!("{first_part} {max_character_input} {last_part}", 
                first_part = "Error: You need to enter".red().bold(), 
                max_character_input = max_character_input.to_string().trim().red().bold(),
                last_part = "character(s)\n".red().bold());

            // debug
            //eprintln!("Testing new to me methods: {}", user_input.trim().chars().count());

            user_input.clear() // try again
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

    let user_input = user_input.trim().parse::<usize>().unwrap_or({
        255
    });

    user_input
}