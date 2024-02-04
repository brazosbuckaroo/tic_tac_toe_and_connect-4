use std::fmt;
use super::player::Sprite;

/// An enumerator used to keep track the state of the Game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    /// `NotOver` - \
    ///  This variant is ussed to keep the game going
    NotOver,
    /// `Won` - \
    ///  Used to end the game
    Won,
    /// `Tie` - \
    ///  Used to report a tie in the game
    Tie,
}

/// An enumerator used to track which mode the game should be in
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    /// `TicTacToe` - \
    ///  Affects the game rules and size to represent Tic-Tac-Toe
    TicTacToe,
    /// `ConnectFour` - \
    ///  Changes the game to follow the Connect-4 rules and size
    ConnectFour,
    /// `Chess` - \
    ///  Changes the game to follw Chess rules and board size
    Chess,
    /// `Checker` - \
    ///  Changes that game to follow the rules of Checkers and \
    ///  the board size
    Checkers,
}

// Just a println formatter for the modes
impl fmt::Display for Mode {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::TicTacToe => {
                write!(format_buffer, "Tic-Tac-Toe")
            }
            Mode::ConnectFour => {
                write!(format_buffer, "Connect-4")
            }
            Mode::Chess => {
                write!(format_buffer, "Chess")
            }
            Mode::Checkers => {
                write!(format_buffer, "Checkers")
            }
        }
    }
}

/// An enumerator that is used to report the validity of the player's name
#[derive(Debug, PartialEq, Clone)]
pub enum MoveStatus<'a> {
    /// `Valid` - \
    ///  Represents that the players move was valid after checking it
    Valid,
    /// `Invalid(str_value)` - \
    ///  Represents reports the move as invalid and returns a string literal \
    ///  that provides and error
    Invalid(&'a str),
}

/// A struct used to represent a Game and its related data
#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    /// `name` - \
    ///  A String that represents the name of the game
    pub name: String,
    /// `current_moce` - \
    ///  A Mode enum used to affect the gameplay
    pub current_mode: Mode,
    /// `current_state` - \
    ///  An enum used to track the state of the game
    pub current_state: State,
    /// `board` - \
    ///  A Vector of Sprites being used as the board
    pub board: Vec<Sprite>,
    /// `size` - \
    ///  Used to track one axis of the board
    pub size: usize,
    /// `num_of_turns` - \
    ///  Keeps track of the number of turns 
    pub num_of_turns: usize,
}


impl Game {
    /// This is a constuctor for Game struct that is here for idiomatic sake
    /// it can be used but there are other constructors that are far more helpful in
    /// in the current version
    ///
    /// # Argruments
    ///
    /// * `name` - \
    ///    A String where the name can be set
    /// * `new_mode` - \
    ///    A enum that allows easy setting of the game mode
    /// * `size` - \
    ///    An unsigned integer used to set the size of the board
    /// 
    /// # Returns
    ///
    /// * `Game` - \
    ///    A Game struct that could have custom behavior
    ///
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// let game = Game::new(String::from("Test Game"), Mode::ConnectFour, 5);
    ///
    /// println!("{game}");
    /// ```
    pub fn new(name: String, new_mode: Mode, size: usize) -> Game {
        Game {
            name,
            current_mode: new_mode,
            current_state: State::NotOver,
            board: vec![Sprite::default(); size * size],
            size,
            num_of_turns: 0,
        }
    }

    /// This constructor is essentially a `default` state that allows easy 
    /// creation of a tic-tac-toe board that can be played as tic-tac-toe in the
    /// main application.
    /// 
    /// # Returns
    ///
    /// * `Game` - \
    ///    A Game struct with Tic-Tac-Toe specs
    ///
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// let game = Game::tic_tac_toe();
    ///
    /// println!("{game}");
    /// ```
    #[must_use] 
    pub fn tic_tac_toe() -> Game {
        Game {
            name: String::from("Tic-Tac-Toe"),
            current_mode: Mode::TicTacToe,
            current_state: State::NotOver,
            board: vec![Sprite::default(); 9],
            size: 3,
            num_of_turns: 0,
        }
    }
    
    /// This constructor is essentially a `default` state that allows easy 
    /// creation of a Connect-4 board that can be played as Connect-4 in the
    /// main application.
    /// 
    /// # Returns
    ///
    /// * `Game` - \
    ///    A Game struct with Connect-4 specs
    ///
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// let game = Game::connect_four();
    ///
    /// println!("{game}");
    /// ```
    #[must_use] 
    pub fn connect_four() -> Game {
        Game {
            name: String::from("Connect-4"),
            current_mode: Mode::ConnectFour,
            current_state: State::NotOver,
            board: vec![Sprite::default(); 16],
            size: 4,
            num_of_turns: 0,
        }
    }    

    /// This constructor was is essentially a `default` state that allows easy 
    /// creation of a Chess board that can be played as Chess in the
    /// main application.
    /// 
    /// # Returns
    ///
    /// * `Game` - \
    ///    A Game struct with Chess specs
    ///
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// let game = Game::chess();
    ///
    /// println!("{game}");
    /// ```
    #[must_use] 
    pub fn chess() -> Game {
        Game {
            name: String::from("Chess"),
            current_mode: Mode::Chess,
            current_state: State::NotOver,
            board: vec![Sprite::default(); 64],
            size: 8,
            num_of_turns: 0,
        }
    }  

    /// This constructor was is essentially a `default` state that allows easy 
    /// creation of a Checkers board that can be played as Checkers in the
    /// main application.
    /// 
    /// # Returns
    ///
    /// * `Game` - \
    ///    A Game struct with Checkers specs
    ///
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// let game = Game::checkers();
    ///
    /// println!("{game}");
    /// ```
    #[must_use] 
    pub fn checkers() -> Game {
        Game {
            name: String::from("Checkers"),
            current_mode: Mode::Checkers,
            current_state: State::NotOver,
            board: vec![Sprite::default(); 64],
            size: 8,
            num_of_turns: 0,
        }
    }

    /// This function takes the game struct and completely resets the board according to the mode it
    /// is in
    ///
    /// # Arguments
    ///
    /// * `self` - \
    ///     The struct takes a mutable reference to itself
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::game::{Game, Mode, MoveStatus};
    /// use super::player::{Player, Sprite};
    /// use game_lib::game_engine::edit_board;
    ///
    /// let mut game = Game::tic_tac_toe();
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
    /// game.reset(); // by commenting this line out, it SHOULD edit the board
    /// 
    /// println!("{game}");
    /// ```
    pub fn reset(&mut self) {
        self.board = vec![Sprite::default(); self.size * self.size];
        self.num_of_turns = 0;
        self.current_state = State::NotOver;
    }

    /// This function takes the game struct and completely resets the board according to the mode it
    /// is in
    ///
    /// # Arguments
    ///
    /// * `self` - \
    ///     The struct takes a mutable reference to itself
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::game::{Game, Mode, MoveStatus};
    /// use super::player::{Player, Sprite};
    /// use game_lib::game_engine::edit_board;
    ///
    /// let mut game = Game::tic_tac_toe();
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
    /// 
    /// println!("{game}");
    /// ```
    pub fn update_turns(&mut self) {
        self.num_of_turns += 1;
    }
}

// the formatter trait for the game struct
impl fmt::Display for Game {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
        writeln!(format_buffer)?;
        write!(format_buffer, " ")?;

        let sprites = &self.board;

        for _ in 0..self.size {
            write!(format_buffer, " {:-^3} ", "-")?;
        }

        write!(format_buffer, "\n ")?;

        for (index, sprite) in sprites.iter().enumerate() {
            write!(format_buffer, "|")?;
            write!(format_buffer, " {sprite:^3} ")?;    
            write!(format_buffer, "|")?;
            write!(format_buffer, "")?;

            // check to see if we are at the
            // end of the row
            if  (index + 1) % self.size == 0 {
                write!(format_buffer, "\n ")?;

                // format the line breaks between
                // rows
                for _ in 0..self.size {
                    write!(format_buffer, " {:-^3} ", "-")?;
                }
                
                write!(format_buffer, "\n ")?;
            }
        }

        write!(format_buffer, "")
    }
}