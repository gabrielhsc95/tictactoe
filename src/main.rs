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

struct Field {
    matrix: [[Option<Player>; 3]; 3],
}

impl Field {
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
        Field {
            matrix: [[None; 3]; 3],
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn are_all_same(a: &Option<Player>, b: &Option<Player>, c: &Option<Player>) -> bool {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => x == y && y == z,
        _ => false,
    }
}

fn check_win_conditions(field: &Field) -> bool {
    let winning_conditions: [(Option<Player>, Option<Player>, Option<Player>); 8] = [
        (field.matrix[0][0], field.matrix[0][1], field.matrix[0][2]),
        (field.matrix[1][0], field.matrix[1][1], field.matrix[1][2]),
        (field.matrix[2][0], field.matrix[2][1], field.matrix[2][2]),
        (field.matrix[0][0], field.matrix[1][0], field.matrix[2][0]),
        (field.matrix[0][1], field.matrix[1][1], field.matrix[2][1]),
        (field.matrix[0][2], field.matrix[1][2], field.matrix[2][2]),
        (field.matrix[0][0], field.matrix[1][1], field.matrix[2][2]),
        (field.matrix[2][0], field.matrix[1][1], field.matrix[0][2]),
    ];
    for (a, b, c) in winning_conditions {
        let win: bool = are_all_same(&a, &b, &c);
        if win {
            return win;
        }
    }
    false
}

fn get_current_player(turn: &u8) -> Player {
    if turn % 2 == 0 {
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
            println!("Invalid x input. Numbers must be 0, 1, or 2.")
        }
        if y > 3 {
            println!("Invalid y input. Numbers must be 0, 1, or 2.")
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

fn main() {
    let mut field: Field = Field::new();
    let mut won: bool = false;
    let mut turn: u8 = 0;
    while !won && turn < 9 {
        let current_player: Player = get_current_player(&turn);
        let coordinates: Option<Coordinates> = get_input(&current_player);
        match coordinates {
            Some(c) => {
                if field.is_valid(&c) {
                    clear_screen();
                    field.update(&c, current_player);
                    field.to_string();
                    turn += 1;
                    won = check_win_conditions(&field);
                } else {
                    println!("This was selected before, please pick another place!")
                }
            }
            None => {}
        }
    }
    if won {
        turn -= 1;
        let current_player: Player = get_current_player(&turn);
        println!("{} won!", current_player.to_string())
    } else {
        println!("DRAW!")
    }
}
