//! CPU Strategies.
pub mod best;
pub mod medium;
pub mod random;
pub mod utils;
use crate::board::Board;
use crate::coordinate::ValidCoordinate;
use crate::error::Result;

/// A trait to implement a valid strategy.
///
/// Strategy trait requires that any strategy that is implemented returns a Coordinate.
/// You only need to implement get_move that returns `Result<Coordinate>`, if results in an
/// Error get_input will make the code panic. get_input also calls is_valid from Coordinate
/// to make sure it is a valid move, otherwise, it will also panic.
///
/// A user can make mistakes, and we can prompt them to correct, but the CPU has always
/// to return a valid move. On integration tests all strategies should run against random
/// 1000 times to make sure it doesn't panic.
///
/// # Example:
///
/// ```
/// use tictactoe::board::Board;
/// use tictactoe::coordinate::{Coordinate, ValidCoordinate};
/// use tictactoe::strategy::Strategy;
/// use tictactoe::error::Result;
///
/// struct ExampleStrategy{}
///
/// impl Strategy for ExampleStrategy {
///     fn get_move(&self, board: &Board) -> Result<ValidCoordinate> {
///         let c = Coordinate(1,1);
///         ValidCoordinate::from(&c, board)
///     }
/// }
/// let board = Board::new();
/// let example_strategy = ExampleStrategy{};
/// example_strategy.get_input(&board);
/// ```
pub trait Strategy {
    /// Returns the next move of a strategy
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate>;
    /// Returns a valid Coordinate
    fn get_input(&self, board: &Board) -> ValidCoordinate {
        match self.get_move(board) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        }
    }
}
