
use std::io;
use std::ops::Neg;
use rand::Rng; // for generating random values



// 2 players, 2 colors
// 8 x 8 board
// 64 tokens
// set up by placing:
    // white tokens on row4, column4 and row5, column5 and black tokens on row4, column5 and row5, column4
    // or
    // black tokens on row4, column4 and row5, column5 and white tokens on row4, column5 and row5, column4
// each player is assigned a color 
// black always moves first
// players take turns  - if they can move
// to make a move, flank an opposing color
    // if flanking is not possible, turn is skipped
    // if flanking is possible, every of the opponents tokens that are within the flanking is flipped to become the current player's token
// gameplay continues until tokens are all used OR no more legal moves exist
// include error handling and an welcome message that offers to explain gameplay rules
// requirements for a legal move: 
    // - 

/*  to make:
> function to flip evry token within a flank to the opposite team
*/


/*
This struct maintains the current GameState through 4 variables. 
board: a visual representation of the tokens placed on the board. A 2D array.
white_tokens & black_tokens: primitive integer types representing the number of tokens possesed by each player (in hand, not on the board)
current_turn: a character representing the current player to move. 'W' or 'B'.
*/
struct GameState {
    board: [[char; 8]; 8], // 8x8 grid, 'W', 'B', or ' ' for empty
    white_tokens: u32,
    black_tokens: u32,
    current_turn: char, // 'W' for white, 'B' for black
    player_color: char,
    alex_color: char

}


impl GameState {
    // Make a new gameboard
    fn new() -> GameState {
        // board is a 2D fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N
        let mut board = [[' '; 8]; 8];
        // Set up starting configuration
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
        Self {
            board,
            white_tokens: 2,
            black_tokens: 2,
            current_turn: 'B', // Black starts by default
            player_color,
            alex_color
        }
    }

    /*
    This function prints the board
     row 1    | | | | | | | | | 
     row 2    | | | | | | | | |
     row 3    | | | | | | | | |
     row 4    | | | |W|B| | | |
     row 5    | | | |B|W| | | |
     row 6    | | | | | | | | |
     row 7    | | | | | | | | |
     row 8    | | | | | | | | |
     */
    //todo does this change based off of the GameState current instance or something else
    fn print_board(&self) {
        println!("  1 2 3 4 5 6 7 8");
        for (i, row) in self.board.iter().enumerate() {
            print!("{} ", i + 1);
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }

    /*
    This function counts and returns the number of tokens that each of the two players has currently.
    @param self: instantiation of the GameState struct
    @return u32, u32: 
     */
    //todo u32 is a type, like int. it must be returned twice, 
    //todo but the returns cannot be clearly separated by name. maybe pass them to an array or send them to the count function
    fn count_tokens(&self) -> (u32, u32) {
        let mut white = 0;
        let mut black = 0;
        for row in &self.board {
            for &cell in row {
                if cell == 'W' {
                    white += 1;
                } else if cell == 'B' {
                    black += 1;
                }
            }
        }
        (white, black)
    }

    /* This function returns an array of all the posssible legal moves that can be made at the current point in the game.
    Requirements for a legal move include: 
        - the space must be empty
        - the space must be connecting to the start of a line of tokens.
            > the line muxt consist of only opponent tokens.
            > the line ends with another one of the same team tokens.
            > the line goes in one of these 6 directions: 
            vertically, horizontally, diagonally (top-left, top-right, bottom-left, bottom-right).
    @param game: an instance of a game object/instantiation of the GameState class.
    @return bool: a True/False value indicating whether the game is over.
     */
    //todo should it be an array
    fn find_legal_moves(&self) -> array {

    }

    /* This function determines whether the game has reached its endstate succsesfully and the game has ended.
    @param game: an instance of a game object/instantiation of the GameState class.
    @return bool: a True/False value indicating whether the game is over.
     */
    fn check_endstate(game: &GameState) -> bool {
        // destructure the tuple containing token values
        let (white, black) = game.count_tokens();
        let total_tokens = white + black;
        let legal_moves = find_legal_moves();
        if (total_tokens == 64 || legal_moves.size() > 1) {
            return true
        }
        return false
    }
    


}


fn explain_rules() {
    let mut user_input = String::new(); // declaration + initialization

    loop {
        println!("Would you like an explanation of the rules? Please enter Yes or No.");
        user_input.clear(); // Clear previous input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let input = user_input.trim().to_lowercase(); // Normalize input
        match input.as_str() {
            "yes" | "y" => {
                println!("Instructions ^_^");
                break;
            }
            "no" | "n" => break,
            _ => println!("Invalid input! Please enter Yes or No."),
        }
    }
}



fn flip_tokens(game: &GameState) ->  {
    println!("FLIPPING TOKENS!");
    return current_board;
}




fn alex_moves() {
    println!("Alex will move now!");
}

fn player_moves() {
    println!("PLayer will move now!");
}


    

fn main() {
    println!("Welcome to Othello!");
    explain_rules();
    
//todo add player colors to the gamestate struct so they are not global variables.

    // randomly assign black an white pieces to each player
    // Generate a random integer in a range
    let random_int: u32 = rng.gen_range(1..3); // 1 to 3 (inclusive lower, exclusive upper)
    if (random_int == 1) {
        let player_color = 'B';
        let alex_color = 'W';
        println("You will be playing as Black for this game, and Alex will be playing as White! You will go first.")
        // player plays as black and goes first
    } else {
        //alex plays as black and goes first
        let player_color = 'W';
        let alex_color = 'B';
        println("You will be playing as White for this game, and Alex will be playing as Black! Alex will go first.")
    }

    let mut game = GameState::new();
    game.print_board();

    while check_endstate(&game) Neg {
        if game.current_turn == 'B' {
            player_moves(&mut game);
        } else {
            alex_moves(&mut game);
        }
        game.print_board();
    }

    println!("Game over!");
}
