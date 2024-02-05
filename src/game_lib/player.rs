use std::fmt;
use super::game::Mode;

/// A List struct to contain two different players
#[derive(Debug, PartialEq, Clone)]
pub struct List {
    /// `player_1` - \
    ///  A Player struct used to represent player 1
    pub player_1: Player,
    /// `player_2` - \
    ///  A Player struct used to represent player 2
    pub player_2: Player,
}

// helps format the output of the list
impl fmt::Display for List {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
        write!(format_buffer, "{player_1} | {player_2} ", 
            player_1 = self.player_1, player_2 = self.player_2)
    }
}

/// Represents a Player with a name. sprite (something to repsents a player),
/// and keeps track of how many times the player won
#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    /// `control` - \
    ///  A field meant to store how the player is controlled, whether it be by AI or a Human
    pub control: ControlMode, 
    /// `name` - \
    ///  A string to represent the name of the player
    pub name: String, 
    /// `sprite` - \
    ///   A Sprite struct meant to represent the player on the board
    pub sprite: Sprite,
    /// `wins` - \
    ///  An arch-sized unsigned integer used to keep track of player wins
    pub wins: usize,
}

impl Player {
    /// This function is used to allow forthe creation of a player with no defaults.
    ///
    /// # Arguments
    /// 
    /// * `control` - \
    ///    A `ControlMode` enumerator meant to show how the player is controlled
    /// * `name` - \
    ///    A String meant to label the player
    /// * `sprite` - \
    ///    A Sprite used as a way to represent the player on the board
    ///
    /// # Returns
    ///
    /// * `Player` -  \
    ///    A player struct constructed witht the arguments passed by the caller
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let player = Player::new(ControlMode::Human, String::from("Hughman"), Sprite::new("X"));
    ///
    /// println!("{player}");
    /// ```
    pub fn new(control: ControlMode, name: String, sprite: Sprite) -> Player {
        Player {
            control,
            name,
            sprite,
            wins: 0,
        }
    }

    /// This function alllows for a convienent way to make a Human controlled player.
    ///
    /// # Arguments
    /// 
    /// * `name` - \
    ///    A String meant to label the player
    /// * `sprite` - \
    ///    A Sprite used as a way to represent the player on the board
    ///
    /// # Returns
    ///
    /// * `Player` - \
    ///     A player struct constructed witht the arguments passed by the caller
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let player = Player::human(String::from("Hughman"), Sprite("X"));
    ///
    /// println!("{player}");
    /// ```
    #[must_use]
    pub fn human(name: String, sprite: Sprite) -> Player {
        Player {
            control: ControlMode::Human,
            name,
            sprite,
            wins: 0,
        }
    }

    /// This function alllows for a convienent way to make an Ai controlled player.
    ///
    /// # Arguments
    /// 
    /// * `name` - \
    ///    A String meant to label the player
    /// * `sprite` - \
    ///    A Sprite used as a way to represent the player on the board
    ///
    /// # Returns
    ///
    /// * `Player` - \
    ///    A player struct constructed witht the arguments passed by the caller
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let player = Player::Ai(String::from("Hal"), Sprite::new("O"));
    ///
    /// println!("{player}");
    /// ```
    #[must_use]
    pub fn ai(sprite: Sprite) -> Player {
        Player {
            control: ControlMode::Ai,
            name: String::from("HAL"),
            sprite,
            wins: 0,
        }
    }

    /// This function allows the caller toupdate a players wins
    ///
    /// # Arguments
    /// 
    /// * `self` - \
    ///    A mutable reference to a Player struct
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let mut player = Player::Ai(String::from("Hal"), Sprite::new("O"));
    ///
    /// player.update_wins(1);
    ///
    /// println!("{player}");
    /// ```
    pub fn update_wins(&mut self, update_scale: usize) {
        self.wins += update_scale;
    }

    /// This function allows the caller toupdate a players wins
    ///
    /// # Arguments
    /// 
    /// * `self` - \
    ///    A mutable reference to a Player struct
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let mut player = Player::Ai(String::from("Hal"), Sprite::new("O"));
    ///
    /// player.update_wins(1);
    ///
    /// println!("{player}");
    ///
    /// let mut player = player;
    ///
    /// player.reset();
    ///
    /// println!("{player}");
    /// ```
    pub fn reset(&mut self) {
        self.wins = 0;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
        write!(format_buffer, "{name}: {wins} wins", name = self.name, wins = self.wins)
    }
}

/// An enum used to present the two types of players available
/// `Human` and `Ai`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ControlMode {
    /// `Human` - \
    ///  Represents a Human Player
    Human,
    /// `Ai` - \
    ///  Represents a Ai player
    Ai,
}

/// This tuple struct is used as a way to represent the player and give them the ability to 
/// change their sprite, if they so wish. For now, it is jsut wrapping a String but eventually it would be
/// a JPEG or some other image
#[derive(Debug, PartialEq, Clone)]
pub struct Sprite(pub String);

impl Sprite {
    /// This function allows the creation of a sprite with a simple string literal.
    ///
    /// # Arguments
    ///
    /// * `new_sprite` - A string literal used as a way to set the player's sprite
    ///
    /// # Returns
    ///
    /// * `Sprite` - \
    ///    A tuple struct containing the character(s) that represent the player
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let sprite = Sprite::new("7UP");
    ///
    /// println!("{sprite}");
    /// ```
    pub fn new(new_sprite: &str) -> Sprite {
        Sprite(new_sprite.to_owned().to_string())
    }
}

    /// This function allows the creation of a empty sprite. Mostly for the sake of ease when filling an 
    /// empty board before a player makes a move
    ///
    /// # Returns
    ///
    /// * `Sprite` - An empty Sprite
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// let sprite = Sprite::default();
    ///
    /// println!("{sprite}"); // should be nothing
    /// ```
impl Default for Sprite {
    fn default() -> Sprite {
        Sprite(String::from(" "))
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
        write!(format_buffer, "{sprite}", sprite = self.0)
    }
}

/// This modules contines all the logic related to how ai function in the engine.
/// There is a very basic dumb `Ai` implemented until actual algorithms can be
/// implemented.
pub mod ai_engine {
    use rand::Rng;
    use rand::seq::IteratorRandom;
    use super::Sprite;
    use super::Mode;

    /// A funtion that, when called, is used by the ai engine to get valid moves
    ///
    /// # Arguments
    /// 
    /// * `game` - \
    ///    A game struct used as a way to anaylze the current state of the board
    ///    to make a valid moves list
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - \
    ///     A vector of unsigned integers that represent the moves an Ai can do 
    ///                  
    /// # Examples
    /// 
    /// Basic Usage:
    ///
    /// ```
    /// use::super::Game;
    /// use::super::Sprite;
    ///
    /// let game = Game::tic_tac_toe();
    /// let valid_moves = create_valid_moves(game);
    ///
    /// for (_, move) in valid_moves.iter().enumerte() {
    ///     eprintln!("These are the valid moves: {move}");   
    /// }
    /// ```
    fn ttt_valid_moves(board: &[Sprite]) -> Vec<usize> {
        let mut valid_moves = Vec::new();

        for (index, cell) in board.iter().enumerate() {
            if cell == &Sprite::default() {
                valid_moves.push(index + 1);
            }
        }

        valid_moves
    }

    /// A very simple function that adds very, very basic `Ai` that can pick a random space based on
    /// a generated move list
    ///
    /// # Arguments
    ///
    /// * `board` - \
    ///     A reference to the board
    /// * `size` - \
    ///     An unsigned integer used to give the rng a range
    /// * `mode` - \
    ///     Adjust the algorithm according to the mode
    ///
    /// # Returns
    ///
    ///  * `usize` - \
    ///      An arch sized unsigned integer meant as the move
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use super::Mode;
    ///
    /// let mut game = Game::tictactoe();
    /// let player_1 = Player::human(String::from("P1"), Sprite::new("X"));
    /// let player_2 = Player::ai(Sprite::new("H"));
    ///
    /// game.board = edit_board(
    ///     game.board, 
    ///     game.size, 
    ///     game.mode, 
    ///     &player_1.sprite, 
    ///     &player_2.sprite, 
    ///     1,
    /// );
    /// let ai_selection = simple_think(&game.board, board.size. game.mode);
    /// game.board = edit_board(
    ///     game.board, 
    ///     game.size, 
    ///     game.mode, 
    ///     &player_1.sprite, 
    ///     &player_2.sprite, 
    ///     ai_selection,
    /// );
    ///
    /// println!("{game}");
    /// ```
    pub fn simple_think(board: &[Sprite], size: usize, mode: Mode) -> usize {
        let mut rng = rand::thread_rng();
        let mut ai_pick: usize = 0;

        match mode {
            Mode::TicTacToe => {
                let valid_moves = ttt_valid_moves(board);
                ai_pick = *valid_moves.iter().choose(&mut rng).unwrap_or_else(| | {
                    panic!("There was an issue in the simple_think function")
                });
            }
            Mode::ConnectFour => {
                ai_pick = rng.gen_range(1..=size);
            }
            Mode::Chess => (),
            Mode::Checkers => (),
        }
                
        ai_pick
    }
}