//! Completely random play
use super::Strategy;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Result;
use crate::strategy::utils;
use rand::prelude::*;

pub struct RandomStrategy {}

impl Strategy for RandomStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let empty_elements: Vec<Coordinate> = board.get_empty_elements();
        let mut rng: ThreadRng = thread_rng();
        utils::random_move(empty_elements, &mut rng)
    }
}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}
