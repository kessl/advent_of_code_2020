mod ticket_validator;
use ticket_validator::TicketValidator;

fn main() {
    let validator = TicketValidator::from_notes("./notes.txt").unwrap();
    let error_rate = validator.find_invalid_values().iter().sum::<u32>();
    println!("Error rate: {}", error_rate);
}

#[test]
fn test() {
    let validator = TicketValidator::from_notes("./test_notes.txt").unwrap();
    let invalid = validator.find_invalid_values();
    let error_rate = invalid.iter().sum::<u32>();
    println!("invalid {:?} = {}", invalid, error_rate);
    assert_eq!(error_rate, 71);
}
