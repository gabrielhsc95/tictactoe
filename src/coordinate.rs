//! Handles the coordinates in a board.
/// Coordinate system used in the board.
///
/// First is x and then y. It is usize, to more easily access
/// the matrix elements of the board.
///
/// 0,0 means the top left element.
///
/// ```
/// let c = Coordinate(0, 0);
/// ```
#[derive(Eq, PartialEq, Hash)]
pub struct Coordinate(pub usize, pub usize);
