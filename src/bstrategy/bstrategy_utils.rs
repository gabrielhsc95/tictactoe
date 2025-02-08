use crate::bcoodinate::{BinaryCoordinate, ValidBinaryCoordinate};
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

fn brian_kernighan(mut masked_board: u32) -> u8 {
    let mut counter: u8 = 0;
    while masked_board != 0 {
        masked_board = masked_board & (masked_board - 1);
        counter += 1;
    }
    counter

}

fn get_missing_position(positions: [u8; 3], masked_board: u32) -> u8 {
    for p in positions {
        if masked_board & (0b11 << (p << 1)) == 0 {
            return p
        }
    }
    unreachable!("It should not be here!")
}


pub fn win_move_for_player(board: u32, player: bool) -> Option<BinaryCoordinate>{
    let winning_conditions: [([u8; 3], u32); 8] = [
        ([0, 1, 2], 0b000000000000111111),
        ([3, 4, 5], 0b000000111111000000),
        ([6, 7, 8], 0b111111000000000000),
        ([2, 5, 8], 0b110000110000110000),
        ([1, 4, 7], 0b001100001100001100),
        ([0, 3, 6], 0b000011000011000011),
        ([0, 4, 8], 0b110000001100000011),
        ([2, 4, 6], 0b000011001100110000),
    ];
    for (positions, wc) in winning_conditions{
        let masked_board = board & wc ;
        if brian_kernighan(masked_board) == 2 {
            return Some(BinaryCoordinate{
                position: get_missing_position(positions, masked_board),
                player
            })
        }
    }
    None
}
