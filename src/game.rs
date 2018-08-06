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
        1..9 => {
            let (row, col) = Self::to_board_location(unchecked_move);
            match self.board[row][col].as_str() {
                "X" | "O" => false,
                _ => true,
            }
        }
        _ => false,
    }
}