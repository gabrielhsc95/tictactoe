//! Handles the coordinates for the board.

use crate::board::Board;
use crate::error::Error;
use crate::error::Result;
use std::fmt;
/// Coordinate is a pair of usize.
///
///
/// Technically speaking the full length usize [0..2^64). The first element is x and then y.
/// It is usize, to more easily access the matrix elements of the board. 0,0 means the top left element.
///
/// However to constrains it to the tictactoe board, we have the function is_valid, that takes into
/// consideration that it should be [0, 2] for both x and y; in addition, it also considers the current
/// state of the board to make sure it is empty. Since this is mostly use to pass the information of
/// the next move.
///
/// # Example
/// ```
/// use tictactoe::board::Board;
/// use tictactoe::coordinate::Coordinate;
/// use tictactoe::error::Result;
///
/// let board = Board::new();
/// let coordinate = Coordinate(42, 42);
/// assert!(coordinate.is_valid(&board).is_err());
/// let coordinate = Coordinate(0, 0);
/// assert!(coordinate.is_valid(&board).is_ok());
/// ```
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Coordinate(pub usize, pub usize);

/// Just to name the dimension for better error messages
#[derive(Debug)]
pub enum Dimension {
    X,
    Y,
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Dimension::X => write!(f, "x"),
            Dimension::Y => write!(f, "y"),
        }
    }
}

impl Coordinate {
    /// Check if the Coordinate is valid on the inputs ([0, 2] for both x and y)
    /// and if the board is board is empty at that position.
    pub fn is_valid(&self, board: &Board) -> Result<()> {
        if self.0 > 2 {
            return Err(Error::CoordinateNumberInvalid {
                dimension: Dimension::X,
            });
        }
        if self.1 > 2 {
            return Err(Error::CoordinateNumberInvalid {
                dimension: Dimension::Y,
            });
        }
        match board.matrix[self.1][self.0] {
            Some(p) => return Err(Error::CoordinateOccupied { occupied_by: p }),
            None => Ok(()),
        }
    }
}
