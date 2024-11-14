use crate::coordinate::{Coordinate, SafeCoordinate};
use crate::player::Player;
use std::collections::HashMap;

pub struct Board {
    pub matrix: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn is_valid(&self, c: &Coordinate) -> bool {
        self.matrix[c.y()][c.x()].is_none()
    }

    pub fn update(&mut self, c: &Coordinate, player: Player) {
        self.matrix[c.y()][c.x()] = Some(player);
    }

    pub fn get_empties_elements(&self) -> Vec<SafeCoordinate> {
        let mut empties_element: Vec<SafeCoordinate> = Vec::new();
        for (row_index, row) in self.matrix.iter().enumerate() {
            for (col_index, element) in row.iter().enumerate() {
                if element.is_none() {
                    empties_element.push(SafeCoordinate(col_index, row_index));
                }
            }
        }
        empties_element
    }

    pub fn get_winning_conditions(
        &self,
    ) -> HashMap<
        (SafeCoordinate, SafeCoordinate, SafeCoordinate),
        (Option<Player>, Option<Player>, Option<Player>),
    > {
        let winning_conditions = HashMap::from([
            (
                (
                    SafeCoordinate(0, 0),
                    SafeCoordinate(1, 0),
                    SafeCoordinate(2, 0),
                ),
                (self.matrix[0][0], self.matrix[0][1], self.matrix[0][2]),
            ),
            (
                (
                    SafeCoordinate(0, 1),
                    SafeCoordinate(1, 1),
                    SafeCoordinate(2, 1),
                ),
                (self.matrix[1][0], self.matrix[1][1], self.matrix[1][2]),
            ),
            (
                (
                    SafeCoordinate(0, 2),
                    SafeCoordinate(1, 2),
                    SafeCoordinate(2, 2),
                ),
                (self.matrix[2][0], self.matrix[2][1], self.matrix[2][2]),
            ),
            (
                (
                    SafeCoordinate(0, 0),
                    SafeCoordinate(0, 1),
                    SafeCoordinate(0, 2),
                ),
                (self.matrix[0][0], self.matrix[1][0], self.matrix[2][0]),
            ),
            (
                (
                    SafeCoordinate(1, 0),
                    SafeCoordinate(1, 1),
                    SafeCoordinate(1, 2),
                ),
                (self.matrix[0][1], self.matrix[1][1], self.matrix[2][1]),
            ),
            (
                (
                    SafeCoordinate(2, 0),
                    SafeCoordinate(2, 1),
                    SafeCoordinate(2, 2),
                ),
                (self.matrix[0][2], self.matrix[1][2], self.matrix[2][2]),
            ),
            (
                (
                    SafeCoordinate(0, 0),
                    SafeCoordinate(1, 1),
                    SafeCoordinate(2, 2),
                ),
                (self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]),
            ),
            (
                (
                    SafeCoordinate(0, 2),
                    SafeCoordinate(1, 1),
                    SafeCoordinate(2, 0),
                ),
                (self.matrix[2][0], self.matrix[1][1], self.matrix[0][2]),
            ),
        ]);
        winning_conditions
    }

    pub fn new() -> Self {
        Board {
            matrix: [[None; 3]; 3],
        }
    }
}
