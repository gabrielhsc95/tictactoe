mod board;
mod coordinates;
mod player;
mod tictactoe;
use tictactoe::TicTacToe;

fn main() {
    let mut tic_tac_toe = TicTacToe::new();
    tic_tac_toe.play();
}
