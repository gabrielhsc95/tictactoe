use crate::board::Board;
use crate::coordinates::Coordinates;
use crate::player::Player;
use regex::Regex;
use std::io;

pub struct Terminal {}

impl Terminal {
    pub fn get_input(&self, current_player: &Player, board: &Board) -> Coordinates {
        loop {
            let mut input: String = String::new();
            println!(
                "player {}, input the coordinates (like \"x, y\"): ",
                print_player(current_player)
            );
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let re = Regex::new(r"\s+").unwrap();
            let input = re.replace_all(&input, "").to_string();
            let numbers: Vec<&str> = input.trim().split(",").collect();
            if numbers.len() == 2 {
                let mut x: usize = 0;
                let mut valid_x: bool = false;
                if let Ok(num) = numbers[0].parse() {
                    if num > 2 {
                        println!("Invalid x input. Numbers must be 0, 1, or 2.");
                    } else {
                        x = num;
                        valid_x = true;
                    }
                } else {
                    println!("Invalid x input. Numbers must be 0, 1, or 2.");
                }
                let mut y: usize = 0;
                let mut valid_y: bool = false;
                if let Ok(num) = numbers[1].parse() {
                    if num > 2 {
                        println!("Invalid y input. Numbers must be 0, 1, or 2.");
                    } else {
                        valid_y = true;
                        y = num;
                    }
                } else {
                    println!("Invalid x input. Numbers must be 0, 1, or 2.");
                }

                if valid_x && valid_y {
                    let c: Coordinates = Coordinates(x, y);
                    if board.is_valid(&c) {
                        break c;
                    } else {
                        println!("This was selected before, please pick another place!");
                    }
                }
            } else {
                println!("Invalid input. Please enter two numbers separated by a comma.");
            }
        }
    }

    pub fn display_board(&self, board: &Board) {
        clear_screen();
        println!("  0 1 2 x");
        for (row_index, row) in board.matrix.iter().enumerate() {
            print!("{} ", row_index);
            for (col_index, element) in row.iter().enumerate() {
                match element {
                    Some(p) => print!("{}", print_player(p)),
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

    pub fn display_winner(&self, player: &Player) {
        println!("{} won!", print_player(player));
    }

    pub fn display_draw(&self) {
        println!("DRAW!");
    }

    pub fn new() -> Self {
        Terminal {}
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_player(player: &Player) -> String {
    match player {
        Player::X => String::from("x"),
        Player::O => String::from("o"),
    }
}
