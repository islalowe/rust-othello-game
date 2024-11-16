
fn main() {
    println!("Welcome to Othello!");
    
}

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
    // white tokens on row4, column4 and row5, column 5 and black tokens on row4, column5 and row5, column4
    // or
    // black tokens on row4, column4 and row5, column 5 and white tokens on row4, column5 and row5, column4
// each player is assigned a color 
// random coin flip to decide who goes first, and then the players take turns
// to make a move, flank an opposing color
    // if flanking is not possible, turn is skipped
    // if flanking is possible, every of the opponents tokens that are within the flanking is flipped to become the current player's token
// gameplay continues until tokens are all used
// include error handling and an welcome message that offers to explain gameplay rules

/*  to make:
// function to explain the rules (only called if user asks for it)
*/