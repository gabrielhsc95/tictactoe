mod board;
mod coordinates;
mod player;
mod tictactoe;
use board::Board;
use coordinates::Coordinates;
use player::Player;
use tictactoe::TicTacToe;

fn main() {
    let mut tic_tac_toe = TicTacToe::new();
    tic_tac_toe.play();
}
