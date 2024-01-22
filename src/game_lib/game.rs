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

/// An enumerator that represents the rules of the current game
#[derive(Debug)]
#[derive(PartialEq)]
pub enum GameRule {
    /// `TicTacToe` -  User selection follows Tic-Tac-Toe Rules
    TicTacToe,
    /// `ConnectFour` - User selection follows Connect-4 Rules
    ConnectFour,
    /// `Invalid` - User selection was invalid
    Invalid,
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
    /// `game_rule` - An enum that tracks which rules the game currently follows
    game_rule: &'a GameRule,
}

/// Game methods and functions
impl<'a> GameBoard<'a> {
    /// Returns a game board with the specified arguments.
    ///
    /// # Arguments
    ///
    /// * `game_name` - A string slice that holds the name of the game being played
    /// * `board_size` - An integer that's used to make a N * N grid
    /// * `game_rule` - A GameRule Enum that allows players to change the behavior of player placement
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
    /// let test_game = GameBoard::new("Test Game", 3, GameRule::TicTacToe, &player_1, &player_2);
    ///
    /// println!("{test_game}");
    /// ```
    pub fn new(
        game_name: &'a str, 
        board_size: usize,
        game_rule: &'a GameRule, 
        player_1: &'a Player, 
        player_2: &'a Player,
    ) -> GameBoard<'a> {
        GameBoard {
            name: game_name,
            grid_size: board_size,
            num_of_turns: 0, 
            grid: vec![""; board_size * board_size], // we're making a square.... for now
            current_player: player_1,
            other_player: player_2,
            game_status: GameStatus::Continue,
            game_rule,
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
    /// let player_1 = Player::new("User 1", "X");
    /// let player_2 = Player::new("User 2", "O");
    /// let mut test_game = GameBoard::new("Test Game", 3, GameRule::TicTacToe, &player_1, &player_2);
    ///
    /// test_game.player_move(1);
    ///
    /// println!("{test_game}");
    /// ```
    pub fn play_move(&mut self, selected_cell: usize) {
        // debug
        //println!("Testing value: {}", self.grid[selected_cell - 1]);
        //println!("Testing Player 1 value: {}", self.player_1.sprite);
        //println!("Testing Player 2 value: {}", self.player_2.sprite);

        // check to see if it was a valid move
        match self.valid_move(selected_cell) {
            MoveStatus::Valid => {
                // adjust logic based on board size
                match self.game_rule {
                    GameRule::ConnectFour => { 
                        // connect-4 specific rules
                        let mut cell_below = selected_cell + (self.grid_size - 1);

                        for _column in 0..self.grid_size {
                            if self.grid[cell_below].is_empty() && cell_below + self.grid_size < self.grid.len() {
                                cell_below += self.grid_size;
                            }
                        }

                        // adjust to place the player above the last player
                        if self.grid[cell_below] == self.current_player.sprite || self.grid[cell_below] == self.other_player.sprite {
                            cell_below -= self.grid_size;
                        }

                        self.grid[cell_below] = self.current_player.sprite;
                        self.num_of_turns += 1;
                    }
                    GameRule::TicTacToe => {
                        self.grid[selected_cell - 1] = self.current_player.sprite;
                        self.num_of_turns += 1;
                    }
                    GameRule::Invalid => panic!("Something went wrong with game rule selection"),
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

    /// A mutator that allows the user to swap players after a move.
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
    /// let mut test_game = GameBoard::new("Test Game", 3, GameRule::TicTacToe, &player_1, &player_2);
    ///
    /// test_game.play_move(3); // autmatically calls the switch_player function
    /// test_game.play_move(1); // autmatically calls the switch_player function
    ///
    /// println!("{test_game}");
    /// ```
    fn switch_player(&mut self) {
        mem::swap(&mut self.current_player, &mut self.other_player)
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
    /// * `selected_cell` -  A reference to a player struct
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::game_lib::players::Player;
    ///
    /// let mut player_1 = Player::new("User 1", "X");
    /// let mut player_2 = Player::new("User 2", "O");
    /// let mut test_game = GameBoard::new("Test Game", 3, GameRule::TicTacToe, &player_1, &player_2);
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
        // testing new column logic
        for column in 0..self.grid_size {
            let mut cell_below: usize = column; 
            let mut player_counter: usize = 0; // reset counter

            for _cell in 0..self.grid_size {
                // debug
                //eprintln!("Testing cell after selected: {}", cell_below);

                // seeing if the player is there
                if self.grid[cell_below] == self.current_player.sprite {
                    player_counter += 1;
                }

                // debug
                //println!("Player Counter: {}", player_counter);

                // did someone win
                if player_counter == self.grid_size {
                    return true
                }

                // moving forward
                if cell_below + self.grid_size <= (self.grid_size * self.grid_size) {
                    cell_below += self.grid_size;
                }
            }
        }

        // new row logic
        for row in 0..self.grid_size {
            let mut next_cell: usize = row * self.grid_size;
            let mut player_counter: usize = 0; // reset counter

            for _cell in 0..self.grid_size {
                // compare cell
                if self.grid[next_cell] == self.current_player.sprite {
                    player_counter += 1;
                }

                // debug
                //println!("Player Counter: {} {_cell}", player_counter);

                // see if a player won
                if player_counter == self.grid_size {
                    return true
                }

                // don't increment if were at the end of a row
                if next_cell + 1 < (self.grid_size * row) + self.grid_size {
                    next_cell += 1;
                }
            }
        }

        // new diag logic
        let mut next_cell: usize = 0;
        let mut player_counter: usize = 0; // reset counter

        for offset in 0..self.grid_size {
            // debug
            //eprintln!("Testing next cell: {}", next_cell + offset);

            // compare cell
            if self.grid[next_cell + offset] == self.current_player.sprite {
                player_counter += 1;
            }

            // see if they won
            if player_counter == self.grid_size{
                return true
            }

            // next cell
            if next_cell + self.grid_size <= (self.grid_size * self.grid_size) {
                next_cell += self.grid_size;
            }
        }

        // new anti diag logic
        let mut next_cell: usize = 0;
        let mut player_counter: usize = 0; // reset counter

        for offset in (0..self.grid_size).rev() {
            // debug
            //eprintln!("Testing next cell: {}", next_cell + offset);

            // compare cell
            if self.grid[next_cell + offset] == self.current_player.sprite {
                player_counter += 1;
            }

            // see if they won
            if player_counter == self.grid_size {
                return true
            }

            // next cell
            if next_cell + self.grid_size <= (self.grid_size * self.grid_size) {
                next_cell += self.grid_size;
            }
        }

        false
    }

    /// Returns an MoveStatus enum that is either `Valid` or `Invalid` based on players move 
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
    /// let mut test_game = GameBoard::new("Test Game", 3, GameRule::TicTacToe, &player_1, &player_2);
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
        if selected_cell > self.grid_size * self.grid_size || selected_cell < 1 {
            return MoveStatus::Invalid("Invalid Cell because it was out of range. Please try again.")
        }

        if self.grid[selected_cell - 1] == self.current_player.sprite || self.grid[selected_cell - 1] == self.other_player.sprite {
            //eprintln!("Invalid cell. Player {player_1} is already in that cell", player_1 = self.player_1.sprite);

            return MoveStatus::Invalid("Invalid cell because a player was there. Please try again.")
        }

        // connect-4 only rule
        if selected_cell > self.grid_size && *self.game_rule == GameRule::ConnectFour {
            return MoveStatus::Invalid("Invalid column selected")
        }

        MoveStatus::Valid
    }
}

// the formatter trait for the game struct
impl<'a> fmt::Display for GameBoard <'a> {
    fn fmt(&self, format_buffer: &mut fmt::Formatter) -> fmt::Result {
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