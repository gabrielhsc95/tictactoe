//! Shared utilities for strategies
use crate::board::Board;
use crate::coordinate::{Coordinate, ValidCoordinate};
use crate::error::{Error, Result};
use crate::player::Player;
use rand::prelude::*;

/// Count how many of a given player is is condition
fn count_player(condition: (Option<Player>, Option<Player>, Option<Player>), player: Player) -> u8 {
    let mut num: u8 = 0;
    for c in [condition.0, condition.1, condition.2] {
        if Some(player) == c {
            num += 1;
        }
    }
    num
}

/// Returns the winning move for a given player, otherwise, None
pub fn win_move_for_player(board: &Board, player: Player) -> Option<Coordinate> {
    let winning_conditions = board.get_winning_conditions();
    for ((c0, c1, c2), (v0, v1, v2)) in winning_conditions {
        let num: u8 = count_player((v0, v1, v2), player);
        if num == 2 {
            if v0.is_none() {
                return Some(c0);
            } else if v1.is_none() {
                return Some(c1);
            } else if v2.is_none() {
                return Some(c2);
            }
        }
    }
    None
}

pub fn random_move(options: Vec<ValidCoordinate>, rng: &mut ThreadRng) -> Result<ValidCoordinate> {
    match options.choose(rng) {
        Some(coordinate) => Ok(coordinate.clone()),
        None => Err(Error::StrategyInvalidMove(String::from(
            "A random_move was called without any option to choose from!",
        ))),
    }
}
