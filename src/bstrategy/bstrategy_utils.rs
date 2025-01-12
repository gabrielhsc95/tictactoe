use crate::bcoodinate::ValidBinaryCoordinate;
use crate::error::{Error, Result};
use rand::prelude::*;

pub fn random_move(
    options: Vec<u8>,
    player: bool,
    board: u32,
    rng: &mut ThreadRng,
) -> Result<ValidBinaryCoordinate> {
    match options.choose(rng) {
        Some(position) => ValidBinaryCoordinate::new(*position, player, board),
        None => Err(Error::StrategyInvalidMove(String::from(
            "A random_move was called without any option to choose from!",
        ))),
    }
}
