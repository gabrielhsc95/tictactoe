use crate::bcoodinate::ValidBinaryCoordinate;
use crate::bstrategy::bstrategy_utils;
use crate::bstrategy::BinaryStrategy;
use crate::butils::get_empty_elements;
use crate::error::Result;
use rand::prelude::*;

pub struct BinaryMediumStrategy {}

impl BinaryStrategy for BinaryMediumStrategy {
    fn get_move(&self, player: bool, board: u32) -> Result<ValidBinaryCoordinate> {
        let win_move = bstrategy_utils::win_move_for_player(board, player);
        match win_move {
            Some(w) => ValidBinaryCoordinate::from(&w, board),
            None => {
                let empties_element = get_empty_elements(board);
                let mut rng: ThreadRng = thread_rng();
                bstrategy_utils::random_move(empties_element, player, board, &mut rng)
            }
        }
    }
}

impl BinaryMediumStrategy {
    pub fn new() -> Self {
        BinaryMediumStrategy {}
    }
}

impl Default for BinaryMediumStrategy {
    fn default() -> Self {
        Self::new()
    }
}
