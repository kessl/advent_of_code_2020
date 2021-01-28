mod program;
use program::Program;

fn main() {
    let mut program = Program::from_source("./bootloader.txt").unwrap();
    program.run();
    program.fix();
}
