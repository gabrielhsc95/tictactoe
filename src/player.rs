#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn to_string(&self) -> String {
        match self {
            Player::X => String::from("x"),
            Player::O => String::from("o"),
        }
    }
}
