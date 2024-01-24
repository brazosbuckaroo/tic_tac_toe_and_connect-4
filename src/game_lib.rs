//! #`game_lib`
//!
//! This game library aims to provide an easy to use library to make a simple command line based
//! game of Tic-Tac-Toe and Connect-4. The end goal for this is to start including more functions,
//! structs, and enums to make it more powerful of a game engine.

pub mod game;
pub mod players;

use super::Player;

/// Represents a list of players in their game
#[derive(Debug)]
pub struct PlayerList {
    /// `player_1` - A Player struct used as player 1 for the game
    pub player_1: Player, 
    /// `player_2` - A player struct used a player 2 for the game
    pub player_2: Player,
}

pub mod game_input {
    use super::general_input::*;
    use super::game::GameMode;
    use super::players::Player;
    use super::PlayerList;
    use colored::Colorize;

    /// Takes a player as an argument to see if they have a name and returns the player
    ///
    /// # Arguments
    ///
    /// * `player` - A player struct who may or may not need a name
    /// * `message` - A string literal used as the prompt for the user  
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::players::Player;
    /// use colored::Colorize;
    /// use std::io;
    ///
    /// let player = Player::new("", "X");
    ///
    /// player = check_player_name(player, "Please input a name: ");
    ///
    /// println!("{player}");
    /// ```
    pub fn check_player_name(mut player : Player, message: &str) -> Player {
        if matches!(player.username.as_str(), "") {
            let player_name = get_str_input(message, 3);

            player.change_username(player_name);
        }

        player
    }

    /// Returns a GameMode enum based on user input
    ///
    /// # Arguments
    ///
    /// * `message` - A string literal used as the prompt for the user
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::players::Player;
    /// use super::game::GameBoard;
    /// use super::game::GameMode;
    ///
    /// let player_1 = Player::new("User 1", "X");
    /// let player_2 = Player::new("User 2", "O");
    /// let game_mode = get_mode_input("Please selected the game mode: `1` for Tic-Tac-Toe, `2` for Connect-4: ");
    ///
    /// let test_game = GameBoard::new("Test Game", 3, &game_mode, &player_1, &player_2);
    ///
    /// println!("{test_game}");
    /// ```
    pub fn get_mode_input(message: &str) -> GameMode {
        let mut mode = GameMode::Invalid;

        while matches!(mode, GameMode::Invalid) {
            mode = match get_int_input(message) {      
                1 => GameMode::TicTacToe,
                2 => GameMode::ConnectFour,
                _ => {
                    eprintln!("{error}",  error = "You selected an invalid game mode. Please try again".red().bold());

                    GameMode::Invalid
                }
            };
        }
    
        mode
    }

    /// Returns a player struct based on user selection form the player list
    ///
    /// # Arguments
    ///
    /// * `message` - A string literal used as the prompt for the user
    /// * `player_list` - A PlayerList struct used to keep track of the player
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::players::Player;
    ///
    /// let player_1 = Player::new("User 1", "X");
    /// let player_2 = Player::new("User 2", "O");
    /// let selected_player = select_player("Please selected a player: `1` for Player 1, `2` for Player 2: ");
    ///
    /// println!("{selected_player}");
    /// ```
    pub fn select_player<'a>(message: &'a str, player_list: &'a mut PlayerList) -> Option<&'a mut Player> {
        let mut selection = None;

        while selection.is_none() {        
            selection = match get_int_input(message) {
                1 => Some(1),
                2 => Some(2),
                0 => Some(0),
                _ => {
                    eprintln!("{error_mssg}", error_mssg = "You selected an invalid player. Try again.".bold().red());

                    None
                }
            };
        }

        match selection {
            Some(1) => Some(&mut player_list.player_1),
            Some(2) => Some(&mut player_list.player_2),
            Some(0) => None,
            _ => panic!("Something went wrong with player selection."), // safety really should never get here
        }
    }
}

pub mod general_input {
    use colored::Colorize;
    use std::io;

    /// Returns a string that is used for game input
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice that can be used to tell the user what to provide
    /// * `max_character_input` - An integer that's used to limit the amount of characters a user input
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let user_input = get_str_input("Please type `Hi`: ", 2);
    ///
    /// println!("{user_input}");
    /// ```
    pub fn get_str_input(message: &str, max_character_input: usize) -> String {
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

    /// Returns an usigned integer, but will panic if, for whatever reason, there is an error in unwrapping 
    /// the user's input to an unsigned integer
    ///
    /// # Arguments
    ///
    /// * `message` -  A string slice that can be used to tell the user what to provide
    ///
    /// # Examples 
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let user_int = get_int_input("Please type an integer: ");
    ///
    /// println!("{user_int}");
    /// ```
    pub fn get_int_input(message: &str) -> usize {
        let mut user_input = String::new();
        let mut user_int = user_input.trim().parse::<usize>();

        while user_int.is_err() {
            eprint!("{message}");

            io::stdin()
                .read_line(&mut user_input)
                .expect("There was an issue getting user input.");

            user_int = user_input.trim().parse::<usize>();
            user_int = match user_int {
                Ok(value) => Ok(value),
                Err(error) => {
                    eprintln!("{error_mssg}", error_mssg = "There was an error getting your input. Try again".bold().red());

                    user_input.clear();

                    Err(error)
                }
            };
        }

        user_int.unwrap_or_else(|err| {
            panic!("Something went really wrong. \nThe following is the error message: {err}");
        })
    }
}