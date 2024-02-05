//! # `game_lib`
//!
//! This library is meant to provide an easy to use game engine for simple games
//! like Tic-Tac-Toe, Connect-4, Chess, and Checkers. As of right now, the only games
//! current implemented are Tic-Tac-Tie and Connect-4.
//! 

/// A simple implementation of a struct that can create any grid based game
pub mod game;
/// Provides a player struct and AI Engine
pub mod player;
/// This module is used as the Game Engine with all the game logic
pub mod game_engine;
/// This is a module that allows users to edit the Game struct. In a way, it is a 
/// program in of itself.
pub mod game_editor;
/// This module is simlar to the Game Editor module but is specifically meant for 
/// editing Player structs.
pub mod player_editor;

use super::ui_lib::cmdln_interface;