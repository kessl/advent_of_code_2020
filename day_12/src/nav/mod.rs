mod types;
use types::{Instruction::*, *};

pub use types::Position;

pub struct Nav {
    position: Position,
    waypoint: Position,

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
            position: Position(0, 0),
            waypoint: Position(10, 1),
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
        println!("Pos: {:?}, waypoint {:?}", self.position, self.waypoint);

        let instruction = self
            .instructions
            .get(self.step)
            .ok_or("Out of instructions")?;
        
        print!("{:?}\t", instruction);

        match instruction {
            Move(cardinal, distance) => {
                self.waypoint += Self::cardinal_offset(&cardinal, *distance)
            }
            Turn(direction, angle) => {
                self.waypoint = Self::rotate(&self.waypoint, &angle, &direction);
            }
            Forward(distance) => self.position += self.waypoint.clone() * *distance,
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

    fn rotate(point: &Position, angle: &Angle, direction: &Direction) -> Position {
        let rotated = match (angle, direction) {
            (Angle::Ninety, Direction::Left) => (point.1, -point.0),
            (Angle::OneEighty, _) => (-point.0, -point.1),
            (Angle::TwoSeventy, Direction::Left) => (-point.1, point.0),
            (Angle::Ninety, Direction::Right) => (-point.1, point.0),
            (Angle::TwoSeventy, Direction::Right) => (point.1, -point.0),
        };

        Position(rotated.0, rotated.1)
    }

    fn parse_instruction(line: &str) -> Result<Instruction, Error> {
        let (action, arg) = line.split_at(1);
        match action {
            "N" => Ok(Move(Cardinal::North, Self::parse_distance(arg)?)),
            "S" => Ok(Move(Cardinal::South, Self::parse_distance(arg)?)),
            "E" => Ok(Move(Cardinal::East, Self::parse_distance(arg)?)),
            "W" => Ok(Move(Cardinal::West, Self::parse_distance(arg)?)),
            "L" => Ok(Turn(Direction::Right, Self::parse_angle(arg)?)), // WHAT!!
            "R" => Ok(Turn(Direction::Left, Self::parse_angle(arg)?)), // WHY IS THIS THE OTHER WAY AROUND
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
