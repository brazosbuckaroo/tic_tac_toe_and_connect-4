#![warn(clippy::all, clippy::pedantic)]

mod game_lib;

use std::mem;
use game_lib::{MAIN_MENU, WHICH_PLAYER, GAME_EDITOR, PLAY_AGAIN};
use game_lib::general_input::{get_int_input, get_str_input};
use game_lib::player::{List, Player, ControlMode, Sprite};
use game_lib::player::ai_engine::simple_think;
use game_lib::player_editor::player_editor;
use game_lib::game::{Game, State, MoveStatus};
use game_lib::game_editor::game_editor;
use game_lib::game_engine::{change_status, edit_board, ttt_cnct_four_board_move_chck};

const SOFTWARE_NAME: &str = env!["CARGO_PKG_NAME"];
const SOFTWARE_VER: &str = env!["CARGO_PKG_VERSION"];

fn main() {
    println!("Software Name:    {SOFTWARE_NAME}");
    println!("Software Version: v{SOFTWARE_VER}");
    println!();

    let mut user_input = get_int_input(MAIN_MENU);
    let mut player_list = List {
        player_1: Player::human(String::from("P1"), Sprite::new("X")),
        player_2: Player::ai(Sprite::new("H")),
    };
    let mut game = Game::tic_tac_toe();
 
    while user_input.is_some() {
        match user_input {
            Some(1) => {
                player_list = player_editor(WHICH_PLAYER, player_list.clone());              

                user_input = get_int_input(MAIN_MENU);
            }
            Some(2) => {
                game = game_editor(GAME_EDITOR, game.clone());

                user_input = get_int_input(MAIN_MENU);
            }
            Some(3) => {
                let mut play_again = String::from("Y");

                while matches!(play_again.as_str(), "Y") {
                    let mut current_player = &mut player_list.player_1;
                    let mut other_player = &mut player_list.player_2;

                    eprintln!("Welcome to {name}", name = game.name);

                    while matches!(game.current_state, State::NotOver) {
                        eprintln!("{game}");
                        eprintln!("Current Player: {current_player}", 
                            current_player = current_player.name);

                        let player_move = match &current_player.control {
                            ControlMode::Human => {
                                    if let Some(val) = get_int_input("Make a move: ") {
                                        val
                                    } else {
                                        eprintln!("Exiting game...");

                                        break;
                                    }
                                }
                            ControlMode::Ai => simple_think(&game.board, game.size, game.current_mode),
                        };
                    
                        match ttt_cnct_four_board_move_chck(
                            &game.board, 
                            game.size, 
                            game.current_mode, 
                            &current_player.sprite, 
                            &other_player.sprite, 
                            player_move,
                        ) {
                            MoveStatus::Valid => {
                                game.board = edit_board(
                                    game.board, 
                                    game.size, 
                                    game.current_mode, 
                                    &current_player.sprite, 
                                    &other_player.sprite, 
                                    player_move);

                                game.update_turns();
                            }
                            MoveStatus::Invalid(error) => {
                                eprintln!("{error}");

                                continue;
                            }
                        }

                        game.current_state = match change_status(&game, &current_player.sprite) {
                            State::Won => {
                                println!("Congrats {current_player} won!", 
                                    current_player = current_player.name);

                                current_player.update_wins(1);

                                State::Won
                            }
                            State::Tie => {
                                println!("It was a tie");

                                State::Tie
                            }
                            State::NotOver => {
                                mem::swap(&mut current_player, &mut other_player);
                                
                                State::NotOver
                            }
                        };
                    } 

                    println!("{game}");
                    println!("{player_list}");

                    play_again = get_str_input(PLAY_AGAIN, 1);

                    while !(matches!(play_again.as_str(), "Y" | "N")) {
                        eprintln!("Please enter a `Y` or `N`: Try again.");

                        play_again = get_str_input(PLAY_AGAIN, 1);
                    }

                    game.reset();
                }

                user_input = get_int_input(MAIN_MENU);
            }
            _ => {
                eprintln!("Invalid selection. Try again.");

                user_input = get_int_input(MAIN_MENU);
            }
        }
    }
}