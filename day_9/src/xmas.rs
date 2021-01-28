use std::collections::VecDeque;
use std::iter::FromIterator;

type Error = &'static str;
type XmasResult<T> = Result<T, Error>;

pub struct Xmas {
    buffer: Vec<u64>,
    preamble: usize,
}

impl Xmas {
    pub fn from_file(filename: &str, preamble: usize) -> XmasResult<Self> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to read data")?;
        let buffer = file
            .lines()
            .map(Self::parse_line)
            .collect::<Result<Vec<_>, Error>>()?;
        if buffer.len() < preamble {
            return Err("Not enough data");
        }

        Ok(Self { buffer, preamble })
    }

    pub fn first_invalid(&self) -> Option<u64> {
        let mut queue = VecDeque::from_iter(self.buffer.iter().take(self.preamble));

        for number in self.buffer.iter().skip(self.preamble) {
            let mut sum_found = false;

            // sum all numbers in queue
            'outer: for i in 0..self.preamble {
                for j in 0..self.preamble {
                    if *number == *queue.get(i).unwrap() + *queue.get(j).unwrap() {
                        sum_found = true;
                        break 'outer;
                    }
                }
            }

            // return if not found
            if !sum_found {
                return Some(*number);
            }

            // if valid, add to queue
            queue.pop_front();
            queue.push_back(number);
        }

        None // all numbers are valid
    }

    pub fn contiguous_sum(&self, sum: u64) -> Option<Vec<u64>> {
        for i in 0..self.buffer.len() {
            let mut running_sum = 0;
            let mut addends = vec![];

            // compute a running sum until larger than the target number
            // and keep track of numbers added together
            for offset in 0..(self.buffer.len() - i) {
                if running_sum >= sum {
                    break;
                }
                let next = self.buffer.get(i + offset).unwrap();
                running_sum += next;
                addends.push(*next);
            }

            if running_sum == sum {
                return Some(addends);
            }
        }

        None
    }

    fn parse_line(line: &str) -> XmasResult<u64> {
        line.parse::<u64>().map_err(|_| "Error parsing data")
    }
}
