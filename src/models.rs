use std::fmt;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
            Direction::Up => write!(f, "up"),
        }
    }
}

#[derive(PartialEq)]
pub enum Outcome {
    Dead,
    Alive,
}

pub struct DirectionResult {
    pub direction: Direction, 
    pub outcome: Outcome, 
    pub other_data: i32,
}

