use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Player {
    X,
    O,
}

impl Player {
    fn to_string(&self) -> String {
        match self {
            Player::X => String::from("x"),
            Player::O => String::from("o"),
        }
    }
}

struct Coordinates(usize, usize);

struct Board {
    matrix: [[Option<Player>; 3]; 3],
}

impl Board {
    fn to_string(&self) {
        println!("  0 1 2 x");
        for (row_index, row) in self.matrix.iter().enumerate() {
            print!("{} ", row_index);
            for (col_index, element) in row.iter().enumerate() {
                match element {
                    Some(p) => print!("{}", p.to_string()),
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

    fn is_valid(&self, c: &Coordinates) -> bool {
        self.matrix[c.1][c.0].is_none()
    }

    fn update(&mut self, c: &Coordinates, player: Player) {
        self.matrix[c.1][c.0] = Some(player);
    }

    fn new() -> Self {
        Board {
            matrix: [[None; 3]; 3],
        }
    }
}

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

struct TicTacToe {
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

    fn play(&mut self) {
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

    fn new() -> Self {
        TicTacToe {
            board: Board::new(),
            won: false,
            turn: 0,
        }
    }
}

fn main() {
    let mut tic_tac_toe = TicTacToe::new();
    tic_tac_toe.play();
}
