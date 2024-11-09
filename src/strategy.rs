pub mod best;
pub mod random;
use crate::board::Board;
use crate::coordinate::Coordinate;

pub trait Strategy {
    fn get_input(&self, board: &Board) -> Coordinate;
}
