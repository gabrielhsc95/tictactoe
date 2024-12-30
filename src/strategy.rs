pub mod best;
pub mod medium;
pub mod random;
use crate::board::Board;
use crate::coordinate::ValidCoordinate;
use crate::error::Result;

// Strategy trait requires that any strategy that is implemented returns a ValidCoordinate,
// otherwise, it will panic. You only need to implement get_move that returns Result<ValidCoordinate>,
// matching the result is already handled by the trait in get_input, which should be the function that
// user interfaces should call.
pub trait Strategy {
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate>;
    fn get_input(&self, board: &Board) -> ValidCoordinate {
        match self.get_move(board) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        }
    }
}
