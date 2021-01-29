use std::ops::AddAssign;

pub type Error = &'static str;
pub type Distance = i32;

#[derive(PartialEq, Debug)]
pub struct Position(pub i32, pub i32);

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

pub enum Direction {
    Left,
    Right,
}

pub enum Angle {
    Ninety,
    OneEighty,
    TwoSeventy,
}

pub enum Instruction {
    Move(Cardinal, Distance),
    Turn(Direction, Angle),
    Forward(Distance),
}
