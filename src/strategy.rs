pub mod best;
pub mod medium;
pub mod random;
pub mod utils;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Result;

// Strategy trait requires that any strategy that is implemented returns a ValidCoordinate,
// otherwise, it will panic. You only need to implement get_move that returns Result<ValidCoordinate>,
// matching the result is already handled by the trait in get_input, which should be the function that
// user interfaces should call.
pub trait Strategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate>;
    fn get_input(&self, board: &Board) -> Coordinate {
        match self.get_move(board) {
            Ok(c) => match c.is_valid(board) {
                Ok(_) => c,
                Err(e) => panic!("{e}"),
            },
            Err(e) => panic!("{e}"),
        }
    }
}
