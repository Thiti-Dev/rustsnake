#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Check if player is trying to make an invalid moves or not
    pub fn is_opposite(self, other: Direction) -> bool {
        self == Direction::Up && other == Direction::Down
            || self == Direction::Down && other == Direction::Up
            || self == Direction::Left && other == Direction::Right
            || self == Direction::Right && other == Direction::Left
    }
}
