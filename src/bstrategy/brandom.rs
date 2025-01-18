use crate::bcoodinate::ValidBinaryCoordinate;
use crate::bstrategy::bstrategy_utils;
use crate::bstrategy::BinaryStrategy;
use crate::butils::get_empty_elements;
use crate::error::Result;
use rand::prelude::*;

pub struct BinaryRandomStrategy {}

impl BinaryStrategy for BinaryRandomStrategy {
    fn get_move(&self, player: bool, board: u32) -> Result<ValidBinaryCoordinate> {
        let empties_element = get_empty_elements(board);
        let mut rng: ThreadRng = thread_rng();
        bstrategy_utils::random_move(empties_element, player, board, &mut rng)
    }
}

impl BinaryRandomStrategy {
    pub fn new() -> Self {
        BinaryRandomStrategy {}
    }
}

impl Default for BinaryRandomStrategy {
    fn default() -> Self {
        Self::new()
    }
}
