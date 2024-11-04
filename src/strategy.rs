pub mod random;
use crate::board::Board;
use crate::coordinates::Coordinates;

pub trait Strategy {
    fn get_input(&self, board: &Board) -> Coordinates;
}
