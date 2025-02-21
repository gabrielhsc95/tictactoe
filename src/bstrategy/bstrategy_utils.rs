use crate::bcoodinate::{BinaryCoordinate, ValidBinaryCoordinate};
use crate::butils;
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

pub fn win_move_for_player(board: u32, player: bool) -> Option<BinaryCoordinate> {
    let winning_conditions: [[u8; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [2, 5, 8],
        [1, 4, 7],
        [0, 3, 6],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for positions in winning_conditions {
        let v1 = butils::identify_position_state(positions[0], board);
        let v2 = butils::identify_position_state(positions[1], board);
        let v3 = butils::identify_position_state(positions[2], board);
        let player_char: char = if player { 'x' } else { 'o' };
        if v1 == ' ' && v2 == v3 && v2 == player_char {
            return Some(BinaryCoordinate {
                position: positions[0],
                player,
            });
        }
        if v2 == ' ' && v1 == v3 && v1 == player_char {
            return Some(BinaryCoordinate {
                position: positions[1],
                player,
            });
        }
        if v3 == ' ' && v1 == v2 && v1 == player_char {
            return Some(BinaryCoordinate {
                position: positions[2],
                player,
            });
        }
    }
    None
}
