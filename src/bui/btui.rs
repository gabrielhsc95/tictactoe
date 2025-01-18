use super::BinaryUserInterface;
use crate::bcoodinate::ValidBinaryCoordinate;
use crate::butils::identify_position_state;
use crate::error::{Error, Result};
use regex::Regex;
use std::io;

pub struct BinaryTerminalUserInterface {}

impl BinaryUserInterface for BinaryTerminalUserInterface {
    fn get_input(&self, current_player: bool, board: u32) -> Result<ValidBinaryCoordinate> {
        let mut input: String = String::new();
        let player: char = if current_player { 'x' } else { 'o' };
        println!("player {player}, input the coordinates (like \"x, y\"): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let re = Regex::new(r"\s+").expect("This regex should be valid!");
        let input = re.replace_all(&input, "").to_string();
        let numbers: Vec<&str> = input.trim().split(|s| [',', '.'].contains(&s)).collect();
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
            ValidBinaryCoordinate::from_xy(x, y, current_player, board)
        } else {
            Err(Error::CoordinateFormatInvalid)
        }
    }

    fn display_error(&self, error: Error) {
        println!("{error}");
    }

    fn display_board(&self, board: u32) {
        clear_screen();
        println!("  0 1 2 x");
        let mut y = 0;
        for p in (0..9).rev() {
            if [8, 5, 2].contains(&p) {
                print!("{y} ");
                y += 1;
            }
            print!("{}", identify_position_state(p, board));
            if [8, 7, 5, 4, 2, 1].contains(&p) {
                print!("|");
            } else if [6, 3].contains(&p) {
                println!("\n  -----");
            } else {
                println!("\ny");
            }
        }
    }

    fn display_winner(&self, player: bool) {
        if player {
            println!("x won!");
        } else {
            println!("o won!");
        }
    }
    fn display_draw(&self) {
        println!("DRAW!")
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
