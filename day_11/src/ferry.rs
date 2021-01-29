type Error = &'static str;

#[derive(Debug, Clone, PartialEq)]
enum SeatLayout {
    Floor,
    Seat(bool), // false = empty, true = occupied
}

type Seats = Vec<Vec<SeatLayout>>;

#[derive(Debug)]
pub struct Ferry {
    seats: Seats,
}

static NEIGHBORS: [[i8; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

impl Ferry {
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load seat layout")?;
        let seats = file.lines().map(Self::parse_line).collect::<Vec<_>>();

        Ok(Self { seats })
    }

    pub fn stabilize(&mut self) -> usize {
        let mut generation = 0;
        loop {
            let occupied_seats = self
                .seats
                .iter()
                .flatten()
                .filter(|seat| match seat {
                    SeatLayout::Seat(true) => true,
                    _ => false,
                })
                .count();

            println!(
                "Generation {}: {} occupied seats",
                generation, occupied_seats
            );
            let new_seats = self.reshuffle();

            if new_seats.iter().flatten().eq(self.seats.iter().flatten()) {
                break occupied_seats; // no change from last reshuffle
            }

            self.seats = new_seats;
            generation += 1;
        }
    }

    fn reshuffle(&self) -> Seats {
        let mut new_seats = self.seats.clone();

        for y in 0..self.seats.len() {
            for x in 0..self.seats.get(y).unwrap().len() {
                let seat = new_seats
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .expect("Seat out of bounds");

                if let SeatLayout::Floor = seat {
                    continue;
                }

                match self.count_occupied_neighbors(x, y) {
                    0 => {
                        *seat = SeatLayout::Seat(true);
                    }
                    5..=8 => {
                        *seat = SeatLayout::Seat(false);
                    }
                    _ => (),
                }
            }
        }

        new_seats
    }

    fn add(u: usize, i: i8) -> Option<usize> {
        if i.is_negative() {
            u.checked_sub(i.wrapping_abs() as u8 as usize)
        } else {
            u.checked_add(i as usize)
        }
    }

    fn count_occupied_neighbors(&self, cell_x: usize, cell_y: usize) -> u8 {
        let mut occupied_neighbors = 0;

        for [dx, dy] in &NEIGHBORS {
            // go in this direction until we find a seat
            for step in 1.. {
                if let (Some(x), Some(y)) =
                    (Self::add(cell_x, *dx * step), Self::add(cell_y, *dy * step))
                {
                    let seat = self.seats.get(y).and_then(|row| row.get(x));
                    match seat {
                        Some(SeatLayout::Seat(false)) => break,
                        Some(SeatLayout::Seat(true)) => {
                            occupied_neighbors += 1;
                            break;
                        }
                        Some(SeatLayout::Floor) => (), // go further
                        None => break, // out of layout bounds
                    }
                } else {
                    break; // out of layout bounds
                }
            }
        }

        occupied_neighbors
    }

    fn parse_line(line: &str) -> Vec<SeatLayout> {
        line.chars().map(Self::parse_symbol).collect()
    }

    fn parse_symbol(symbol: char) -> SeatLayout {
        match symbol {
            '.' => SeatLayout::Floor,
            'L' => SeatLayout::Seat(false),
            '#' => SeatLayout::Seat(true),
            _ => panic!("Invalid character in seat layout"),
        }
    }
}

impl std::fmt::Display for Ferry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.seats {
            for seat in row {
                write!(f, "{}", seat).unwrap();
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl std::fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ascii = match self {
            Self::Floor => ".",
            Self::Seat(false) => "L",
            Self::Seat(true) => "#",
        };

        write!(f, "{}", ascii)
    }
}
