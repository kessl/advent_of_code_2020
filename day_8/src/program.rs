use std::collections::HashSet;

type Error = &'static str;
type Arg = i32;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Acc(Arg),
    Jmp(Arg),
    Nop(Arg),
}

#[derive(Debug, PartialEq)]
pub enum ExitCode {
    InfiniteLoop,
    Halt,
}

#[derive(Debug)]
pub struct Program {
    code: Vec<Instruction>,
    pc: usize,
    acc: Arg,
}

impl Program {
    pub fn from_source(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load source code")?;
        let code = file
            .lines()
            .map(Self::parse_instruction)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            code,
            pc: 0,
            acc: 0,
        })
    }

    pub fn run(&mut self) -> ExitCode {
        let mut visited: HashSet<usize> = HashSet::new();

        loop {
            if visited.contains(&self.pc) {
                println!("Infinite loop detected. Acc = {}", self.acc);
                return ExitCode::InfiniteLoop;
            } else {
                visited.insert(self.pc);
            }

            if self.execute().is_err() {
                break;
            }
        }

        println!("Program halted. Acc = {}", self.acc);
        ExitCode::Halt
    }

    pub fn fix(&mut self) {
        // try changing jmp <> nop and see if program halts
        for i in 0..self.code.len() {
            // swap instruction, drop reference before running program
            {
                let instruction = self.code.get_mut(i).unwrap();
                if let Instruction::Acc(_) = instruction {
                    continue;
                }

                print!("Trying\t{}\t{:?} <> ", i, instruction);
                Self::swizzle_instruction(instruction);
                print!("{:?}\t", instruction);
            }

            if self.run() == ExitCode::Halt {
                break; // we're done
            }

            // change back, reset and try next instruction
            let instruction = self.code.get_mut(i).unwrap();
            Self::swizzle_instruction(instruction);
            self.acc = 0;
            self.pc = 0;
        }
    }

    fn swizzle_instruction(instruction: &mut Instruction) {
        *instruction = match *instruction {
            Instruction::Nop(ref mut arg) => Instruction::Jmp(std::mem::take(arg)),
            Instruction::Jmp(ref mut arg) => Instruction::Nop(std::mem::take(arg)),
            Instruction::Acc(arg) => Instruction::Acc(arg),
        }
    }

    fn parse_argument(arg_str: &str) -> Result<Arg, Error> {
        arg_str.parse().map_err(|_| "Failed to parse argument")
    }

    fn parse_instruction(line: &str) -> Result<Instruction, Error> {
        let mut split = line.split(" ");

        match (split.next(), split.next()) {
            (Some("acc"), Some(arg)) => Ok(Instruction::Acc(Self::parse_argument(arg)?)),
            (Some("jmp"), Some(arg)) => Ok(Instruction::Jmp(Self::parse_argument(arg)?)),
            (Some("nop"), Some(arg)) => Ok(Instruction::Nop(Self::parse_argument(arg)?)),
            _ => Err("Unknown instruction"),
        }
    }

    fn offset_pc(&self, offset: Arg) -> usize {
        if offset.is_negative() {
            self.pc
                .checked_sub(offset.wrapping_abs() as u32 as usize)
                .expect("Program counter underflow")
        } else {
            self.pc
                .checked_add(offset as usize)
                .expect("Program counter overflow")
        }
    }

    fn execute(&mut self) -> Result<(), Error> {
        if let Some(instruction) = self.code.get(self.pc) {
            match instruction {
                Instruction::Acc(arg) => {
                    self.acc += arg;
                    self.pc += 1;
                }
                Instruction::Jmp(arg) => {
                    self.pc = self.offset_pc(*arg);
                }
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
            }

            Ok(())
        } else {
            Err("Reached end of program")
        }
    }
}
