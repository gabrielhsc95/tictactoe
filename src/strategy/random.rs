use super::Strategy;
use crate::board::Board;
use crate::coordinate::Coordinate;
use rand::prelude::*;

pub struct RandomStrategy {}

impl Strategy for RandomStrategy {
    fn get_input(&self, board: &Board) -> Coordinate {
        let options: Vec<Coordinate> = board.get_empties_elements();
        let mut rng = thread_rng();
        let random_coordinate = options.choose(&mut rng);
        match random_coordinate {
            Some(coordinate) => Coordinate(coordinate.0, coordinate.1),
            None => unreachable!(
                "There must be coordinate available, otherwise the game should have ended."
            ),
        }
    }
}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}
