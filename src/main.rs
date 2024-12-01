
use std::io;

// row 1    | | | | | | | | | 
// row 2    | | | | | | | | |
// row 3    | | | | | | | | |
// row 4    | | | |W|B| | | |
// row 5    | | | |B|W| | | |
// row 6    | | | | | | | | |
// row 7    | | | | | | | | |
// row 8    | | | | | | | | |

// 2 players, 2 colors
// 8 x 8 board
// 64 tokens
// set up by placing:
    // white tokens on row4, column4 and row5, column5 and black tokens on row4, column5 and row5, column4
    // or
    // black tokens on row4, column4 and row5, column5 and white tokens on row4, column5 and row5, column4
// each player is assigned a color 
// random coin flip to decide who goes first, and then the players take turns
// to make a move, flank an opposing color
    // if flanking is not possible, turn is skipped
    // if flanking is possible, every of the opponents tokens that are within the flanking is flipped to become the current player's token
// gameplay continues until tokens are all used
// include error handling and an welcome message that offers to explain gameplay rules

/*  to make:
> function to explain the rules (only called if user asks for it)
> function to print the current state of the board
> function to flip very token within a flank to the opposite team
> function to keep track of tokens for each player
*/


struct GameState {
    board: [[char; 8]; 8], // 8x8 grid, 'W', 'B', or ' ' for empty
    white_tokens: u32,
    black_tokens: u32,
    current_turn: char // 'W' for white, 'B' for black
}


impl GameState {
    // Make a new gameboard
    fn new() -> GameState {
        // Board is a 2D fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N
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
        }
    }

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

    fn check_endstate(game: &GameState) -> bool {
        let (white, black) = game.count_tokens();
        let total_tokens = white + black;
        total_tokens == 64 || white == 0 || black == 0
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


fn print_board() {
    println!("Current BOARD!");
}

fn update_board(tokens_flipped) {
    println!("FLIPPING TOKENS!");
    return current_board;
}

fn count_tokens() {
    println!("Counting Tokens!");
}

fn check_endstate(current_board) {
    bool game_over = false;
    return game_over;
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

    let mut game = GameState::new();
    game.print_board();

    while !check_endstate(&game) {
        if game.current_turn == 'B' {
            player_moves(&mut game);
        } else {
            alex_moves(&mut game);
        }
        game.print_board();
    }

    println!("Game over!");
}
