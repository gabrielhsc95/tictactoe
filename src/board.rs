use crate::coordinates::Coordinates;
use crate::player::Player;

pub struct Board {
    pub matrix: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn is_valid(&self, c: &Coordinates) -> bool {
        self.matrix[c.1][c.0].is_none()
    }

    pub fn update(&mut self, c: &Coordinates, player: Player) {
        self.matrix[c.1][c.0] = Some(player);
    }

    pub fn get_empties_elements(&self) -> Vec<Coordinates> {
        let mut empties_element: Vec<Coordinates> = Vec::new();
        for (row_index, row) in self.matrix.iter().enumerate() {
            for (col_index, element) in row.iter().enumerate() {
                if element.is_none() {
                    empties_element.push(Coordinates(col_index, row_index));
                }
            }
        }
        empties_element
    }

    pub fn new() -> Self {
        Board {
            matrix: [[None; 3]; 3],
        }
    }
}
