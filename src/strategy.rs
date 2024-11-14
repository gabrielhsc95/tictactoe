pub mod best;
pub mod random;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Result;

pub trait Strategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate>;
    fn get_input(&self, board: &Board) -> Coordinate {
        match self.get_move(board) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        }
    }
}
