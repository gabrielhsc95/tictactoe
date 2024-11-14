use super::Strategy;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::{Error, Result};
use rand::prelude::*;

pub struct RandomStrategy {}

impl Strategy for RandomStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let options: Vec<Coordinate> = board.get_empties_elements();
        let mut rng = thread_rng();
        let random_coordinate = options.choose(&mut rng);
        match random_coordinate {
            Some(coordinate) => Ok(Coordinate(coordinate.0, coordinate.1)),
            None => Err(Error::StrategyInvalidMove(String::from(
                "There must be coordinate available, otherwise the game should have ended.",
            ))),
        }
    }
}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}
