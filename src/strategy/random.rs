use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, ValidCoordinate};
use crate::error::{Error, Result};
use rand::prelude::*;

pub struct RandomStrategy {}

impl Strategy for RandomStrategy {
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate> {
        let options: Vec<Coordinate> = board.get_empties_elements();
        println!("{options:?}");
        let mut rng: ThreadRng = thread_rng();
        let random_coordinate: Option<&Coordinate> = options.choose(&mut rng);
        match random_coordinate {
            Some(coordinate) => ValidCoordinate::from(coordinate, board),
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
