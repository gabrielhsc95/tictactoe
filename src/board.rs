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

    pub fn new() -> Self {
        Board {
            matrix: [[None; 3]; 3],
        }
    }
}
