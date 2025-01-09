//! Players representation (X or O)
use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Player::X => write!(f, "x"),
            Player::O => write!(f, "o"),
        }
    }
}
