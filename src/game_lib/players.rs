use std::fmt;

/// Represents a Player a name. sprite (something to repsents a player),
/// and keeps track of how many times the player won
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Player<'a> {
    /// `username` - A string literal to represent the name of the player
    pub username: String, 
    /// `sprite` - A string literal used to represent the player on grid
    pub sprite: &'a str,
    /// `total_wins` - An arch-sized unsigned integer used to keep track of player wins
    pub total_wins: usize,
}

/// Player methods and functions
impl<'a> Player<'a> {
    /// Returns a player with the name given them and a symbol used to 
    /// represent them on the game board. Players are also able to track 
    /// the amount of wins they have
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice that holds the name of the person
    /// * `user_sprite` -  A string literal used to represent the player on the board
    ///
    /// # Examples
    ///
    /// ```
    /// let test_player = Player::new("User 1", "X");
    ///
    /// println!("Test Player: \n {test_player}");
    /// ```
    pub fn new(username: String, user_sprite: &'a str) -> Self {
        Player {
            username,
            sprite: user_sprite,
            total_wins: 0,
        }
    }

    /// Takes an unsigned integer to update a players score by a specified integer.
    ///
    /// # Arguments
    ///
    /// * `update_scale` - A unsigned integer used to update a player's score
    ///
    /// # Examples
    ///
    /// ``` 
    /// use crate::game_lib::game::GameBoard;
    /// use crate::game_lib::game::GameStatus;
    ///
    /// let mut test_player_1 = Player::new("User 1", "X");
    /// let mut test_player_2 = Player::new("User 2", "O");
    /// let mut test_board = Gameboard::new("Test Game", 3, &test_player_1, &test_player_2);
    ///
    /// test_board.play_move(1);
    /// test_board.play_move(4);
    /// test_board.play_move(2);
    /// test_board.play_move(9);
    /// test_board.play_move(3);
    ///
    /// match test_board.game_status {
    ///     GameStatus::Win(player) => {
    ///             if player.sprite == test_player_1.sprite {
    ///                 test_player_1.update_wins(1);
    ///                 println!("{player} won!", player = test_player_1.username);   
    ///             } else {
    ///                 test_player_2.update_wins(1);
    ///                 println!("{player} won!", player = test_player_2.username);   
    ///             }
    ///         }
    ///     GameStatus::Tie => (),
    ///     GameStatus::Continue => (),
    ///     }
    /// ```
    pub fn update_wins(&mut self, update_scale: usize) {
        self.total_wins += update_scale
    }

    /// Edits the username of the player
    ///
    /// # Arguments
    ///
    /// * `updated_username` - A string slice that holds the new username
    ///
    /// # Examples
    ///
    /// ```
    /// let mut test_player = Player::new("User 1", "X");
    ///
    /// println!("{test_player}");
    ///
    /// test_player.update_username("New Username");
    ///
    /// println!("{test_player}");
    /// ```
    pub fn change_username(&mut self, updated_username: String) {
        self.username = updated_username
    }
}

impl<'b> fmt::Display for Player<'b> {
    // just print out the players name
    fn fmt(&self, format_buffer: & mut fmt::Formatter) -> fmt::Result {
        write!(format_buffer, "{username}: {wins} win(s)", username = self.username, wins =self.total_wins)
    }
}
