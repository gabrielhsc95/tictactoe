use crate::coordinate::Coordinate;
use crate::player::Player;
use std::collections::HashMap;

pub struct Board {
    pub matrix: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn update(&mut self, c: &Coordinate, player: Player) {
        self.matrix[c.1][c.0] = Some(player);
    }

    pub fn get_empty_elements(&self) -> Vec<Coordinate> {
        let mut empties_element: Vec<Coordinate> = Vec::new();
        for (row_index, row) in self.matrix.iter().enumerate() {
            for (col_index, element) in row.iter().enumerate() {
                if element.is_none() {
                    empties_element.push(Coordinate(col_index, row_index));
                }
            }
        }
        empties_element
    }

    pub fn get_corners(&self) -> Vec<Coordinate> {
        let corners: Vec<Coordinate> = vec![
            Coordinate(0, 0),
            Coordinate(0, 2),
            Coordinate(2, 2),
            Coordinate(2, 0),
        ];
        corners
    }

    pub fn get_empty_corners(&self) -> Vec<Coordinate> {
        self.get_corners()
            .into_iter()
            .filter(|c| self.matrix[c.1][c.0].is_none())
            .collect()
    }

    pub fn get_edges(&self) -> Vec<Coordinate> {
        let edges: Vec<Coordinate> = vec![
            Coordinate(1, 0),
            Coordinate(2, 1),
            Coordinate(1, 2),
            Coordinate(0, 1),
        ];
        edges
    }

    pub fn get_empty_edges(&self) -> Vec<Coordinate> {
        self.get_edges()
            .into_iter()
            .filter(|c| self.matrix[c.1][c.0].is_none())
            .collect()
    }

    pub fn get_winning_conditions(
        &self,
    ) -> HashMap<
        (Coordinate, Coordinate, Coordinate),
        (Option<Player>, Option<Player>, Option<Player>),
    > {
        let winning_conditions = HashMap::from([
            (
                (Coordinate(0, 0), Coordinate(1, 0), Coordinate(2, 0)),
                (self.matrix[0][0], self.matrix[0][1], self.matrix[0][2]),
            ),
            (
                (Coordinate(0, 1), Coordinate(1, 1), Coordinate(2, 1)),
                (self.matrix[1][0], self.matrix[1][1], self.matrix[1][2]),
            ),
            (
                (Coordinate(0, 2), Coordinate(1, 2), Coordinate(2, 2)),
                (self.matrix[2][0], self.matrix[2][1], self.matrix[2][2]),
            ),
            (
                (Coordinate(0, 0), Coordinate(0, 1), Coordinate(0, 2)),
                (self.matrix[0][0], self.matrix[1][0], self.matrix[2][0]),
            ),
            (
                (Coordinate(1, 0), Coordinate(1, 1), Coordinate(1, 2)),
                (self.matrix[0][1], self.matrix[1][1], self.matrix[2][1]),
            ),
            (
                (Coordinate(2, 0), Coordinate(2, 1), Coordinate(2, 2)),
                (self.matrix[0][2], self.matrix[1][2], self.matrix[2][2]),
            ),
            (
                (Coordinate(0, 0), Coordinate(1, 1), Coordinate(2, 2)),
                (self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]),
            ),
            (
                (Coordinate(0, 2), Coordinate(1, 1), Coordinate(2, 0)),
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
