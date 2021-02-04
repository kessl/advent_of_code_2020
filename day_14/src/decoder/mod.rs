use std::collections::HashMap;

mod mask;
use mask::Mask;

pub type Error = &'static str;

#[derive(Debug)]
pub enum Instruction {
    SetMask(Mask),
    Write(u64, u64),
}

#[derive(Debug)]
pub struct Decoder {
    instructions: Vec<Instruction>,
    memory: HashMap<u64, u64>,
    pc: usize,
    last_mask: Option<usize>
}

impl Decoder {
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load program")?;
        let instructions = file
            .lines()
            .map(Self::parse_instruction)
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self {
            instructions,
            memory: HashMap::new(),
            pc: 0,
            last_mask: None,
        })
    }

    pub fn run(&mut self) {
        while let Ok(_) = self.execute() {}
    }

    pub fn sum(&self) -> u64 {
        self.memory.values().sum()
    }

    fn parse_instruction(line: &str) -> Result<Instruction, Error> {
        let mut split = line.split_whitespace();
        let instruction = split.next().ok_or("Missing instruction")?;
        let value = split.skip(1).next().ok_or("Missing value")?;

        match instruction {
            "mask" => Ok(Instruction::SetMask(Mask::from_string(value)?)),
            _ => {
                if &instruction[0..3] == "mem" {
                    let addr = &instruction[4..instruction.len() - 1];
                    return Ok(Instruction::Write(
                        addr.parse::<u64>().map_err(|_| "Invalid address")?,
                        value.parse::<u64>().map_err(|_| "Invalid value")?,
                    ));
                }

                Err("Unknown instruction")
            }
        }
    }

    fn execute(&mut self) -> Result<(), Error> {
        if let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                    Instruction::SetMask(_) => {
                        self.last_mask = Some(self.pc)
                    },
                    Instruction::Write(addr, value) => {
                        if let Some(last_mask) = self.last_mask {
                            let inst = self.instructions.get(last_mask).ok_or("Invalid mask position")?;
                            if let Instruction::SetMask(mask) = inst {
                                let result = mask.apply(*value);
                                self.memory.insert(*addr, result);
                            }
                        }
                    }
            }
        } else {
            return Err("Reached end of program")
        }

        self.pc += 1;

        Ok(())
    }
}
