mod decoder;
use decoder::Decoder;

fn main() {
    let mut decoder = Decoder::from_file("./program.txt").unwrap();
    decoder.run();
    println!("Sum in memory: {}", decoder.sum());
}

#[test]
fn test() {
    let mut decoder = Decoder::from_file("./test_program.txt").unwrap();
    decoder.run();
    assert_eq!(decoder.sum(), 165);
}
