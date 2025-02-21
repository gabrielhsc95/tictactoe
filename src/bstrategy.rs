use crate::bcoodinate::ValidBinaryCoordinate;
use crate::error::Result;
pub mod bbest;
pub mod bmedium;
pub mod brandom;
pub mod bstrategy_utils;

pub trait BinaryStrategy {
    /// Returns the next move of a strategy
    fn get_move(&self, player: bool, board: u32) -> Result<ValidBinaryCoordinate>;
    /// Returns a valid Coordinate
    fn get_input(&self, player: bool, board: u32) -> ValidBinaryCoordinate {
        match self.get_move(player, board) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        }
    }
}
