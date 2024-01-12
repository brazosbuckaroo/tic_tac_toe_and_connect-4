use std::fmt;
use std::mem;
use colored::Colorize;
use crate::game_lib::players::Player;

/// An enumerator that represents whether a players move was
/// valid or not
#[derive(Debug)]
enum MoveStatus<'a> {
    /// `Valid` - If the move was Valid, then be Valid
    Valid,
    /// `Invalid` - Invalid moves return an error message and what caused the error
    Invalid(&'a str),
}

/// An enumerator that represents whether the game has
/// been Won, Tied, or needs to continue
#[derive(Debug)]
#[derive(PartialEq)]
pub enum GameStatus<'a> {
    /// `Won` - When the game is won, the player who won can be printed out
    Won(&'a Player<'a>),
    /// `Tie` - Used when the game has been tied
    Tie,
    /// `Continue` - Used when the game can continue being played
    Continue,
}

/// Represents a Tic-Tac-Toe board or Connect-4 board
/// can potentially be used with an N-sized grid
#[derive(Debug)]
pub struct GameBoard<'a> {
    /// `name` - A string literal used to represent the name of the game
    pub name: &'a str,
    /// `grid_size` - A arch-sized unsigned integer used to represent the size of the drig
    grid_size: usize,
    /// `num_of_turns` - An arch-sized unsigned integer to represent the number of turns
    num_of_turns: usize,
    /// `grid` - A vec `grid_size` * `grid_size` vec filled with empty string literals    
    grid: Vec<&'a str>,
    /// `player_1` - A reference to a player struct meant for one of the players
    pub current_player: &'a Player<'a>,
    /// `player_2` - A reference to a player struct meant for one of the players
    other_player: &'a Player<'a>,
    /// `game_status` - A GameStatus enum that keeps track of game progress
    pub game_status: GameStatus<'a>,
}

/// Game methods and functions
impl<'a,> GameBoard<'a> {
    /// Returns a game board with the specified arguments. Will panic if the board size is not 3 or 4. This
    /// will be changed in the future in version 0.2.0.
    ///
    /// # Arguments
    ///
    /// * `game_name` - A string slice that holds the name of the game being played
    /// * `board_size` - An integer that's used to make a N * N grid
    /// * `player_1` - A reference to a player
    /// * `player_2` - A reference to a player
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let player_1 = Player::new("User 1", "X");
    /// let player_2 = Player::new("User 2", "O");
    /// let test_game = GameBoard::new("Test Game", 3, &player_1, &player_2);
    ///
    /// println!("{test_game}");
    /// ```
    pub fn new(game_name: &'a str, board_size: usize, player_1: &'a Player, player_2: &'a Player) -> Self {
        if board_size != 3 && board_size != 4 {
            panic!("You gave {board_size} as a board size! This is not valid at the moment. Try either `3` or `4.`");
        }

        GameBoard {
            name: game_name,
            grid_size: board_size,
            num_of_turns: 0, 
            grid: vec![""; board_size * board_size], // we're making a square.... for now
            current_player: player_1,
            other_player: player_2,
            game_status: GameStatus::Continue,
        }
    }

    /// Edits the `GameStatus` based on the result of checking for a winner. 
    ///
    /// `GameStatus::Won(&'a Player<'a>)`  - means a player has won and passes an argument that allows
    ///                                                              the player who won to be shown
    ///
    /// `GameStatus::Tie`  - means there was no winner
    ///
    /// `GameStatus::Continue`  - Means the game can continue
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference of self to update
    /// * `selected_cell` - An usigned integer that represents a cell on the grid being played on 
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let mut test_game = GameBoard::new("Test Game", 3);
    /// let mut test_player = Player::new("Test Player", "X");
    ///
    /// test_game.play_move(3);
    ///
    /// println!("{test_game}");
    /// ```
    pub fn play_move(& mut self, selected_cell: usize) {
        // debug
        //println!("Testing value: {}", self.grid[selected_cell - 1]);
        //println!("Testing Player 1 value: {}", self.player_1.sprite);
        //println!("Testing Player 2 value: {}", self.player_2.sprite);

        // check to see if it was a valid move
        match self.valid_move(selected_cell) {
            MoveStatus::Valid => {
                // adjust logic based on board size
                match self.grid_size {
                    4 => { 
                        // connect-4 specific rules
                        let mut temp = selected_cell + (self.grid_size - 1);

                        for _cell in 0..self.grid_size {
                            if self.grid[temp].is_empty() && temp + self.grid_size < self.grid.len() {
                                temp += 4;
                            }
                        }

                        if self.grid[temp] == self.current_player.sprite || self.grid[temp] == self.other_player.sprite {
                            temp -= 4;
                        }

                        self.grid[temp] = self.current_player.sprite;
                        self.num_of_turns += 1;
                    }
                    _ => {
                        self.grid[selected_cell - 1] = self.current_player.sprite;
                        self.num_of_turns += 1;
                    }
                }

                // check to see if its worth checking if someone could win
                let check_turns: bool = self.num_of_turns >= self.grid_size + 2;

                match check_turns {
                    true => {
                        // check to see if the player won
                        if self.is_won() {
                            // debug
                            //println!("Testing logic");

                            self.game_status = GameStatus::Won(self.current_player);
                        }

                        // check to see if there was a tie
                        if self.num_of_turns == self.grid_size * self.grid_size {
                            self.game_status = GameStatus::Tie;
                        }
                    }
                    false => (), // do nothing if there aren't enough turns yet
                }

                // switch which player is playing after a successful move
                self.switch_player();

            }
            MoveStatus::Invalid(error_mssg) => {
                eprintln!("{error_mssg}", error_mssg = error_mssg.red().bold());

                self.game_status = GameStatus::Continue;
            }
        }
    }

    /// A mutator that uses a temporary storage variable to swap which player is player after a successful
    /// move.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference of self to update
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let mut player_1 = Player::new("User 1", "X");
    /// let mut player_2 = Player::new("User 2", "O");
    /// let mut test_game = GameBoard::new("Test Game", 3, &player_1, &player_2);
    ///
    /// test_game.play_move(3); // autmatically calls the switch_player function
    /// test_game.play_move(1); // autmatically calls the switch_player function
    ///
    /// println!("{test_game}");
    /// ```
    fn switch_player(& mut self) {
        mem::swap(& mut self.current_player, & mut self.other_player)
    }

    /// Returns false or a true depending on the outcome of board check
    ///
    /// `false` - means there is no winner yet
    ///
    /// `true` - means there was a winner
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to gameboard
    /// * `player` -  A reference to a player struct
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let mut player_1 = Player::new("User 1", "X");
    /// let mut player_2 = Player::new("User 2", "O");
    /// let mut test_game = GameBoard::new("Test Game", 3, &player_1, &player_2);
    ///
    /// test_game.play_move(1);
    /// test_game.play_move(4);
    /// test_game.play_move(2);
    /// test_game.play_move(5);
    /// test_game.play_move(3);
    ///
    /// if test_game.is_won() {
    ///     println!("Player 1 won!");   
    /// }
    /// ```
    fn is_won(&self) -> bool {
        // see if player won depending on the game the user selected
        match self.grid_size {
            3 => {
                // rows
                if self.grid[0] == self.current_player.sprite && self.grid[1] == self.current_player.sprite && 
                    self.grid[2]  == self.current_player.sprite 
                {
                    return true
                }
        
                if self.grid[3] == self.current_player.sprite && self.grid[4] == self.current_player.sprite && 
                    self.grid[5]  == self.current_player.sprite 
                {
                    return true
                }
        
                if self.grid[6] == self.current_player.sprite && self.grid[7] == self.current_player.sprite && 
                    self.grid[8]  == self.current_player.sprite 
                {
                    return true
                }
        
                // columns
                if self.grid[0] == self.current_player.sprite && self.grid[3]== self.current_player.sprite && 
                    self.grid[6]  == self.current_player.sprite 
                {
                    return true
                }
        
                if self.grid[1] == self.current_player.sprite && self.grid[4]== self.current_player.sprite && 
                    self.grid[7]  == self.current_player.sprite 
                {
                    return true
                }
        
                if self.grid[2] == self.current_player.sprite && self.grid[5]== self.current_player.sprite && 
                    self.grid[8]  == self.current_player.sprite 
                {
                    return true
                }
        
                // diag
                if self.grid[0] == self.current_player.sprite && self.grid[4]== self.current_player.sprite && 
                    self.grid[8]  == self.current_player.sprite 
                {
                    return true
                }
        
                // anti diag
                if self.grid[2] == self.current_player.sprite && self.grid[4]== self.current_player.sprite && 
                    self.grid[6]  == self.current_player.sprite 
                {
                    return true
                }
            }
            4 => {
                // rows
                if self.grid[0] == self.current_player.sprite && self.grid[1] == self.current_player.sprite && 
                    self.grid[2] == self.current_player.sprite && 
                    self.grid[3] == self.current_player.sprite 
                {
                    return true
                }

                if self.grid[4] == self.current_player.sprite && self.grid[5] ==self.current_player.sprite && 
                    self.grid[6] == self.current_player.sprite && 
                    self.grid[7] == self.current_player.sprite
                {
                    return true
                }

                if self.grid[8] == self.current_player.sprite && self.grid[9] == self.current_player.sprite && 
                    self.grid[10] == self.current_player.sprite && 
                    self.grid[11] == self.current_player.sprite
                {
                    return true
                }          
                
                if self.grid[12] == self.current_player.sprite && self.grid[13] == self.current_player.sprite && 
                    self.grid[14] == self.current_player.sprite && 
                    self.grid[15] == self.current_player.sprite 
                {
                    return true
                } 

                // columns
                if self.grid[0] == self.current_player.sprite && self.grid[4] == self.current_player.sprite && 
                    self.grid[8] == self.current_player.sprite && 
                    self.grid[12] == self.current_player.sprite
                {
                    return true
                }
                
                if self.grid[1] == self.current_player.sprite && self.grid[5] == self.current_player.sprite && 
                    self.grid[9] == self.current_player.sprite && 
                    self.grid[13] == self.current_player.sprite
                {
                    return true
                }      

                if self.grid[2] == self.current_player.sprite && self.grid[6] == self.current_player.sprite && 
                    self.grid[10] == self.current_player.sprite && 
                    self.grid[14] == self.current_player.sprite
                {
                    return true
                }

                if self.grid[3] == self.current_player.sprite && self.grid[7] == self.current_player.sprite && 
                    self.grid[11] == self.current_player.sprite && 
                    self.grid[15] == self.current_player.sprite 
                {
                    return true
                }            

                // diag
                if self.grid[0] == self.current_player.sprite && self.grid[5] == self.current_player.sprite && 
                    self.grid[10] == self.current_player.sprite && 
                    self.grid[15] == self.current_player.sprite 
                {
                    return true
                }       
                
                // anti diag
                if self.grid[3] == self.current_player.sprite && self.grid[6] == self.current_player.sprite && 
                    self.grid[9] == self.current_player.sprite && 
                    self.grid[12] == self.current_player.sprite
                {
                    return true
                }     
            }
            _ => panic!("Not a valid grid size: Should be either 3 or 4. You selected {user_size}.", user_size = self.grid_size),
        }

        false
    }

    /// Returns true or false depending on if the player made a valid move
    ///
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to gameboard
    /// * `selected_cell` -  A usize integer gotten from user input
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let mut player_1 = Player::new("User 1", "X");
    /// let mut player_2 = Player::new("User 2", "O");
    /// let mut test_game = GameBoard::new("Test Game", 3, &player_1, &player_2);
    ///
    /// test_game.play_move(1);
    /// test_game.play_move(4);
    /// test_game.play_move(2);
    /// test_game.play_move(5);
    /// test_game.play_move(9);
    ///
    /// test_game.play_move(5); // show an error
    /// ```
    fn valid_move(&self, selected_cell: usize) -> MoveStatus {
        // general rules for the games
        if selected_cell > self.grid_size * self.grid_size {
            return MoveStatus::Invalid("Invalid Cell because it was out of range. Please try again.")
        }

        if selected_cell == 0 {
            return MoveStatus::Invalid("Invalid Cell because it was out of range. Please try again.")
        }

        if self.grid[selected_cell - 1] == self.current_player.sprite || self.grid[selected_cell - 1] == self.other_player.sprite {
            //eprintln!("Invalid cell. Player {player_1} is already in that cell", player_1 = self.player_1.sprite);

            return MoveStatus::Invalid("Invalid cell because a player was there. Please try again.")
        }

        // connect-4 only rule
        if selected_cell > self.grid_size && self.grid_size == 4 {
            return MoveStatus::Invalid("Invalid column selected")
        }

        MoveStatus::Valid
    }
}

// the formatter trait for the game struct
impl<'a> fmt::Display for GameBoard <'a>{
    fn fmt(&self, format_buffer: & mut fmt::Formatter) -> fmt::Result {
        write!(format_buffer, " ")?;

        let sprites = &self.grid;

        for _index in 0..self.grid_size {
            write!(format_buffer, " {:-^3} ", "-")?;
        }

        write!(format_buffer, "\n ")?;

        for (index, sprite) in sprites.iter().enumerate() {
            write!(format_buffer, "|")?;
            write!(format_buffer, "{:^3}", sprite)?;    
            write!(format_buffer, "|")?;
            write!(format_buffer, "")?;

            // debug
            //print!("{:^1}", index);

            // check to see if we are at the
            // end of the row
            if  (index + 1) % self.grid_size  == 0 {
                write!(format_buffer, "\n ")?;

                // format the line breaks between
                // rows
                for _counter in 0..self.grid_size {
                    write!(format_buffer, " {:-^3} ", "-")?;
                }
                
                write!(format_buffer, "\n ")?;
            }
        }

        write!(format_buffer, "")
    }
}
