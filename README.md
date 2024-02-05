# Tic-Tac-Toe and Connect-4 #

A simple game engine to make a Tic-Tac-Toe or Connect-4 game. This is mostly being used as a way to learn Rust before I go on to do anything more complicated.


## Features ##

### Current Features ###
* Allows players to add their own name with a three character limit
* Users can select whether they want to play Tic-Tac-Toe or Connect-4 
* After each round, users are able to change their names
* Keep track of scores outside of each round so players can keep their score until they want to switch players
* Format error messages to be more clear and visible
* Allow user to play on a user selected sized grid
* Allow users to select which player to change the name of and reset wins
* Allow users to change what their sprite is between rounds
* Added a very simple ai that just randomly picks a number from a valid moves list


### Planned Features ###
* Improve the interface by allowing the user to select a cell with arrow keys


## Bug Tracking ##

### Known Bugs ###


### Fixed Bugs ###
* Move logic for Connect-4 is not accurate to what it is in the real world
* When asking to start a new round, anything that isn't 'N' can cause it to restart.
* When starting a new round, after getting a new player name the wins are persistent
* When the last move should be a winning move, the game will report a tie


## Dependencies ##
