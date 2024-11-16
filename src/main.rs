
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


fn explain_rules() {
    print!("Would you like an explanation of the rules? Please enter Yes or No.");
    // todo maybe use unwrap for error hanlding, or something more graceful
    // create a mutable String to store the input
    let mut user_input; // declaration
    user_input = String::new(); // initialization

    io::stdin()
        .read_line(&mut user_input) // method to read input, assigns the input to user_input
        .expect("Failed to read line"); // todo make sure this is the best way to handle errors

    // print the instructions if requested
    if user_input == "YES" || user_input == "Yes" || user_input == "yes" || user_input == "Y" || user_input == "y" {
        println!("Instructions ^_^")
    }
}

fn print_board() {
    println!("BOARD!");
}

fn flip_tokens() {
    println!("FLIPPING TOKENS!");
}

fn count_tokens() {
    println!("Counting Tokens!");
}

    

fn main() {
    println!("Welcome to Othello!");
    explain_rules();

}