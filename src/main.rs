mod board;
mod coordinates;
mod player;
mod terminal;
mod tictactoe;
use terminal::Terminal;
use tictactoe::TicTacToe;

fn main() {
    let terminal = Terminal::new();
    let mut tic_tac_toe = TicTacToe::new(terminal);
    tic_tac_toe.play();
}
