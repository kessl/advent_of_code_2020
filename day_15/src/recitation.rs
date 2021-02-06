use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Recitation {
    spoken: HashMap<u32, u32>,
    last: u32,
    position: u32,
}

impl Recitation {
    pub fn start(starting_numbers: Vec<u32>) -> Self {
        assert!(starting_numbers.len() > 0);

        Self {
            spoken: HashMap::from_iter(
                starting_numbers
                    .iter()
                    .take(starting_numbers.len() - 1)
                    .enumerate()
                    .map(|(index, &value)| (value, index as u32)),
            ),
            last: *starting_numbers.last().unwrap(),
            position: starting_numbers.len() as u32 - 1,
        }
    }

    pub fn speak(&mut self) -> u32 {
        let distance = self.distance(&self.last).unwrap_or(0);

        self.spoken.insert(self.last, self.position);
        self.last = distance;
        self.position += 1;

        distance
    }

    fn distance(&self, number: &u32) -> Option<u32> {
        self.spoken
            .get(number)
            .map(|position| self.position - position)
    }
}
