//! Handles the coordinates for the board.

use crate::board::Board;
use crate::error::Error;
use crate::error::Result;
use std::fmt;
/// Coordinate is a pair of usize.
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
/// use tictactoe::coordinate::{Coordinate, ValidCoordinate};
/// use tictactoe::error::Result;
///
/// let board = Board::default();
/// let coordinate = Coordinate(42, 42);
/// assert!(ValidCoordinate::from(&coordinate, &board).is_err());
/// let coordinate = Coordinate(0, 0);
/// assert!(ValidCoordinate::from(&coordinate, &board).is_ok());
/// ```
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ValidCoordinate(usize, usize);

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

impl ValidCoordinate {
    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn new(x: usize, y: usize, board: &Board) -> Result<Self> {
        if x > 2 {
            return Err(Error::CoordinateNumberInvalid {
                dimension: Dimension::X,
            });
        }
        if y > 2 {
            return Err(Error::CoordinateNumberInvalid {
                dimension: Dimension::Y,
            });
        }
        match board.matrix[y][x] {
            Some(p) => Err(Error::CoordinateOccupied { occupied_by: p }),
            None => Ok(ValidCoordinate(x, y)),
        }
    }

    pub fn from(coordinate: &Coordinate, board: &Board) -> Result<Self> {
        ValidCoordinate::new(coordinate.0, coordinate.1, board)
    }
}
