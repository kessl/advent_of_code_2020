mod nav;
use nav::{Nav, Position};

fn main() {
    let mut nav = Nav::from_file("./instructions.txt").unwrap();
    let destination = nav.navigate();
    println!(
        "Destination: {:?}, distance from origin: {}",
        destination,
        Nav::manhattan_distance(&Position(0, 0), destination)
    );
}

#[test]
fn test_step2() {
    let mut nav = Nav::from_file("./test_instructions.txt").unwrap();
    let destination = nav.navigate();
    assert_eq!(destination, &Position(214, -72));
    assert_eq!(Nav::manhattan_distance(&Position(0, 0), destination), 286);
}

// #[test]
// fn test_step1() {
//     let mut nav = Nav::from_file("./test_instructions.txt").unwrap();
//     let destination = nav.navigate();
//     assert_eq!(destination, &Position(17, -8));
//     assert_eq!(Nav::manhattan_distance(&Position(0, 0), destination), 25);
// }
