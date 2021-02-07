mod ticket_validator;
use ticket_validator::TicketValidator;

fn main() {
    let mut validator = TicketValidator::from_notes("./notes.txt").unwrap();
    let valid = validator.find_valid();
    validator.name_fields(&valid);
}

#[test]
fn test() {
    let mut validator = TicketValidator::from_notes("./test_notes.txt").unwrap();
    let valid = validator.find_valid();
    validator.name_fields(&valid);
    assert_eq!(1, 71);
}
