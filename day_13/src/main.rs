type Error = &'static str;

struct Timetable {
    lines: Vec<Option<u32>>,
    earliest_departure: u32,
}

impl Timetable {
    fn from_notes(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to open notes")?;
        let mut iter = file.lines();
        let earliest_departure = iter
            .next()
            .ok_or("Missing timestamp in notes")?
            .parse::<u32>()
            .map_err(|_| "Could not parse timestamp")?;

        let lines = iter
            .next()
            .ok_or("Missing lines in service in notes")?
            .split(",")
            .map(|token| token.parse::<u32>().ok())
            .collect::<Vec<_>>();

        Ok(Self {
            lines,
            earliest_departure,
        })
    }

    fn find_earliest(&self) -> (u32, u32) {
        self.lines
            .iter()
            .map(|&num| (num, num - self.earliest_departure % num))
            .min_by_key(|(_, wait_time)| *wait_time)
            .expect("Could not find earliest bus departure")
    }

    fn find_subsequent(&self) -> u32 {

    }
}

fn main() {
    let timetable = Timetable::from_notes("./notes.txt").unwrap();
    let (line, wait_time) = timetable.find_earliest();
    println!(
        "Earliest is bus ID {} in {} minutes. Multiplied: {}",
        line,
        wait_time,
        line * wait_time
    );
}

#[test]
fn test() {
    let timetable = Timetable::from_notes("./test_notes.txt").unwrap();
    let (line, wait_time) = timetable.find_earliest();
    assert_eq!(line, 59);
    assert_eq!(wait_time, 5);
}
