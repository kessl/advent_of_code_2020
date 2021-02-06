pub struct Recitation {
    spoken: Vec<u32>,
}

impl Recitation {
    pub fn start(starting_numbers: Vec<u32>) -> Self {
        Self {
            spoken: starting_numbers,
        }
    }

    pub fn speak(&mut self) -> u32 {
        let last = self.spoken.get(self.spoken.len() - 1).unwrap();
        let distance = self.distance(&last).unwrap_or(0) as u32;
        self.spoken.push(distance);
        distance
    }

    fn distance(&self, number: &u32) -> Option<usize> {
        self.spoken
            .iter()
            .rev()
            .skip(1)
            .position(|item| item == number)
            .map(|res| res + 1)
    }
}
