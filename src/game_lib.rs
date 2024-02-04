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
pub struct PlayerList {//! #`game_lib`
    //!
    //! This library is meant to provide an easy to use game engine for simple games
    //! like Tic-Tac-Toe, Connect-4, Chess, and Checkers. As of right now, the only games
    //! current implemented are Tic-Tac-Tie and Connect-4.
    //! 
    
    /// A simple implementation of a struct that can create any grid based game
    pub mod game;
    /// Provides a player struct and AI Engine
    pub mod player;
    
    ///
    /// # All the strings for the program's output and prompts for the user.
    ///
    pub const ESCAPE_WORD: &str = "Q";
    pub const MAIN_MENU: &str = "WHat would you like to do? \
        \n`1` for player editor \n`2` for game editor \n`3` to play game \n`0` to exit \nSelection: ";
    pub const TO_MAIN: &str = "Exiting to main menu. . .";
    pub const GAME_EDITOR: &str = "Would you like to edit the gamemode or board size? \
        \n`1` for Gamemode \n`2` for Boardsize \n`0` Exit \nSelection: ";
    pub const GAME_MODE_SEL: &str = "Which game mode would you like to play? \
       \n`1` Tic-Tac-Toe \n`2` Connect-4 \n`3` Chess \n`4` Checkers \n`0` To keep current mode \
        \nSelection:";
    pub const BOARD_SIZE_SEL: &str = "Which size board would you like to play on? `0` to keep current size: ";
    pub const WHICH_PLAYER: &str = "Which player do you want to edit? \
        \n`1` Player 1  \n`2` Player 2 \n`0` to exit \nSelection: ";
    pub const PLAYER_ATTRIBUTE_MENU: &str = "Which player attribute do you want to edit or reset to default? \
        \n`1` Name \n`2` Type \n`3` Sprite \n'4' Reset Player \n`0` Go back to player selection \nSelection: ";
    pub const PROMPT_PLAYER_NAME: &str = "Enter the players name (Type `q` to exit): ";
    pub const PROMPT_PLAYER_TYPE: &str = "Select a player type: \
        \n`1` for Huamn \n`2` for Ai \n`0` to exit \nSelection: ";
    pub const PROMPT_PLAYER_SPRITE: &str = "Type in the new sprite (Type `q` to exunt)`: ";
    pub const PLAY_AGAIN: &str = "Would you like to play again? `y` or `n`: ";
    
    /// This module is used to provide general input for the lib; currently, supports String
    /// input and Arch-sized unsigned integer input.
    pub mod general_input {
        use super::ESCAPE_WORD;
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
    
                if user_input.trim().to_uppercase().as_str() == ESCAPE_WORD {
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
                        eprintln!("There was an error getting your input. Try again");
    
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
    
    /// This is a module that allows users to edit the Game struct. In a way, it is a 
    /// program in of itself.
    pub mod game_editor {
        use super::{TO_MAIN, GAME_MODE_SEL, BOARD_SIZE_SEL};
        use super::general_input::get_int_input;
        use super::player::Sprite;
        use super::game::{Game, Mode};
    
        /// This function is used to take a Game struct and edit the fields with user
        /// selected input so they can edit the game mode and size of the board. After editing
        /// the funciton will then return the edited game.
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///      A string literal used for the user prompt
        /// * `game` - \
        ///     A Game struct that's meant to edited
        /// 
        /// # Examples
        /// 
        /// Basic Usage:
        /// 
        /// ```
        /// use super::game::{Game, Mode};
        ///
        /// let mut game = Game::tic_tac_toe();
        ///
        /// println!("{game}");
        ///
        /// game = game_editor("Editing the game: ", game.clone());
        ///
        /// println!("{game}");
        /// ```
        pub fn game_editor(message: &str, mut game: Game) -> Game {
            loop {
                match get_int_input(message) {
                    0 => {
                        eprintln!("{TO_MAIN}");
    
                        break;
                    }
                    1 => {
                        let new_mode = match get_mode(GAME_MODE_SEL) {
                            Some(new_mode) => new_mode,
                            None => game.current_mode,
                        };
                        
                        game = match new_mode {
                            Mode::TicTacToe => Game::tic_tac_toe(),
                            Mode::ConnectFour => Game::connect_four(),
                            Mode::Chess => Game::chess(),
                            Mode::Checkers => Game::checkers(),
                        };
                    }
                    2 => {
                        match game.current_mode {
                            Mode::ConnectFour | Mode::TicTacToe => {      
                                let new_size = match get_size(BOARD_SIZE_SEL) {
                                    None => game.size,
                                    Some(size) => size,
                                };
                                
                                game.size = new_size;
                                game.board = vec![Sprite::default(); game.size * game.size];
                            }
                            _ => {
                                println!("Error: You can only edit board size for `tic-tac-toe` \
                                    or `connect-4`");
                            }
                        }
                    }
                    _ => eprintln!("Invalid selection. Try again."),
                }
            }
    
            game
        }
        
        /// This function is used to display a prompt and get a game mode for the new 
        /// game. It automatically filters out incorrect output.
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///      A string literal used for the user prompt
        ///
        /// # Returns
        ///
        /// * `Some(Mode::TicTacToe)` - \
        ///      Represents Tic-Tac-Toe and sets up the game with those rules
        /// * `Some(Mode::ConnectFour)` - \
        ///      Represents Connect-4 and sets up the game with those rules
        /// * `Some(Mode::Chess)` - \
        ///      Represents Chess and sets up the game with those rules
        /// * `Some(Mode::Checkers)` - \
        ///      Represents Checkers and sets up the game with those rules
        /// * `None` - \
        ///      A value that can be used to tell the calling function
        ///      that it does not need to change
        ///
        /// # Panics
        ///
        /// Will panic if input filtering did not work properly; there should be no 
        /// reason for this to happen but it is here for safety sake.
        /// 
        /// # Examples
        /// 
        /// Basic Usage:
        /// 
        /// ```
        /// use super::game::{Mode};
        ///
        /// let mut mode = Mode::ConnectFour;
        ///
        /// println!("{mode}");
        ///
        /// mode = match get_mode("Edit the mode: ") {
        ///     Some(new_mode) => new_mode,
        ///     None => mode,
        /// };
        ///
        /// println!("{mode}");
        /// ```
        fn get_mode(message: &str) -> Option<Mode> {
            let mut user_input: Option<usize> = None;
    
            while user_input.is_none() {
                user_input = match get_int_input(message) {
                    0 => Some(0),
                    1 => Some(1),
                    2 => Some(2),
                    3 => Some(3),
                    4 => Some(4),
                    _ => {
                        eprintln!("Error: Please give valid input");
    
                        None
                    }
                }
            }
    
            match user_input {
                Some(0) => None,
                Some(1) => Some(Mode::TicTacToe),
                Some(2) => Some(Mode::ConnectFour),
                Some(3) => Some(Mode::Chess),
                Some(4) => Some(Mode:: Checkers),
                _ => panic!("Something went terribly wrong in the game editor with changing \
                            in get_mode"),
            }
        }
    
        /// This function is used to display a prompt and get a new board size for the 
        /// game. It automatically filters out incorrect output, and if the user chooses
        /// to not edit the current value, it will return `None`.
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///     A string literal used for the user prompt
        ///
        /// # Returns
        ///
        /// * `Some(int_value)` - \
        ///     An integer that's used to set the size of a the game's board
        /// * `None` - \
        ///     A value that can be used to tell the calling function that it does not 
        ///     need to change
        ///
        /// # Panics
        ///
        /// Will panic if input filtering did not work properly; there should be no 
        /// reason for this to happen but it is here for safety sake.
        /// 
        /// # Examples
        /// 
        /// Basic Usage:
        /// 
        /// ```
        /// use super::game::{Game, Mode};
        ///
        /// let mut game = Game::tic_tac_toe();
        /// let mut previous_size = 3;
        ///
        /// println!("{game}");
        ///
        /// game.size = match get_size("Edit the size: ") {
        ///     Some(new_size) => new_size,
        ///     None => previous_size,
        /// };
        ///
        /// println!("{game}");
        /// ```
        fn get_size(message: &str) -> Option<usize> {
            let mut user_input: Option<usize> = None;
    
            while user_input.is_none() {
                user_input = match get_int_input(message) {
                    0 => Some(0),
                    1 | 2 => {
                         eprintln!("Must enter a value above `3`. Try again.");
    
                        None
                    }
                    value => Some(value),
                };
            }
    
            match user_input {
                Some(0) => None,
                Some(value) => Some(value),
                None => panic!("There was an issue getting the board size in get_size."),
            }
        }
    }
    
    /// This module is simlar to the Game Editor module but is specifically meant for 
    /// editing Player structs.
    pub mod player_editor {
        use super::general_input::{get_int_input, get_str_input};
        use super::player::{List, Player, ControlMode, Sprite};
        use super::{TO_MAIN, ESCAPE_WORD, PLAYER_ATTRIBUTE_MENU, PROMPT_PLAYER_NAME,
            PROMPT_PLAYER_SPRITE, PROMPT_PLAYER_TYPE};
    
        /// This function is used to take a player list to allow user selection of
        /// a specfic player. It will then return a new version of the list with updated
        /// player(s).
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///    A string literal used to prompt the user
        /// * `player_list` - \
        ///    A List struct used to track players
        ///
        /// # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::player::{List, Player};
        ///
        /// let mut player_list = List {
        ///     player_1: Player::human(String::from("P1"), Sprite::new("X")),
        ///     player_2: Player::ai(Sprite::new("H")),
        /// };
        ///
        /// println!("{player_list}");
        ///
        /// player_list = player_editor("Select a Player to edit: ", player_list.clone());
        ///
        /// println!("{player_list}");
        /// ```
        pub fn player_editor(message: &str, mut player_list: List) -> List {
            loop {
                match get_int_input(message) {
                    0 => {
                        eprintln!("{TO_MAIN}");
    
                        break;
                    }
                    1 => {
                        let edited_player = edit_player(player_list.player_1.clone());
                        player_list.player_1 = edited_player;
                    }
                    2 => {
                        let edited_player = edit_player(player_list.player_2.clone());
                        player_list.player_2 = edited_player;
                    }
                    _ => {
                        eprintln!("Invalid selection.");
                    }
                }
                
            }
    
            player_list
        }
    
        /// This function is used to take a player struct to allow users to edit
        /// the fields of the selected player. After editing is finished, it will return 
        /// the new player to the calling code
        ///
        /// # Arguments
        ///
        /// * `player` - \
        ///   A Player struct that was selected for editing
        ///
        /// # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::player::{Player};
        ///
        /// let mut player = Player::human(Sprite::new("X"));
        ///
        /// println!("{player}");
        ///
        /// player = edit_player("Editing a player: ", player.clone());
        ///
        /// println!("{player}");
        /// ```
        fn edit_player(mut selected_player: Player) -> Player {
            loop {
                match get_int_input(PLAYER_ATTRIBUTE_MENU) {
                    0 => {
                        eprintln!("{TO_MAIN}");
    
                        break;
                    }
                    1 => {
                        let new_name = match new_name(PROMPT_PLAYER_NAME) {
                            Some(name) => name,
                            None => (*selected_player.name).to_string(),
                        };
                        selected_player.name = new_name;
                    }
                    2 => {
                        let new_type = match new_type(PROMPT_PLAYER_TYPE) {
                            Some(new_type) => new_type,
                            None => selected_player.control,
                        };
                        selected_player.control = new_type;
                    }
                    3 => {
                        let new_sprite = match new_sprite(PROMPT_PLAYER_SPRITE) {
                            Some(sprite) => sprite,
                            None => selected_player.sprite.clone(),
                        };
                        selected_player.sprite = new_sprite;
                    }
                    4 => {
                        selected_player.reset();
    
                        eprintln!("Player wins reset...");
                    }
                    _ => {
                        eprintln!("Invalid Selection. Try again.");
                    }
                }
            }
    
            selected_player
        }
    
        /// This function is used to prompt users for a new player sprite. After getting
        /// input from the user, it should return the sprite to the calling function(s).
        /// If the user inputs the `ESCAPE_WORD,` it will abort the change and leave it
        /// unchanged by returning `None`.
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///   A string literal used to get a prompt to display to thee user
        ///
        /// # Returns
        ///
        /// * `Some(Sprite(string_value))` - \
        ///      A Sprite struct that is used to represent the player on the board
        /// * `None` - \
        ///      Should tell the calling function that the user did not want to change
        ///      the current value
        ///
        /// # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::player::{Sprite};
        ///
        /// let mut sprite = Sprite::default();
        ///
        /// println!("{sprite}");
        ///
        /// sprite = new_sprite("Editing a sprite: ");
        ///
        /// println!("{sprite}");
        /// ```
        fn new_sprite(message: &str) -> Option<Sprite> {
            let user_input = get_str_input(message, 1);
    
            if user_input == ESCAPE_WORD {
                None
            } else {
                Some(Sprite(user_input))
            }
        }
    
        /// This function is used to prompt users for a new player sprite. After getting
        /// input from the user, it should return the sprite to the calling function(s). If
        /// the user provides the `ESCAPE_WORD,` the function will abort and leave the
        /// the sprite unchanged by returning `None`.
        ///
        /// # Arguments
        ///
        /// * `message` - \
        ///   A string literal used to get a prompt to display to thee user
        ///
        /// # Returns
        ///
        /// * `Some(string_value)` - \
        ///      A 3 character string that represent the player's name
        /// * `None` - \
        ///      Should tell the calling function that the user did not want to change
        ///      the current value
        ///
        /// # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::player::{Player};
        ///
        /// let mut player = Player::human(String::from("P1"), Sprite::default("X"));
        ///
        /// println!("{player}");
        ///
        /// let name = new_name("Editing the player name: ");
        /// player.name = match name {
        ///     Some(new_name) => new_name,
        ///     None => player.name,   
        /// };
        ///
        /// println!("{player}");
        /// ```
        fn new_name(message: &str) -> Option<String> {
            let user_input = get_str_input(message, 3);
    
            if user_input == ESCAPE_WORD {
                None
            } else {
                Some(user_input)
            }
        }
    
        /// This function is used to prompt users for changing their control mode. There
        /// are two control modes for the players:
        ///
        /// # Arguments
        ///
        /// `message` - A string literal used to get a prompt to display to thee user
        ///
        /// # Returns
        ///
        /// * `Some(Type::Human)` - \  
        ///     Represents Human controlled player and allows the game engine to 
        ///     pick prompt the user to make a move
        /// * `Some(Type::Ai)` - \  
        ///     Representa an Ai controlled player and allows that game engine to
        ///     pick how a move is made
        /// * `None` - \  
        ///     A value that can be used to tell the calling function that it does not 
        ///     need to change
        ///
        /// # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::player::{Type};
        ///
        /// let mut control_type = Type::Human;
        ///
        /// println!("{:?}", control_type);
        ///
        /// control_type = new_type("Editing the type: ");
        ///
        /// println!("{:?}", control_type);
        /// ```
        fn new_type(message: &str) -> Option<ControlMode> {
            let mut user_input = None;
    
            while user_input.is_none() {
                user_input = match get_int_input(message) {
                    0 => Some(0),
                    1 => Some(1),
                    2 => Some(2),
                    _ => None,
                };
            }
    
            match user_input {
                Some(0) => None,
                Some(1) => Some(ControlMode::Human),
                Some(2) => Some(ControlMode::Ai),
                _ => panic!("There was an error getting user type"),
            }
        }
    }
    
    /// This module is used as the Game Engine with all the game logic
    pub mod game_engine {
        use super::game::{Game, Mode, State, MoveStatus};
        use super::player::Sprite;
    
        /// This function is used to validate a players move based on the player's 
        /// input.
        ///
        /// # Arguments
        ///
        /// * `board` - \
        ///    A references to the game's board
        /// * `size` - \
        ///    An unsigned integer that is used to represent one axis of the board's
        ///    size. (i.e. `size` for tic-tac-toe would be 3)
        /// * `mode` - \
        ///    Represents the games current mode via an enumerator
        /// * `current_player` - \
        ///    A Sprite reference that is used to represent the current player
        /// * `other_player` - \
        ///    A Sprite reference that is used to represent the opposite player
        /// * `selection` - \
        ///    An unsigned integer recieved from the player's input
        ///
        /// # Returns
        ///
        /// * `MoveStatus::Valid` - \
        ///    An enum used to represent a valid move
        /// * `MoveStatus::Invalid(error_str)` - \
        ///    An enum used to represent an invalid move that returns an error message
        ///
        ///  # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::game::{Mode, MoveStatus};
        /// use super::player::Sprite;
        ///
        /// let mut game = Game::tictactoe();
        /// let player_1 = Player::human(String::from("P1"), Sprite::new("X"));
        /// let player_2 = Player::human(String::from("P2"), Sprite::new("O"));
        ///
        /// game = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     1,
        /// );
        /// match validate_move(
        ///     &game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_2.sprite, 
        ///     &player_1.sprite, 
        ///     1,
        /// ) {
        ///     MoveStatus::Valid => (),
        ///     MoveStatus::Invalid(err) => {
        ///         eprintln!("A player is already there,")
        ///     }
        /// }
        /// ```
        pub fn ttt_cnct_four_board_move_chck<'a>(
            board: &'a [Sprite],
            size: usize, 
            mode: Mode,
            current_player: &'a Sprite, 
            other_player: &'a Sprite, 
            selection: usize
        ) -> MoveStatus<'a> {
            if selection == 0 || selection > size * size {
                return MoveStatus::Invalid("Selected cell was out of range")
            }
    
            if board[selection - 1] == *current_player || board[selection - 1] == *other_player {
                return MoveStatus::Invalid("A player was already there")
            }
    
            if selection > size  && mode == Mode::ConnectFour {
                return MoveStatus::Invalid("Selected an invalid column")
            }
    
            MoveStatus::Valid
        }
    
        /// This function is used to take the game board, edit it accordingly and 
        /// return the edited board.
        ///
        /// # Arguments
        ///
        /// * `board` - \
        ///    The games board to use to edit
        /// * `size` - \
        ///    An unsigned integer that is used to represent one axis of the board's
        ///    size. (i.e. `size` for tic-tac-toe would be 3)
        /// * `mode` - \
        ///    Represents the games current mode via an enumerator
        /// * `current_player` - \
        ///    A Sprite reference that is used to represent the current player
        /// * `other_player` - \
        ///    A Sprite reference that is used to represent the opposite player
        /// * `selection` - \
        ///    An unsigned integer recieved from the player's input
        ///
        /// # Returns
        ///
        /// * `Vec<Sprite>` - \
        ///    An enum used to represent a valid move
        ///
        ///  # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::game::{Mode, MoveStatus};
        /// use super::player::Sprite;
        ///
        /// let mut game = Game::tictactoe();
        /// let player_1 = Player::human(String::from("P1"), Sprite::new("X"));
        ///
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     1,
        /// );
        ///
        /// println!("{game}");
        /// ```
        pub fn edit_board<'a>(
            mut board: Vec<Sprite>,
            size: usize,
            current_mode: Mode, 
            current_player: &'a Sprite, 
            other_player: &'a Sprite, 
            selected_cell: usize
        ) -> Vec<Sprite> {
            match current_mode {
                Mode::TicTacToe => {
                    board[selected_cell - 1] = current_player.clone();
                }
                Mode::ConnectFour => {
                    // connect-4 specific rules
                    let mut cell_below = selected_cell + (size - 1);
        
                    for _ in 0..size {
                        if board[cell_below] == Sprite::default() && 
                            cell_below + size < board.capacity() 
                        {
                            cell_below += size;
                        }
                    }
        
                    // adjust to place the player above the last player
                    if board[cell_below] == *current_player || board[cell_below] == *other_player {
                        cell_below -= size;
                    }
        
                    board[cell_below] = current_player.clone();            
                }
                Mode::Chess => {
                    board[selected_cell - 1] = current_player.clone();
                }
                Mode::Checkers => {
                    board[selected_cell - 1] = current_player.clone();
                }
            }
    
            board
        }
    
        /// This function is used to see if the given player has won the game
        ///
        /// # Arguments
        ///
        /// * `board` - \
        ///    The games board to use to edit
        /// * `size` - \
        ///    An unsigned integer that is used to represent one axis of the board's
        ///    size. (i.e. `size` for tic-tac-toe would be 3)
        /// * `player` - \
        ///    A Sprite reference that is used to represent the current player
        ///
        /// # Returns
        ///
        /// * `Bool` - \
        ///    A bool that when `true` means the player won or `false` if they have not won
        ///
        ///  # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::game::{Mode, MoveStatus};
        /// use super::player::Sprite;
        ///
        /// let mut game = Game::tictactoe();
        /// let player_1 = Player::human(String::from("P1"), Sprite::new("X"));
        ///
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     1,
        /// );
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     2,
        /// );
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     3,
        /// );
        ///
        /// println!("{game}");
        /// ```
        fn ttt_cnct_four_board_check(board: &[Sprite], size: usize, player: &Sprite) -> bool {
            // testing new column logic
            for column in 0..size {
                let mut cell_below: usize = column; 
                let mut player_counter: usize = 0; // reset counter
    
                for _ in 0..size {
                    // seeing if the player is there
                    if board[cell_below] == *player {
                        player_counter += 1;
                    }
    
                    // did someone win
                    if player_counter == size {
                        return true;
                    }
    
                    // moving forward
                    if cell_below + size <= (size * size) {
                        cell_below += size;
                    }
                }
            }
    
            // new row logic
            for row in 0..size {
                let mut next_cell: usize = row * size;
                let mut player_counter: usize = 0; // reset counter
    
                for _ in 0..size {
                    // compare cell
                    if board[next_cell] == *player {
                        player_counter += 1;
                    }
    
                    // see if a player won
                    if player_counter == size {
                        return true;
                    }
    
                    // don't increment if were at the end of a row
                    if next_cell + 1 < (size * row) + size {
                        next_cell += 1;
                    }
                }
            }
    
            // new diag logic
            let mut next_cell: usize = 0;
            let mut player_counter: usize = 0; // reset counter
    
            for offset in 0..size {
                // compare cell
                if board[next_cell + offset] == *player {
                    player_counter += 1;
                }
    
                // see if they won
                if player_counter == size {
                    return true;
                }
    
                // next cell
                if next_cell +  size <= (size * size) {
                    next_cell += size;
                }
            }
    
            // new anti diag logic
            let mut next_cell: usize = 0;
            let mut player_counter: usize = 0; // reset counter
    
            for offset in (0..size).rev() {
                // compare cell
                if board[next_cell + offset] == *player {
                    player_counter += 1;
                }
    
                // see if they won
                if player_counter == size {
                    return true;
                }
    
                // next cell
                if next_cell + size <= (size * size) {
                    next_cell += size;
                }
            }
    
            false
        }
    
        /// This function is used to check the game's status
        ///
        /// # Arguments
        ///
        /// * `game` - \
        ///    A reference to a Game struct
        /// * `player` - \
        ///    A reference to a Sprite struct that was used to represent the player
        ///
        /// # Returns
        ///
        /// * `State::Won` - \
        ///    Tells the game that a player has won
        /// * `State::NotOver` - \
        ///    Tella the game that the is still active
        /// * `State::Tie` - \
        ///    Tells the game that it was a tie
        ///
        ///  # Examples
        ///
        /// Basic Usage:
        ///
        /// ```
        /// use super::game::{Game, Mode, MoveStatus};
        /// use super::player::Sprite;
        ///
        /// let mut game = Game::tic_tac_toe();
        /// let player_1 = Player::human(String::from("P1"), Sprite::new("X"));
        /// let player_2 = Player::human(String::from("P2"), Sprite::new("0"));
        ///
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     1,
        /// );
        /// game.update_turns();
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     2,
        /// );
        /// game.update_turns();
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     3,
        /// );
        /// game.update_turns();
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     5,
        /// );
        /// game.update_turns();
        /// game.board = edit_board(
        ///     game.board, 
        ///     game.size, 
        ///     game.mode, 
        ///     &player_1.sprite, 
        ///     &player_2.sprite, 
        ///     6,
        /// );
        /// game.update_turns();
        ///
        /// assert_equals!(game.state, State::Won);
        /// ```
        pub fn change_status(game: &Game, player: &Sprite) -> State  {
            if game.num_of_turns >= game.size + 2 {
                if ttt_cnct_four_board_check(&game.board, game.size, player) {
                    eprintln!("fuck a duck");
     
                    return State::Won;
                } else if game.num_of_turns == game.board.capacity() {  
                    return State::Tie;
                }
            }
    
            State::NotOver 
        }
    }
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