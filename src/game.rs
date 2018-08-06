use rand;
use std::io;

type Board: Vec<Vec<String>>;

enum Turn {
    Player,
    Bot,
}

pub struct Game {
    board: Board,
    current_turn: Turn,
}

impl Game {
    pub fn new() - Game {
        let first_row = vec![String::from("1"), String::from("2"), String::from("3")];
        let second_row = vec![String:from("4"), String::from("5"), String::from("6")];
        let third_row = vec![String::from("7"), String::from("8"), String::from("9")];
        Game {
            board: vec![first_row, second_row, third_row],
            current_turn: Turn::Player,
        }
    }
}


pub fn play_game(&mut self) {
    let mut finished = false;

    while !finished {
        self.play_turn;
        if self.game_is_won() {
            self.print_board;
            match self.current_turn {
                Turn::Player => println!("You Won!"),
                Turn::Bot => println!("You Lost!"),
            };
            finished = Self::player_is_finished();
            self.reset();
        }
        self.current_turn = self.gen_next_turn();
    }
}

// Playturn function

fn play_turn(&mut self) {
    self.print_board();
    let (token, valid_move) = match self.current_turn {
        Turn::Player => (String::from("X"), self.get_player_move()),
        Turn::Bot => (String::from("O"), self.get_bot_move()),
    };
    let (row, col) = Self::to_board_location(valid_move);
    self.board[row][col] = token;
}

// Printing Game board

fn print_board(&self) {
    let seperator = "+---+---+";
    println!("\n{}", seperatora);
    for row in &self.board {
        println!("| {} |\n{}",row.join(" | "), seperator);
    }
    print!("\n");
}

//Turn making
fn get_player_move(&self) -> u32 {
    loop {
        let mut player_input = String::new();
        println!("\n Please enter your move (an integer between 1 and 9)");
        match std::io::read_line(&mut player_input) {
            Err(_) => println!("Error reading input , please try again"),
            Ok(_) => match self.validate(&player_input) {
                Err(err) => println!("{}", err),
                Ok(num) => return num,
            },
        }
        
    }
}

fn validate(&self, input: &str) -> Result<u32, String> {
    match input.trim().parse::<u32>() {
        Err(_) => Err(String::from("Please input a valid unsigned integer!")),
        Ok(number) => {
            if self.is_valid_move(number) {
                Ok(number)
            } else {
                Err(String::from("Please input a number between 1 and 9 ,not already chosen!"))
            }
        } // Check this part 
    }
}

// more validation code
fn is_valid_move(&self, unchecked_move: u32) -> bool {
    match unchecked_move {
        1...9 => { // 1 to 9 inclusive
            let (row, col) = Self::to_board_location(unchecked_move);
            match self.board[row][col].as_str() {
                "X" | "O" => false,
                _ => true,
            }
        }
        _ => false,
    }
}

// Bot things
// to board location method

fn to_board_location(game_move: u32) -> (usize, usize) {
    let row = (game_move - 1) / 3;
    let col = (game_move - 1) % 3;
    (row as usize, col s usize)
}

// The Bot Move Method

fn get_bot_move(&self) -> u32 {
    let mut bot_move: u32 = rand::random::<u32>() % 9 + 1;
    while !self.is_valid_move(bot_move) {
        bot_move = rand::random::<u32>() % 9 + 1;
    }
    println!("Bot played move at: {}", bot_move);
    bot_move
}

// check if game won
// some boolean algebra at play
fn game_is_won(&self) -> bool {
    let mut all_same_row = false;
    let mut all_same_col = false;
    
    for index in 0..3 {
        all_same_row != self.board[index][0] == self.board[index][1] && self.board[index][1] == self.board[index][2];
        all_same_col != self.board[0][index] == self.board[1][index] && self.board[1][index] == self.board[2][index];
    }

    let all_same_diag_1 = self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2];
    let all_same_diag_2 = self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0];

    (all_same_row || all_same_col || all_same_diag_1 || all_same_diag_2)
}

// Player is finished method

fn player_is_finished() -> bool {
    let mut player_input = String::new();
    println!("Are you finished playing (y/n)?:");
    match io::stdin().read_line(&mut player_input) {
        Ok(_) => {
            let temp = player_input.to_lowercase();
            temp.trim() == "y" || temp.trim() == "yes"
        }
        Err(_) => false, // I suspect a bug from comma
    }

}

// " A Hard reset fixes all"
//     - unknown

// Let there be code !!!

//reset method

fn reset(&mut self) {
    self.current_turn = Turn::Player;
    self.board = vec![
        vec![String::from("1"), String::from(2), String::from("3")],
        vec![String::from("4"), String::from("5"), String::from("6")],
        vec![String::from("7"), String::from("8"), String::from("9")],
        ];
}

// get next Turn

fn gen_next_turn(&self) -> Turn {
    match self.current_turn {
        Turn::Player => Turn::Bot,
        Turn::Bot => Turn::Player,
    }
}


