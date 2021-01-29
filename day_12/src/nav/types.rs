use std::ops::{AddAssign, Mul};

pub type Error = &'static str;
pub type Distance = i32;

#[derive(PartialEq, Clone, Debug)]
pub struct Position(pub i32, pub i32); // east, north

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Angle {
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Debug)]
pub enum Instruction {
    Move(Cardinal, Distance),
    Turn(Direction, Angle),
    Forward(Distance),
}
