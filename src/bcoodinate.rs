use crate::butils::{identify_position_player, xy_to_position};
use crate::error::{Error, Result};

pub struct BinaryCoordinate {
    pub position: u8,
    pub player: bool,
}

impl BinaryCoordinate {
    pub fn new(position: u8, player: bool) -> Self {
        BinaryCoordinate { position, player }
    }
}

pub struct ValidBinaryCoordinate {
    pub position: u8,
    pub player: bool,
}

impl ValidBinaryCoordinate {
    pub fn new(position: u8, player: bool, board: u32) -> Result<Self> {
        if position > 8 {
            return Err(Error::BinaryCoordinatePositionInvalid);
        }
        if board & (0b11 << ((position as u32) << 1)) == 0 {
            Ok(ValidBinaryCoordinate { position, player })
        } else {
            let occupied_by = identify_position_player(position, board);
            Err(Error::BinaryCoordinateOccupied { occupied_by })
        }
    }

    pub fn from(binary_coordinate: &BinaryCoordinate, board: u32) -> Result<Self> {
        ValidBinaryCoordinate::new(binary_coordinate.position, binary_coordinate.player, board)
    }

    pub fn from_xy(x: usize, y: usize, current_player: bool, board: u32) -> Result<Self> {
        let position = xy_to_position(x, y);
        ValidBinaryCoordinate::new(position, current_player, board)
    }
}
