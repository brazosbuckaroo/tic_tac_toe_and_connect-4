mod game_lib;

use game_lib::game::GameBoard;
use game_lib::game::GameStatus;
use game_lib::game::GameMode;
use game_lib::players::Player;
use game_lib::game_input::*;
use game_lib::general_input::*;
use game_lib::PlayerList;
use colored::Colorize;

const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");
const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");
const INPUT_QUESTION_1: &str = 
    "Which game do you want to play? Tic-Tac-Toe or Connect-4? \
    Enter a `1`, `2`, `3` to get an 'N' sized board, `4` to get new player names, `5` to change player sprite, or `0` to exit: ";
const INPUT_QUESTION_2: &str = 
    "Would you like to play again? Enter a `Y` or `N`: ";
const INPUT_QUESTION_3: &str = 
    "What sized board do you want to play on?  ";
const INPUT_QUESTION_4: &str = 
    "What game mode would you like to play? `1` for Tic-Tac-Toe or `2` for Connect-4. ";
const INPUT_QUESTION_5: &str = 
    "Which player do you want to edit? `1`, `2`, or `0` to exit to main menu: ";
const PLAYER_1_INPUT: &str = 
    "Please input player 1 name: ";
const PLAYER_2_INPUT: &str = 
    "Please input player 2 name: ";
const PLAYER_NAME_INPUT: &str = 
    "Please input player name: ";
const PLAYER_INPUT: &str = 
    "Please input player sprite: ";

// test application
fn main() {
    println!("{first_part} {soft_name}", 
        first_part = "Software Name:".underline().bold(), 
        soft_name = SOFTWARE_NAME.italic().bold());
    println!("{first_part} {soft_ver}", 
        first_part = "Software Version:".underline().bold(), 
        soft_ver = SOFTWARE_VERSION.italic().bold());
    println!();

    let mut user_input = get_int_input(INPUT_QUESTION_1);
    let mut player_list = PlayerList {
        player_1: Player::new("".to_string(), "X".to_string()), 
        player_2: Player::new("".to_string(), "O".to_string()),
    };

    while !(matches!(user_input, 0)) {
        match user_input {
            1 => {
                player_list.player_1 = check_player_name(player_list.player_1, PLAYER_1_INPUT);
                player_list.player_2 = check_player_name(player_list.player_2, PLAYER_2_INPUT);

                play_game(&mut player_list.player_1, &mut player_list.player_2, "Tic-Tac-Toe", 3, &GameMode::TicTacToe);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            2 => {
                player_list.player_1 = check_player_name(player_list.player_1, PLAYER_1_INPUT);
                player_list.player_2 = check_player_name(player_list.player_2, PLAYER_2_INPUT);

                play_game(&mut player_list.player_1, &mut player_list.player_2, "Connect-4", 4, &GameMode::ConnectFour);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            3 => {
                player_list.player_1 = check_player_name(player_list.player_1, PLAYER_1_INPUT);
                player_list.player_2 = check_player_name(player_list.player_2, PLAYER_2_INPUT);

                println!();

                let board_size = get_int_input(INPUT_QUESTION_3);

                println!();

                let game_rule = get_mode_input(INPUT_QUESTION_4);
                
                play_game(&mut player_list.player_1, &mut player_list.player_2, "Tic-Tac-Toe Augmented", board_size, &game_rule);

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            4 => {
                println!();

                match select_player(INPUT_QUESTION_5, &mut player_list) {
                    Some(player) => {
                        player.change_username(get_str_input(PLAYER_NAME_INPUT, 3));

                        eprintln!("{mssg}", mssg = "Changing player name was successful.".green().bold());
                    }
                    None => eprintln!("{mssg}", mssg = "Exiting to main menu...".bold()),
                }

                println!();

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            5 => {
                println!();

                match select_player(INPUT_QUESTION_5, &mut player_list) {
                    Some(player) => {
                        player.change_sprite(get_str_input(PLAYER_INPUT, 1));

                        eprintln!("{mssg}", mssg = "Changing player sprite was successful.".green().bold());
                    }
                    None => eprintln!("{mssg}", mssg = "Exiting to main menu...".bold()),
                }

                println!();

                user_input = get_int_input(INPUT_QUESTION_1);
            }
            _ => {
                eprintln!("{error}", error = "Please enter a '1', '2', '3', `4`, or '0' to exit\n".red().bold());

                user_input = get_int_input(INPUT_QUESTION_1);
            }
        }
    }
}

fn play_game(
    player_1: &mut Player, 
    player_2: &mut Player, 
    game_name: & str, 
    grid_size: usize,
    game_mode: &GameMode,
) {
    println!("\nYou selected {game_name}", game_name = game_name.bold());

    let mut another_round = String::from("Y"); // assume they want to play once

    while matches!(another_round.as_str(), "Y") {
        let mut game = GameBoard::new(game_name, grid_size, game_mode, player_1, player_2); // creates a new board

        println!("\nWelcome to {game_name}", game_name = game.name);

        while matches!(game.game_status, GameStatus::Continue) {
            eprint!("\n{game}");
            println!("\nCurrent Player: {name}", name = game.current_player.username);

            // debug
            //println!("Testing Logic: {:?}", game.game_status);

            let selected_cell: usize = if *game_mode == GameMode::ConnectFour {
                    get_int_input("Select a column: ")
                } else {
                    get_int_input("Select an open cell: ")
            };

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
        
        println!();
    }
}