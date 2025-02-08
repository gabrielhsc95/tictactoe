//! It will play random until it finds a winning condition
use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, ValidCoordinate};
use crate::error::{Error, Result};
use crate::player::Player;
use crate::strategy::utils;
use rand::prelude::*;

pub struct MediumStrategy {}

impl Strategy for MediumStrategy {
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate> {
        let empty_elements: Vec<ValidCoordinate> = board.get_empty_elements();
        let mut rng: ThreadRng = thread_rng();
        let win_move: Option<Coordinate> = if empty_elements.len() % 2 == 0 {
            utils::win_move_for_player(board, Player::O)
        } else {
            utils::win_move_for_player(board, Player::X)
        };
        match win_move {
            Some(w) => ValidCoordinate::from(&w, board),
            None => {
                let random_coordinate: Option<&ValidCoordinate> = empty_elements.choose(&mut rng);
                match random_coordinate {
                    Some(coordinate) => Ok(coordinate.clone()),
                    None => Err(Error::StrategyInvalidMove(String::from(
                        "There must be coordinate available, otherwise the game should have ended.",
                    ))),
                }
            }
        }
    }
}

impl MediumStrategy {
    pub fn new() -> Self {
        MediumStrategy {}
    }
}

impl Default for MediumStrategy {
    fn default() -> Self {
        Self::new()
    }
}
