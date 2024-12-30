use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, ValidCoordinate};
use crate::error::{Error, Result};
use crate::player::Player;
use rand::prelude::*;

pub struct BestStrategy {}

impl Strategy for BestStrategy {
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate> {
        let empty_elements: Vec<Coordinate> = board.get_empties_elements();
        let mut rng: ThreadRng = thread_rng();

        if empty_elements.len() % 2 == 0 {
            self.defensive(empty_elements, &mut rng, board)
        } else {
            self.offensive(empty_elements, &mut rng, board)
        }
    }
}

impl BestStrategy {
    pub fn new() -> Self {
        BestStrategy {}
    }

    fn get_valid_corners(&self, board: &Board) -> Vec<ValidCoordinate> {
        let corners: Vec<Coordinate> = board.get_corners();
        let mut valid_corners: Vec<ValidCoordinate> = Vec::new();
        for c in corners {
            match ValidCoordinate::from(&c, board) {
                Ok(coord) => valid_corners.push(coord),
                Err(_) => {}
            }
        }
        valid_corners
    }

    fn get_one_random_valid_corner(
        &self,
        board: &Board,
        rng: &mut ThreadRng,
    ) -> Result<ValidCoordinate> {
        let valid_corners: Vec<ValidCoordinate> = self.get_valid_corners(board);
        let coordinate: Option<&ValidCoordinate> = valid_corners.choose(rng);
        match coordinate {
            Some(c) => Ok(c.clone()),
            None => Err(Error::StrategyInvalidMove(String::from(
                "There must be corner available!",
            ))),
        }
    }

    fn get_first_play(&self, board: &Board) -> Coordinate {
        // don't use this function if it is not the second play of the CPU
        let corners: Vec<Coordinate> = board.get_corners();
        self.find_x(corners, board)
    }

    fn find_x(&self, coordinates: Vec<Coordinate>, board: &Board) -> Coordinate {
        for c in coordinates {
            let on_the_board = board.matrix[c.1][c.0];
            match on_the_board {
                Some(p) => {
                    if p == Player::X {
                        return c;
                    }
                }
                None => {}
            }
        }
        unreachable!("It should not be here!");
    }

    fn opposite_value(&self, index: usize) -> usize {
        match index {
            0 => 2,
            2 => 0,
            _ => index,
        }
    }

    fn win_move_for_player(&self, board: &Board, player: Player) -> Option<Coordinate> {
        let winning_conditions = board.get_winning_conditions();
        for ((c0, c1, c2), (v0, v1, v2)) in winning_conditions {
            let num: u8 = self.count_player((v0, v1, v2), player);
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

    fn count_player(
        &self,
        condition: (Option<Player>, Option<Player>, Option<Player>),
        player: Player,
    ) -> u8 {
        let mut num: u8 = 0;
        for c in [condition.0, condition.1, condition.2] {
            if Some(player) == c {
                num += 1;
            }
        }
        num
    }

    fn random_move(&self, empty_elements: Vec<Coordinate>, rng: &mut ThreadRng) -> Coordinate {
        let coordinate = empty_elements
            .choose(rng)
            .expect("There should be one last valid coordinate!");
        return coordinate.clone();
    }

    fn offensive(
        &self,
        empty_elements: Vec<Coordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<ValidCoordinate> {
        if empty_elements.len() == 9 {
            // first play
            // random corner
            return self.get_one_random_valid_corner(board, rng);
        } else if empty_elements.len() == 7 {
            // second play
            let first_play: Coordinate = self.get_first_play(board);
            if !board.matrix[1][1].is_none() {
                // second player chose middle
                // opposite corner
                return ValidCoordinate::new(
                    self.opposite_value(first_play.0),
                    self.opposite_value(first_play.1),
                    board,
                );
            } else {
                // adjacent corner that is not blocked
                let opposite_first_play_0 = self.opposite_value(first_play.0);
                let opposite_first_play_1 = self.opposite_value(first_play.1);
                if board.matrix[1][first_play.0].is_none()
                    && board.matrix[opposite_first_play_1][first_play.0].is_none()
                {
                    // between the first_play and (first_play.0, opposite_first_play_1),
                    // which is (first_play.0, 1) is free,
                    // but also (first_play.0, opposite_first_play_1),
                    // so we can play there
                    return ValidCoordinate::new(first_play.0, opposite_first_play_1, board);
                } else if board.matrix[1][opposite_first_play_0].is_none()
                    && board.matrix[first_play.1][opposite_first_play_0].is_none()
                {
                    // if blocked then the other adjacent is (opposite_first_play_0, first_play.1),
                    // but we still need to check that corner, which is (opposite_first_play_0, first_play.1)
                    return ValidCoordinate::new(opposite_first_play_0, first_play.1, board);
                } else {
                    return self.get_one_random_valid_corner(board, rng);
                }
            }
        } else if empty_elements.len() == 5 {
            // third play
            let win_move = self.win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                // else not lose or another corner
                None => {
                    let not_lose_move = self.win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => {
                            let valid_corners = self.get_valid_corners(board);
                            for vc in valid_corners {
                                if board.matrix[1][vc.x()].is_none()
                                    && board.matrix[vc.y()][1].is_none()
                                {
                                    return Ok(vc);
                                }
                            }
                            unreachable!("There must be a valid corner with no adjacent elements!");
                        }
                    }
                }
            }
        } else if empty_elements.len() == 3 {
            // forth play
            let win_move = self.win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                None => {
                    // else not lose or draw so random
                    let not_lose_move = self.win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => {
                            let m: Coordinate = self.random_move(empty_elements, rng);
                            return ValidCoordinate::from(&m, board);
                        }
                    }
                }
            }
        } else {
            // last play
            // random
            let m: Coordinate = self.random_move(empty_elements, rng);
            return ValidCoordinate::from(&m, board);
        }
    }
    fn defensive(
        &self,
        empty_elements: Vec<Coordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<ValidCoordinate> {
        if empty_elements.len() == 8 {
            // first play
            if board.matrix[1][1].is_none() {
                return ValidCoordinate::new(1, 1, board);
            } else {
                let m: Coordinate = self.random_move(empty_elements, rng);
                return ValidCoordinate::from(&m, board);
            }
        } else if empty_elements.len() == 6 {
            // second play
        } else if empty_elements.len() == 4 {
            // third play
        } else if empty_elements.len() == 2 {
            // forth play
        } else {
            // last play
        }
        panic!("Not implemented yet!");
    }
}
