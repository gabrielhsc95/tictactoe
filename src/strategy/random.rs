use super::Strategy;
use crate::board::Board;
use crate::coordinates::Coordinates;
use rand::prelude::*;

pub struct RandomStrategy {}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}

impl Strategy for RandomStrategy {
    fn get_input(&self, board: &Board) -> Coordinates {
        let options: Vec<Coordinates> = board.get_empties_elements();
        let mut rng = thread_rng();
        let random_coordinate = options.choose(&mut rng);
        match random_coordinate {
            Some(coordinate) => Coordinates(coordinate.0, coordinate.1),
            None => unreachable!(
                "There must be coordinate available, otherwise the game should have ended."
            ),
        }
    }
}
