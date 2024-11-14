//! Handles the coordinates in a board.

/// Coordinate system used in the board.
///
/// First is x and then y. It is usize, to more easily access
/// the matrix elements of the board.
///
/// 0,0 means the top left element.
///
/// ```
/// let c = Coordinate(0, 0);
/// ```
use crate::board::Board;
use crate::error::Error;
use crate::error::Result;
use std::fmt;
#[derive(Eq, PartialEq, Hash)]
pub struct Coordinate(pub usize, pub usize);

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
            Some(p) => return Err(Error::CoordinateOccupied { occupied_by: p }),
            None => Ok(Coordinate(x, y)),
        }
    }
}
