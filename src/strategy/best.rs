//! Best Strategy, it never loses.
use super::Strategy;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Result;
use crate::player::Player;
use crate::strategy::utils;
use rand::prelude::*;

pub struct BestStrategy {}

impl Strategy for BestStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let empty_elements: Vec<Coordinate> = board.get_empty_elements();
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
    /// Back tracks what was done as a first move
    ///
    /// Do not use this function if it is not the second play of the CPU
    fn get_first_play(&self, board: &Board) -> Coordinate {
        let corners: Vec<Coordinate> = board.get_corners();
        self.find_x(corners, board)
    }

    /// Find the first Player X for given Vec<Coordinate>
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

    /// Return the opposite coordinate
    fn opposite_value(&self, index: usize) -> usize {
        match index {
            0 => 2,
            2 => 0,
            _ => index,
        }
    }

    /// Plays offensive move, it starts
    fn offensive(
        &self,
        empty_elements: Vec<Coordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<Coordinate> {
        if empty_elements.len() == 9 {
            // first play
            // random corner
            let empty_corners = board.get_empty_corners();
            return utils::random_move(empty_corners, rng);
        } else if empty_elements.len() == 7 {
            // second play
            let first_play: Coordinate = self.get_first_play(board);
            if !board.matrix[1][1].is_none() {
                // second player chose middle
                // opposite corner
                return Ok(Coordinate(
                    self.opposite_value(first_play.0),
                    self.opposite_value(first_play.1),
                ));
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
                    return Ok(Coordinate(first_play.0, opposite_first_play_1));
                } else if board.matrix[1][opposite_first_play_0].is_none()
                    && board.matrix[first_play.1][opposite_first_play_0].is_none()
                {
                    // if blocked then the other adjacent is (opposite_first_play_0, first_play.1),
                    // but we still need to check that corner, which is (opposite_first_play_0, first_play.1)
                    return Ok(Coordinate(opposite_first_play_0, first_play.1));
                } else {
                    let valid_corners = board.get_empty_corners();
                    return utils::random_move(valid_corners, rng);
                }
            }
        } else if empty_elements.len() == 5 {
            // third play
            let win_move = utils::win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => Ok(w),
                // else not lose or another corner
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => Ok(l),
                        None => {
                            let empty_corners = board.get_empty_corners();
                            for c in empty_corners {
                                if board.matrix[1][c.0].is_none() && board.matrix[c.1][1].is_none()
                                {
                                    return Ok(c);
                                }
                            }
                            unreachable!("There must be a valid corner with no adjacent elements!");
                        }
                    }
                }
            }
        } else if empty_elements.len() == 3 {
            // forth play
            let win_move = utils::win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => Ok(w),
                None => {
                    // else not lose or draw so random
                    let not_lose_move = utils::win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => Ok(l),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            }
        } else {
            // last play
            // random move
            utils::random_move(empty_elements, rng)
        }
    }

    /// Play defensive mode, the enemy starts.
    fn defensive(
        &self,
        empty_elements: Vec<Coordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<Coordinate> {
        if empty_elements.len() == 8 {
            // first play
            // try to play middle
            if board.matrix[1][1].is_none() {
                return Ok(Coordinate(1, 1));
            } else {
                // random if not random corner
                let empty_corners = board.get_empty_corners();
                return utils::random_move(empty_corners, rng);
            }
        } else if empty_elements.len() == 6 {
            // second play
            // not lose
            let not_lose_move = utils::win_move_for_player(board, Player::X);
            match not_lose_move {
                Some(l) => Ok(l),
                None => {
                    let has_middle = match board.matrix[1][1] {
                        Some(player) => {
                            if player == Player::O {
                                true
                            } else {
                                false
                            }
                        }
                        None => false,
                    };
                    if has_middle {
                        let empty_corners = board.get_empty_corners();
                        if empty_corners.len() == 2 {
                            let empty_edges = board.get_empty_edges();
                            return utils::random_move(empty_edges, rng);
                        } else {
                            for c in empty_corners {
                                let adjacent_1 = board.matrix[1][c.0];
                                let adjacent_2 = board.matrix[c.1][1];

                                if adjacent_1.is_some() || adjacent_2.is_some() {
                                    return Ok(c);
                                }
                            }
                            unreachable!("There should be an available corner at this point!");
                        }
                    } else {
                        let empty_corners = board.get_empty_corners();
                        return utils::random_move(empty_corners, rng);
                    }
                }
            }
        } else if empty_elements.len() == 4 {
            // third play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => Ok(w),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => Ok(l),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        } else if empty_elements.len() == 2 {
            // forth play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => Ok(w),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => Ok(l),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        } else {
            // last play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => Ok(w),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => Ok(l),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        }
    }
}
