//! It will play random until it finds a winning condition
use super::Strategy;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Result;
use crate::player::Player;
use crate::strategy::utils;
use rand::prelude::*;

pub struct MediumStrategy {}

impl Strategy for MediumStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let empty_elements: Vec<Coordinate> = board.get_empty_elements();
        let mut rng: ThreadRng = thread_rng();
        let win_move: Option<Coordinate>;
        if empty_elements.len() % 2 == 0 {
            win_move = utils::win_move_for_player(board, Player::O);
        } else {
            win_move = utils::win_move_for_player(board, Player::X);
        }
        match win_move {
            Some(w) => Ok(w),
            None => {
                let options: Vec<Coordinate> = board.get_empty_elements();
                utils::random_move(options, &mut rng)
            }
        }
    }
}

impl MediumStrategy {
    pub fn new() -> Self {
        MediumStrategy {}
    }
}
