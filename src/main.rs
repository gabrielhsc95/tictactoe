mod board;
mod coordinates;
mod player;
mod terminal;
mod tictactoe;
use std::io;
use terminal::Terminal;
use tictactoe::TicTacToe;

fn main() {
    let terminal = Terminal::new();
    let mut tic_tac_toe = TicTacToe::new(terminal);
    tic_tac_toe.play();
    println!("Press Enter to exit...");
    match io::stdin().read_line(&mut String::new()) {
        Ok(_) => {}
        Err(error) => println!("Error reading input: {}", error),
    }
}
