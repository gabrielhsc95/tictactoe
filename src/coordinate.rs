//! Handles the coordinates in a board.

/// Coordinate is a pair of usize. Technically speaking the full length usize [0..2^64).
///
/// The first element is x and then y. It is usize, to more easily access the matrix elements
/// of the board. 0,0 means the top left element.
///
/// However to constrains it to the tictactoe game, we have ValidCoordinate, that takes into consideration the current state of the board.
/// Therefore, it can only be created if that position exists ([0..2] for both x and y) and it is empty (available)
use crate::board::Board;
use crate::error::Error;
use crate::error::Result;
use std::fmt;
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ValidCoordinate(usize, usize);

#[derive(Eq, PartialEq, Hash, Debug)]
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
            Some(p) => return Err(Error::CoordinateOccupied { occupied_by: p }),
            None => Ok(ValidCoordinate(x, y)),
        }
    }

    pub fn from(coordinate: &Coordinate, board: &Board) -> Result<Self> {
        ValidCoordinate::new(coordinate.0, coordinate.1, board)
    }
}
