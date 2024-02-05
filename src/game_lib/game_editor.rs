use super::cmdln_interface::{TO_MAIN, GAME_MODE_SEL, BOARD_SIZE_SEL};
use super::cmdln_interface::get_int_input;
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
            None => {
                eprintln!("{TO_MAIN}");

                break;
            }
            Some(1) => {
                if let Some(new_mode) = get_new_mode(GAME_MODE_SEL) {
                    match new_mode {
                        Mode::TicTacToe => {
                            game.current_mode = Mode::TicTacToe;
                        }
                        Mode::ConnectFour => {
                            game.current_mode = Mode::ConnectFour;
                        }
                        Mode::Chess => {
                            game = Game::chess();
                        }
                        Mode::Checkers => {
                            game = Game::checkers();
                        }
                    }
                }
            }
            Some(2) => {
                match game.current_mode {
                    Mode::ConnectFour | Mode::TicTacToe => {      
                        if let Some(new_size) = get_new_size(BOARD_SIZE_SEL) {
                            game.size = new_size;
                            game.board = vec![Sprite::default(); game.size * game.size];
                        }
                    }
                    _ => {
                        println!("Error: You can only edit board size for `tic-tac-toe` or `connect-4`");
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
/// mode = match get_new_mode("Edit the mode: ") {
///     Some(new_mode) => new_mode,
///     None => mode,
/// };
///
/// println!("{mode}");
/// ```
fn get_new_mode(message: &str) -> Option<Mode> {
    let mut new_mode: Option<usize> = None;

    loop {
        if let Some(val) = get_int_input(message) {
            if val == 0 || val > 4 {
                eprintln!("invalid selection.");

                continue;
            }

            new_mode = Some(val);

            break;
        } else {
            eprintln!("Keeping the current mode...");

            break;
        }
    }

    match new_mode {
        None => None,
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
/// game.size = match new_sprite("Edit the size: ") {
///     Some(new_size) => new_size,
///     None => previous_size,
/// };
///
/// println!("{game}");
/// ```
fn get_new_size(message: &str) -> Option<usize> {
    let mut new_size: Option<usize> = None;

    loop {
        match get_int_input(message) {
            None => {
                eprintln!("Keeping the current size...");

                break;
            }
            Some(1 | 2) => {
                    eprintln!("Must enter a value above `3`. Try again.");

                continue;
            }
            Some(val) => {
                new_size = Some(val);

                break;
            }
        }
    }

    new_size.map(|val| val)
}