//! Handles the coordinates in a board.
/// Coordinate system used in the board.
///
/// First is x and then y. It is usize, to more easily access
/// the matrix elements of the board.
///
/// 0,0 means the top left element.
///
/// ```
/// let c = Coordinates(0, 0);
/// ```
pub struct Coordinates(pub usize, pub usize);
