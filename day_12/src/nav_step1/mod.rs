mod types;
use types::{Instruction::*, *};

pub use types::Position;

const CARDINALS_CW: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::East,
    Cardinal::South,
    Cardinal::West,
];
const CARDINALS_CCW: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::West,
    Cardinal::South,
    Cardinal::East,
];

pub struct Nav {
    facing: Cardinal,
    position: Position, // east, north
    instructions: Vec<Instruction>,
    step: usize,
}

impl Nav {
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to read instructions")?;
        let instructions = file
            .lines()
            .map(Self::parse_instruction)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            facing: Cardinal::East,
            position: Position(0, 0),
            instructions,
            step: 0,
        })
    }

    pub fn manhattan_distance(a: &Position, b: &Position) -> u32 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
    }

    pub fn navigate(&mut self) -> &Position {
        while let Ok(_) = self.move_ship() {}
        &self.position
    }

    fn move_ship(&mut self) -> Result<(), Error> {
        println!("Pos: {:?}, facing {:?}", self.position, self.facing);

        let instruction = self
            .instructions
            .get(self.step)
            .ok_or("Out of instructions")?;

        match instruction {
            Move(cardinal, distance) => {
                self.position += Self::cardinal_offset(&cardinal, *distance)
            }
            Turn(direction, angle) => {
                self.facing = Self::facing_offset(&self.facing, &direction, &angle)
            }
            Forward(distance) => self.position += Self::cardinal_offset(&self.facing, *distance),
        }

        self.step += 1;
        Ok(())
    }

    fn cardinal_offset(cardinal: &Cardinal, distance: Distance) -> Position {
        match cardinal {
            Cardinal::North => Position(0, distance),
            Cardinal::South => Position(0, -distance),
            Cardinal::East => Position(distance, 0),
            Cardinal::West => Position(-distance, 0),
        }
    }

    fn facing_offset(facing: &Cardinal, direction: &Direction, angle: &Angle) -> Cardinal {
        let offset = match angle {
            Angle::Ninety => 1,
            Angle::OneEighty => 2,
            Angle::TwoSeventy => 3,
        };

        let cardinals = match direction {
            Direction::Right => &CARDINALS_CW,
            Direction::Left => &CARDINALS_CCW,
        };

        let current = cardinals.iter().position(|item| item == facing).unwrap();
        let desired = (current + offset) % 4;
        cardinals[desired]
    }

    fn parse_instruction(line: &str) -> Result<Instruction, Error> {
        let (action, arg) = line.split_at(1);
        match action {
            "N" => Ok(Move(Cardinal::North, Self::parse_distance(arg)?)),
            "S" => Ok(Move(Cardinal::South, Self::parse_distance(arg)?)),
            "E" => Ok(Move(Cardinal::East, Self::parse_distance(arg)?)),
            "W" => Ok(Move(Cardinal::West, Self::parse_distance(arg)?)),
            "L" => Ok(Turn(Direction::Left, Self::parse_angle(arg)?)),
            "R" => Ok(Turn(Direction::Right, Self::parse_angle(arg)?)),
            "F" => Ok(Forward(Self::parse_distance(arg)?)),
            _ => Err("Invalid instruction"),
        }
    }

    fn parse_distance(arg: &str) -> Result<Distance, Error> {
        arg.parse().map_err(|_| "Could not parse distance")
    }

    fn parse_angle(arg: &str) -> Result<Angle, Error> {
        match arg {
            "90" => Ok(Angle::Ninety),
            "180" => Ok(Angle::OneEighty),
            "270" => Ok(Angle::TwoSeventy),
            _ => Err("Could not parse angle"),
        }
    }
}
