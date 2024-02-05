use super::cmdln_interface::{get_int_input, get_str_input};
use super::player::{List, Player, ControlMode, Sprite};
use super::cmdln_interface::{TO_MAIN, ESCAPE_CHAR, PLAYER_ATTRIBUTE_MENU, PROMPT_PLAYER_NAME,
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
            None => {
                eprintln!("{TO_MAIN}");

                break;
            }
            Some(1) => {
                let edited_player = edit_player(player_list.player_1.clone());
                player_list.player_1 = edited_player;
            }
            Some(2) => {
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
            None => {
                eprintln!("{TO_MAIN}");

                break;
            }
            Some(1) => {
                if let Some(new_name) = get_new_name(PROMPT_PLAYER_NAME) {
                    selected_player.name = new_name;
                };
            }
            Some(2) => {
                if let Some(new_type) = get_new_type(PROMPT_PLAYER_TYPE) {
                    selected_player.control = new_type;
                };
            }
            Some(3) => {
                if let Some(new_sprite) = get_new_sprite(PROMPT_PLAYER_SPRITE) {
                    selected_player.sprite = new_sprite;
                };
            }
            Some(4) => {
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
/// sprite = get_new_sprite("Editing a sprite: ");
///
/// println!("{sprite}");
/// ```
fn get_new_sprite(message: &str) -> Option<Sprite> {
    let new_sprite = get_str_input(message, 1);

    if new_sprite == ESCAPE_CHAR {
        None
    } else {
        Some(Sprite(new_sprite))
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
/// let name = get_new_name("Editing the player name: ");
/// player.name = match name {
///     Some(new_name) => new_name,
///     None => player.name,   
/// };
///
/// println!("{player}");
/// ```
fn get_new_name(message: &str) -> Option<String> {
    let new_name = get_str_input(message, 3);

    if new_name == ESCAPE_CHAR {
        None
    } else {
        Some(new_name)
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
/// control_type = get_new_type("Editing the type: ");
///
/// println!("{:?}", control_type);
/// ```
fn get_new_type(message: &str) -> Option<ControlMode> {
    let mut selected_type = None;

    loop {
        match get_int_input(message) {
            None => {
                eprintln!("Leaving the type editor...");

                break;
            }
            Some(1) => {
                selected_type = Some(1);

                break;
            }
            Some(2) => {
                selected_type = Some(2);

                break;
            }
            Some(_) => {
                eprintln!("Invalid selection");
            }
        }
    }

    match selected_type {
        None => None,
        Some(1) => Some(ControlMode::Human),
        Some(2) => Some(ControlMode::Ai),
        _ => panic!("There was an error getting user type"),
    }
}
