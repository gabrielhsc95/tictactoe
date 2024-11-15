use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, SafeCoordinate};
use crate::error::{Error, Result};
use crate::player::Player;
use rand::prelude::*;

pub struct MediumStrategy {}

impl Strategy for MediumStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let win_move = self.win_move(board);
        match win_move {
            Some(w) => Coordinate::from(&w, board),
            None => {
                let options: Vec<SafeCoordinate> = board.get_empties_elements();
                let mut rng: ThreadRng = thread_rng();
                let random_coordinate: Option<&SafeCoordinate> = options.choose(&mut rng);
                match random_coordinate {
                    Some(coordinate) => Coordinate::from(coordinate, board),
                    None => Err(Error::StrategyInvalidMove(String::from(
                        "There must be coordinate available, otherwise the game should have ended.",
                    ))),
                }
            }
        }
    }
}

impl MediumStrategy {
    fn win_move(&self, board: &Board) -> Option<SafeCoordinate> {
        let winning_conditions = board.get_winning_conditions();
        for ((c0, c1, c2), (v0, v1, v2)) in winning_conditions {
            let num: u8 = self.count_x((v0, v1, v2));
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

    fn count_x(&self, condition: (Option<Player>, Option<Player>, Option<Player>)) -> u8 {
        let mut num: u8 = 0;
        for c in [condition.0, condition.1, condition.2] {
            if Some(Player::X) == c {
                num += 1;
            }
        }
        num
    }

    pub fn new() -> Self {
        MediumStrategy {}
    }
}
