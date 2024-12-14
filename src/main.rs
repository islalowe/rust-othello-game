/* Othello game for one player to play against a computer player, named Alex. The Alex character will play random legal moves. 
Kristen Lowe    -   12 Dec 2014
Concepts of Programming Languages
Winter 2024     -   Prof Arias */


use std::io;
use rand::Rng; 


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
    player_color: char
}


impl GameState {
    // Make a new gameboard, and include the methods that manage it.
    fn new(player_color: char) -> GameState {
        let mut board = [[' '; 8]; 8];
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
        Self {
            board,
            white_tokens: 2,
            black_tokens: 2,
            current_turn: 'B',
            player_color
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
    fn print_board(&self) {
        println!("\n          1 2 3 4 5 6 7 8");
        // Print the board with row numbers and separators
        for (i, row) in self.board.iter().enumerate() {
            // Print the row number with a fixed width for alignment
            print!("row {}    ", i + 1);
            // Print each cell with vertical borders
            for cell in row.iter() {
                print!("|{}", cell);
            }
            println!("|");  // Close the row with a final vertical border
        }
        println!();
    }
    

    /*
    This function counts and returns the number of tokens that each of the two players has currently.
    @param self: instantiation of the GameState struct
    @return u32, u32: 
     */
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


    
    /* This function returns a vector of all the posssible legal moves that can be made at the current point in the game.
    The current instance of the gamestate is passed as a paramteter, immutably.
    Requirements for a legal move include: 
        - the space must be empty
        - the space must be connecting to the start of a line of tokens.
            > the line muxt consist of only opponent tokens.
            > the line ends with another one of the same team tokens.
            > the line goes in one of these 6 directions: 
            vertically, horizontally, diagonally (top-left, top-right, bottom-left, bottom-right).
    @param game: an instance of a game object/instantiation of the GameState class.
    @return legal_moves: a vector of tuples, representing possible moves that can be made.
     */
    fn find_legal_moves(&self) -> Vec<(usize, usize)> {
        let mut legal_moves = Vec::new();
    
        // Check each space on the board for an empty one
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] == ' ' {
                    // The space is empty, now check surrounding tokens
                    let directions = [
                        (-1, 0), (1, 0), (0, -1), (0, 1),
                        (-1, -1), (-1, 1), (1, -1), (1, 1),
                    ]; // valid move will become true if the current empty square is next to an opponent square(s), followed by current player square
                    let mut valid_move = false;
                    // iterating through 
                    for &(dx, dy) in &directions {
                        let mut temp_row = row as i32 + dx;
                        let mut temp_col = col as i32 + dy;
                        let mut has_opponent_between = false;
                        //let mut tokens_to_flip = Vec::new();                     
                        // Traverse in the direction until we reach the edge of the board
                        while temp_row >= 0 && temp_row < 8 && temp_col >= 0 && temp_col < 8 {
                            let r = temp_row as usize;
                            let c = temp_col as usize;
    
                            if self.board[r][c] == ' ' {
                                // Empty space, stop checking this direction
                                break;
                            } else if self.board[r][c] == self.current_turn {
                                // If we reach our token, check if there's an opponent in between
                                if has_opponent_between {
                                    valid_move = true; // Valid move found
                                   // println!("tokens_to_flip: {:?}", tokens_to_flip); // Debugging
                                }
                                break;
                            } else {
                                // Opponent's token
                                has_opponent_between = true;
                               // tokens_to_flip.push((r, c));
                            }
    
                            // Move further in the current direction
                            temp_row += dx;
                            temp_col += dy;
                        }
                    }
                    // If the move is valid, add it to the list of legal moves
                    if valid_move {
                        legal_moves.push((row, col));
                    }
                }
            }
        }
        legal_moves // Return the list of legal moves
    }


 
    /* This function will generate random legal moves for the computer player, named Alex, to make */
    fn alex_moves(&mut self) {
        //Alex picks a random legal move
        println!("It is Alex's turn!");
        let legal_moves = self.find_legal_moves();
        // if there are no legal moves, Alex's turn is skipped
        if !legal_moves.is_empty() {
            let (move_row, move_col) = legal_moves[rand::thread_rng().gen_range(0..legal_moves.len())];
            println!("Alex's move is: {}, {}", move_row + 1, move_col + 1);
            self.flip_tokens(move_row, move_col);
            self.print_board();
            if self.player_color == 'W' {       // if Alex is using the Black tokens
                self.black_tokens = self.black_tokens + 1;
            } else {
                self.white_tokens = self.white_tokens + 1;
            }
        } else {
            println!("No legal moves available. Skipping turn.");
        }
    }


     /* This function handles the player turn. It prompts a user for input and then checks the input move to make sure it is valid.
    @return: a tuple containing the move input by the player
     */
    fn player_moves(&mut self) {
        println!("It is your turn! Please enter a valid move in the format: row column");
        self.print_board();
        let possible_moves = self.find_legal_moves(); 
        if possible_moves.is_empty() {
            println!("No legal moves available. Skipping turn.");
            return;
        }
        let mut user_input = String::new();
        loop {
            user_input.clear(); // Clear previous input
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read input");

            // split input and parse it into integers
            let parts: Vec<&str> = user_input.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    let move_tuple = (row - 1, col - 1);  // decrease by 1 so the player can count from 1 instead of 0
                    // check if the move is in the list of possible moves
                    if possible_moves.contains(&move_tuple) {
                        println!("Move accepted.");
                        self.flip_tokens(row - 1, col - 1);  // apply the move to the board
                        self.print_board();
                        // adjust token count
                        if self.player_color == 'W' {       
                            self.white_tokens = self.white_tokens + 1;
                        } else {
                            self.black_tokens = self.black_tokens + 1;
                        }
                        break; // Exit the loop
                    } else {
                        println!("Invalid move. Please try again.");
                        //println!("Invalid move. Please choose from the available moves: {:?}", possible_moves);
                    }
                } else {
                    println!("Invalid input format. Please enter two numbers separated by a space.");
                }
            } else {
                println!("Invalid input. Please enter your move as 'row column'.");
            }
        }
    }


    /* This function determines whether the game has reached its endstate succsesfully and the game has ended.
    Will be true if there are no more possible legal moves, or if all the tokens are used.
    @param game: an instance of a game object/instantiation of the GameState class.
    @return bool: a True/False value indicating whether the game is over.
    */
    fn check_endstate(&self) -> bool {
        // destructure the tuple containing token values
        let (white, black) = self.count_tokens();
        let total_tokens = white + black;
        let legal_moves = self.find_legal_moves();   
        if total_tokens == 64 || legal_moves.len() < 1 {
            return true
        }
        return false
    }
    

/* This function will flip the tokens that can be flipped in 8 directions from the current space.
@param game: an instance of a game object/instantiation of the GameState class.
@param row: an index representing the current row
@param col: an index representing the current column
*/
fn flip_tokens(&mut self, row: usize, col: usize) {
    // place the current player's token at the move location
    self.board[row][col] = self.current_turn;
    // check in each direction and flip tokens
    let directions = [
        (-1, 0), (1, 0), // vertical: up, down
        (0, -1), (0, 1), // horizontal: left, right
        (-1, -1), (-1, 1), // diagonal: top-left, top-right
        (1, -1), (1, 1), // diagonal: bottom-left, bottom-right
    ];
    let current_player = self.current_turn;
    let opponent = if current_player == 'B' { 'W' } else { 'B' };
    // Check each direction
    for &(dx, dy) in &directions {
        let mut temp_row = row as i32 + dx;
        let mut temp_col = col as i32 + dy;
        let mut tokens_to_flip = Vec::new();
        // continue in the direction until the end or the same color token is reached
        while temp_row >= 0 && temp_row < 8 && temp_col >= 0 && temp_col < 8 {
            let r = temp_row as usize;
            let c = temp_col as usize;
            // r and c are row and columns
            if self.board[r][c] == opponent {
                tokens_to_flip.push((r, c)); // opponent token found
            } else if self.board[r][c] == current_player {
                // if a token of the same color is found, flip the opponent tokens
                for (flip_r, flip_c) in tokens_to_flip {
                    self.board[flip_r][flip_c] = current_player;
                }
                break;
            } else {
                break; // empty space, stop in this direction
            }
            // move in the direction
            temp_row += dx;
            temp_col += dy;
        }
    }
}

/* This function will switch the current turn to the opposite player. 
*/
fn switch_turn(&mut self) {
    self.current_turn = if self.current_turn == 'B' { 'W' } else { 'B' };
}


/* This function will determine the winner of the game by summing and comparing the totals of black and white tokens on the board.
@return: either 'W' or 'B', representing the winner of the game, or 'D' for a draw. 
*/
fn determine_winner(&self) -> char {
    let (white_count, black_count) = self.count_tokens();
    if white_count > black_count {
        return 'W';
    } else if black_count > white_count {
        return 'B';
    } else {
        // If the counts are the same, it could be a draw
        println!("It's a draw!");
        return 'D'; // You can return 'D' for draw or handle it however you prefer
    }
}

}
   
    
/* This function will ofter explain the rules to the user. 
*/
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
                println!("Here are the instructions ^_^\n There are 2 players who play with 64 tokens on an 8x8 board. 
                You will play against Alex as either white or black. Black always moves first. The fresh board begins with two white and two black 
                tokens criss-crossed in the center of hte board. You and Alex will take turns placing tokens until either 
                no valid moves remian, or all the tokens are used up. If there are no valid moves possible for a player, 
                then that player's turn is skipped. The winner is the whoever has more tokens on the board than the other.
                For a move to be valid the space must be empty, and it must be flanking a sequence of opponent tokens.");
                break;
            }
            "no" | "n" => break,
            _ => println!("Invalid input! Please enter Yes or No."),
        }
    }
}    


fn main() {
    println!("Welcome to Othello!");
    explain_rules();

    // randomly assign black an white pieces to each player
    // Generate a random integer in a range
    let random_int = rand::thread_rng().gen_range(0..3);   // 1 to 3 (inclusive lower, exclusive upper)
    let player_color: char;
    if random_int == 1 {
        player_color = 'B';
        println!("You will be playing as Black for this game, and Alex will be playing as White! You will go first.");
    } else {
        player_color = 'W';
        println!("You will be playing as White for this game, and Alex will be playing as Black! Alex will go first.");
    }
    // start a new game with the teams decided
    let mut game = GameState::new(player_color);
    println!("Here is the gameboard! Let's play.");
    game.print_board();
    // turn loop that continues until the game is over
    while !game.check_endstate() {
        if game.current_turn == game.player_color {
            game.player_moves();
        } else {
            game.alex_moves();
        }
        game.switch_turn();
    }

    println!("Game Over! Final Board:");
    game.print_board();
    // determine who won the game
    let winner_color = game.determine_winner();
    if player_color == winner_color {
        println!("You win!")
    } else {
        println!("Alex wins!")
    }

}

