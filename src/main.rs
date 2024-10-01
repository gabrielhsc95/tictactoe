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

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_field(field: &[[Option<Player>; 3]; 3]) {
    for (row_index, row) in field.iter().enumerate() {
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
            println!("\n-----");
        }
    }
    println!();
}

fn are_all_same(a: &Option<Player>, b: &Option<Player>, c: &Option<Player>) -> bool {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => x == y && y == z,
        _ => false,
    }
}

fn check_win_conditions(field: &[[Option<Player>; 3]; 3]) -> bool {
    let winning_conditions: [(Option<Player>, Option<Player>, Option<Player>); 8] = [
        (field[0][0], field[0][1], field[0][2]),
        (field[1][0], field[1][1], field[1][2]),
        (field[2][0], field[2][1], field[2][2]),
        (field[0][0], field[1][0], field[2][0]),
        (field[0][1], field[1][1], field[2][1]),
        (field[0][2], field[1][2], field[2][2]),
        (field[0][0], field[1][1], field[2][2]),
        (field[2][0], field[1][1], field[0][2]),
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

fn get_input(turn: &u8) -> Option<Coordinates> {
    let current_player: String = get_current_player(turn).to_string();
    let mut input: String = String::new();
    println!(
        "player {}, input the coordinates (like \"x, y\"): ",
        current_player
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

fn is_valid(c: &Coordinates, field: &[[Option<Player>; 3]; 3]) -> bool {
    field[c.1][c.0].is_none()
}

fn update_field(field: &mut [[Option<Player>; 3]; 3], c: &Coordinates, turn: &u8) {
    let current_player: Player = get_current_player(turn);
    field[c.1][c.0] = Some(current_player);
}

fn main() {
    let mut field: [[Option<Player>; 3]; 3] =
        [[None, None, None], [None, None, None], [None, None, None]];
    let mut won: bool = false;
    let mut turn: u8 = 0;
    while !won && turn < 10 {
        let coordinates = get_input(&turn);
        match coordinates {
            Some(c) => {
                if is_valid(&c, &field) {
                    clear_screen();
                    update_field(&mut field, &c, &turn);
                    print_field(&field);
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
