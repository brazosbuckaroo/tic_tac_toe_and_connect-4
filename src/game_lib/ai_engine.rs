use rand::seq::IteratorRandom;
use super::player::Sprite;
use super::game::Mode;

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
fn get_valid_moves(board: &[Sprite], size: usize, mode: Mode) -> Vec<usize> {
    let mut valid_moves = Vec::new();

    match mode {
        Mode::TicTacToe => {
            for (index, cell) in board.iter().enumerate() {
                if cell == &Sprite::default() {
                    valid_moves.push(index + 1);
                }
            }
        } 
        Mode::ConnectFour => {
            for (index, _) in board.iter().enumerate().take(size) {
                if board[index] == Sprite::default() {
                    valid_moves.push(index + 1);
                }
            }
        }
        Mode::Chess => {
            ();
        }
        Mode::Checkers => {
            ();
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

    let valid_moves = get_valid_moves(board, size, mode);
    let ai_pick = *valid_moves.iter().choose(&mut rng).unwrap_or_else(| | {
        panic!("There was an issue in the simple_think function")
    });
            
    ai_pick
}