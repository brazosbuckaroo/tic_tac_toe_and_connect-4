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