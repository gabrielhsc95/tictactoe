use crate::Coordinates;
use crate::Player;

pub struct Board {
    pub matrix: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn to_string(&self) {
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

    pub fn is_valid(&self, c: &Coordinates) -> bool {
        self.matrix[c.1][c.0].is_none()
    }

    pub fn update(&mut self, c: &Coordinates, player: Player) {
        self.matrix[c.1][c.0] = Some(player);
    }

    pub fn new() -> Self {
        Board {
            matrix: [[None; 3]; 3],
        }
    }
}
