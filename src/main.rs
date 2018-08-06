extern crate rand;
mod game;
use game::Game;
fn main() {
    println!("Tic Tac toe");
    let mut game = Game::new();
    game.play_game();

}
