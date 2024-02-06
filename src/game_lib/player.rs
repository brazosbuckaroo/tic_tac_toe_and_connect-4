use std::fmt;

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