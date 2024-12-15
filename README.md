## Othello
### Game Description
This game is Othello and is played by 2 players. One player controlls white tokens and one player controlls black tokens. The white and black teams are randomly assigned to the user player and to the computer player, called "Alex". The players take turns placing tiles on the board as long as on their turns they have tokens to play and legal moves they can make. If a move cannot be played then the turn is skipped. The game continues until all the tokens are used up or until the board is filled with tokens. When the game ends the winner is the player with more tokens on the board.

### How to Run
* You will need to have the Rust programming language installed on your computer. You can download and install Rust from rust-lang.org.
* Clone this repository to your computer. You can use the green `code` button, or git clone <https://github.com/csc3310-fall2024/rust-islalowe.git>
* Navigate on your computer to the directory where the project was saved.
* Run the game with the command: `cargo run`
* If you want to quit the game, use `cntr + c`

### How to Play
* To play the game, run it and then answer the prompt about whether you would like the instructions printed for you.
* After you pass through the instructions, the black and white tokens will be randomly assigned to each player. Whoever is assigned to play as black will go first. 
* the initial game board will be printed. After each player moves, the board will be printed again. There is a pause of 3 seconds before the board is paused to maximize readability.
* When it is the player's turn, you may type the coordinates of any legal move into the command line, in the form of `row` space `column`.
* The computer and human players will take turns placing tokens while they each have tokens to place and legal moves remain available.
* When the board is full or no more legal moves are possible, then the game ends and the player with more tokens on the board is the winner.

Requirements for a space to be considered a legal move include: 


#### YouTube Video Tutorial
https://youtu.be/Dd7qPwr3pZo

### Data Types
#### The Data Types used in this game include:
#### * Char
The color of the tokens being used by the user player and by the computer player is tracked with a character: either `W` or `B`.
Similarly, the tokens themselves and the current turn are also `W` or `B`, and when determining the winner the char `D` is used for a draw.
#### * Struct/Record
The Gamestate is a struct that creates a fresh instance state of the game. It builds the board as a multidimensional array, and establishes `white_tokens` and `black_tokens` as ints or u32. 
#### * Tuple
Tuples are used to represent coordinates and directions, as (row, column). They simplify the passing of position data between functions.
#### * String
Strings are used to display messages to the player, such as prompts for moves, game status updates, and the final results.
#### * Int/U32
The number of tokens in available for use by each of the players is bound to the `white_tokens` and `black_tokens` variables, which are of primitive datatype int. The values of both start at 2 because the game begins with their first two tokens placed on the board already. The board can contain 60 more tokens after these first setup tokens have been placed. When the `white_tokens` or `black_tokens` variables reach 32, the player has run out of available tokens.
#### * Multidimensional arrays
The gameboard is a  two-dimensional array of 8 rows and 8 columns. Each cell in the board is an array elemeent. These elements are occupied by ' ' whitespace, a `W` white token or a `B` black token.
#### * Vector
Vectors are used to dynamically store a list of tuples that represent all of the possible legal moves, in the form of (row, column) indeces that can be made for a player. 

