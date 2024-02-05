//! #`game_lib`
//!
//! This library is meant to provide an easy to use game engine for simple games
//! like Tic-Tac-Toe, Connect-4, Chess, and Checkers. As of right now, the only games
//! current implemented are Tic-Tac-Tie and Connect-4.
//! 

/// A simple implementation of a struct that can create any grid based game
pub mod game;
/// Provides a player struct and AI Engine
pub mod player;
/// This module is used as the Game Engine with all the game logic
pub mod game_engine;
/// This is a module that allows users to edit the Game struct. In a way, it is a 
/// program in of itself.
pub mod game_editor;
/// This module is simlar to the Game Editor module but is specifically meant for 
/// editing Player structs.
pub mod player_editor;

///
/// # All the strings for the program's output and prompts for the user.
///
pub const ESCAPE_CHAR: &str = "Q";
pub const MAIN_MENU: &str = "WHat would you like to do? \
    \n`1` for player editor \n`2` for game editor \n`3` to play game \n`q` to exit \nSelection: ";
pub const TO_MAIN: &str = "Exiting to main menu. . .";
pub const GAME_EDITOR: &str = "Would you like to edit the gamemode or board size? \
    \n`1` for Gamemode \n`2` for Boardsize \n`q` Exit \nSelection: ";
pub const GAME_MODE_SEL: &str = "Which game mode would you like to play? \
   \n`1` Tic-Tac-Toe \n`2` Connect-4 \n`3` Chess \n`4` Checkers \n`q` To keep current mode \
    \nSelection:";
pub const BOARD_SIZE_SEL: &str = "Which size board would you like to play on? \
    `q` to keep current size: ";
pub const WHICH_PLAYER: &str = "Which player do you want to edit? \
    \n`1` Player 1  \n`2` Player 2 \n`q` to exit \nSelection: ";
pub const PLAYER_ATTRIBUTE_MENU: &str = "Which player attribute do you want to edit or reset to default? \
    \n`1` Name \n`2` Type \n`3` Sprite \n`4` Reset Player \n`q` Go back to player selection \nSelection: ";
pub const PROMPT_PLAYER_NAME: &str = "Enter the players name (Type `q` to exit): ";
pub const PROMPT_PLAYER_TYPE: &str = "Select a player type: \
    \n`1` for Huamn \n`2` for Ai \n`q` to exit \nSelection: ";
pub const PROMPT_PLAYER_SPRITE: &str = "Type in the new sprite (Type `q` to exunt)`: ";
pub const PLAY_AGAIN: &str = "Would you like to play again? `y` or `n`: ";

/// This module is used to provide general input for the lib; currently, supports String
/// input and Arch-sized unsigned integer input.
pub mod general_input {
    use super::ESCAPE_CHAR;
    use std::io;

    /// Returns a string that is used for game input
    ///
    /// # Arguments
    ///
    /// * `message` - \
    ///    A string slice that can be used to tell the user what to provide
    /// * `max_character_input` - \
    ///    An integer that's used to limit the amount of characters a user input
    ///
    /// # Returns
    ///                           
    /// * `String` - \
    ///     A String made from user input that get extra character's (carriage return) trimmeed  
    ///     and made to be uppercase.
    ///
    /// # Panics
    ///
    /// Will panic if there was a error getting keyboard input
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

        while user_input.trim().chars().count() != max_character_input {
            eprint!("{message}");

            io::stdin()
                .read_line(&mut user_input)
                .expect("There was an issue getting your name");

            if user_input.trim().to_uppercase().as_str() == ESCAPE_CHAR {
                return user_input.trim().to_uppercase()
            }

            if user_input.trim().chars().count() < max_character_input || 
                user_input.trim().chars().count() > max_character_input 
            {
                eprintln!("Error: You need to enter {max_character_input} character(s)");

                user_input.clear(); // try again
            }
        }

        user_input.trim().to_uppercase()
    }

    /// Returns an usigned integer, but will panic if, for whatever reason, 
    /// there is an error in unwrapping the user's input to an unsigned integer
    ///
    /// # Arguments
    ///
    /// * `message` -  \
    ///    A string slice that can be used to tell the user what to provide
    ///
    /// # Returns
    ///                           
    /// * `Option<usize>` - \
    ///     An Option enum that will return 
    ///
    /// # Panics
    ///
    /// Will panic if there was a error getting keyboard input or unwrapping the integer
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
    pub fn get_int_input(message: &str) -> Option<usize> {
        let mut user_input = String::new();
        let mut user_int: Option<usize> = None;

        loop {
            eprint!("{message}");

            io::stdin()
                .read_line(&mut user_input)
                .expect("There was an issue getting user input.");

            if user_input.trim().to_uppercase() == ESCAPE_CHAR {
                break;
            }

            let filter = user_input.trim().parse::<usize>();

            if let Ok(val) = filter {
                user_int = Some(val);

               break;
            } else {
                eprintln!("There was an error getting your input. Try again");    
                 
                user_input.clear();
            }
        }
    
        user_int
    }
}

