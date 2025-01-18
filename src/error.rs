//! Error from tictactoe
use derive_more::From;

use crate::{coordinate::Dimension, player::Player};

/// Wrapper around Result<T, E>, so every E is a create error.
pub type Result<T> = std::result::Result<T, Error>;
/// Errors for the whole game
#[derive(Debug, From)]
pub enum Error {
    // Coordinate
    CoordinateNumberInvalid {
        dimension: Dimension,
    },
    CoordinateFormatInvalid,
    BinaryCoordinatePositionInvalid,
    CoordinateOccupied {
        occupied_by: Player,
    },
    BinaryCoordinateOccupied {
        occupied_by: bool,
    },
    // }
    StrategyInvalidMove(String),
    // External
    #[from]
    Io(std::io::Error),
    #[from]
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CoordinateNumberInvalid { dimension } => {
                write!(f, "Invalid {dimension} input. Numbers must be 0, 1, or 2.")
            }
            Error::CoordinateFormatInvalid => write!(
                f,
                "Invalid input. Please enter two numbers separated by a comma."
            ),
            Error::CoordinateOccupied { occupied_by } => write!(
                f,
                "This is already occupied by {occupied_by}, please pick another place!"
            ),
            Error::BinaryCoordinatePositionInvalid => {
                write!(f, "Invalid input. Position must be between 0 and 8.")
            }
            Error::BinaryCoordinateOccupied { occupied_by } => {
                let player: char = if *occupied_by { 'x' } else { 'o' };
                write!(
                    f,
                    "This is already occupied by {player}, please pick another place!"
                )
            }
            Error::StrategyInvalidMove(s) => write!(f, "{s}"),
            Error::Io(e) => write!(f, "{e}"),
            Error::Parse(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}
