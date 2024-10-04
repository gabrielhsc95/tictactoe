use crate::Board;
use crate::Coordinates;
use crate::Player;
use std::io;
fn are_all_same(a: &Option<Player>, b: &Option<Player>, c: &Option<Player>) -> bool {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => x == y && y == z,
        _ => false,
    }
}

fn check_win_conditions(board: &Board) -> bool {
    let winning_conditions: [(Option<Player>, Option<Player>, Option<Player>); 8] = [
        (board.matrix[0][0], board.matrix[0][1], board.matrix[0][2]),
        (board.matrix[1][0], board.matrix[1][1], board.matrix[1][2]),
        (board.matrix[2][0], board.matrix[2][1], board.matrix[2][2]),
        (board.matrix[0][0], board.matrix[1][0], board.matrix[2][0]),
        (board.matrix[0][1], board.matrix[1][1], board.matrix[2][1]),
        (board.matrix[0][2], board.matrix[1][2], board.matrix[2][2]),
        (board.matrix[0][0], board.matrix[1][1], board.matrix[2][2]),
        (board.matrix[2][0], board.matrix[1][1], board.matrix[0][2]),
    ];
    for (a, b, c) in winning_conditions {
        let win: bool = are_all_same(&a, &b, &c);
        if win {
            return win;
        }
    }
    false
}

pub struct TicTacToe {
    board: Board,
    won: bool,
    turn: u8,
}

impl TicTacToe {
    fn get_current_player(&self) -> Player {
        if self.turn % 2 == 0 {
            Player::X
        } else {
            Player::O
        }
    }

    fn get_input(current_player: &Player) -> Option<Coordinates> {
        let mut input: String = String::new();
        println!(
            "player {}, input the coordinates (like \"x, y\"): ",
            current_player.to_string()
        );

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let numbers: Vec<&str> = input.trim().split(", ").collect();
        if numbers.len() == 2 {
            let x: usize = numbers[0].parse().expect("Invalid input for x");
            let y: usize = numbers[1].parse().expect("Invalid input for y");

            if x > 3 {
                println!("Invalid x input. Numbers must be 0, 1, or 2.");
            }
            if y > 3 {
                println!("Invalid y input. Numbers must be 0, 1, or 2.");
            }

            if x < 3 && y < 3 {
                Some(Coordinates(x, y))
            } else {
                None
            }
        } else {
            println!("Invalid input. Please enter two numbers separated by a comma.");
            None
        }
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn play(&mut self) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            let coordinates: Option<Coordinates> = TicTacToe::get_input(&current_player);
            match coordinates {
                Some(c) => {
                    if self.board.is_valid(&c) {
                        Self::clear_screen();
                        self.board.update(&c, current_player);
                        self.board.to_string();
                        self.turn += 1;
                        self.won = check_win_conditions(&self.board);
                    } else {
                        println!("This was selected before, please pick another place!");
                    }
                }
                None => {}
            }
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            println!("{} won!", current_player.to_string());
        } else {
            println!("DRAW!");
        }
    }

    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
            won: false,
            turn: 0,
        }
    }
}
