use super::UserInterface;
use crate::board::Board;
use crate::coordinate::ValidCoordinate;
use crate::error::Error;
use crate::error::Result;
use crate::player::Player;
use regex::Regex;
use std::io;

pub struct TerminalUserInterface {}

impl UserInterface for TerminalUserInterface {
    fn get_input(&self, current_player: &Player, board: &Board) -> Result<ValidCoordinate> {
        let mut input: String = String::new();
        println!("player {current_player}, input the coordinates (like \"x, y\"): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let re = Regex::new(r"\s+").expect("This regex should be valid!");
        let input = re.replace_all(&input, "").to_string();
        let numbers: Vec<&str> = input.trim().split(|s| s == ',' || s == '.').collect();
        let x: usize;
        let y: usize;
        if numbers.len() == 2 {
            match numbers[0].parse() {
                Ok(num) => x = num,
                Err(e) => return Err(Error::Parse(e)),
            }
            match numbers[1].parse() {
                Ok(num) => y = num,
                Err(e) => return Err(Error::Parse(e)),
            }
            return ValidCoordinate::new(x, y, board);
        } else {
            return Err(Error::CoordinateFormatInvalid);
        }
    }

    fn display_error(&self, error: crate::error::Error) {
        println!("{error}");
    }

    fn display_board(&self, board: &Board) {
        clear_screen();
        println!("  0 1 2 x");
        for (row_index, row) in board.matrix.iter().enumerate() {
            print!("{row_index} ");
            for (col_index, element) in row.iter().enumerate() {
                match element {
                    Some(p) => print!("{p}"),
                    None => print!(" "),
                }
                if col_index != 2 {
                    print!("|");
                }
            }
            if row_index != 2 {
                println!("\n  -----");
            }
        }
        println!("\ny");
    }

    fn display_winner(&self, player: &Player) {
        println!("{player} won!");
    }

    fn display_draw(&self) {
        println!("DRAW!");
    }
}

impl TerminalUserInterface {
    pub fn new() -> Self {
        TerminalUserInterface {}
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
