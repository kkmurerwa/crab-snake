pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for PlayerDirection {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PlayerDirection::Up, PlayerDirection::Up) => true,
            (PlayerDirection::Down, PlayerDirection::Down) => true,
            (PlayerDirection::Left, PlayerDirection::Left) => true,
            (PlayerDirection::Right, PlayerDirection::Right) => true,
            _ => false,
        }
    }
}
