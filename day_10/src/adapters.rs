use std::collections::HashMap;

type Error = &'static str;

pub struct Adapters {
    bag: Vec<u8>,
}

impl Adapters {
    pub fn from_file(filename: &str) -> Result<Adapters, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load adapters")?;
        let mut bag = file
            .lines()
            .map(Self::parse_adapter)
            .collect::<Result<Vec<_>, _>>()?;
        bag.sort();

        Ok(Self { bag })
    }

    pub fn chain(&self) -> (usize, usize, usize) {
        let mut differences: HashMap<u8, usize> = HashMap::new();
        let mut last_input_joltage = 0;

        for adapter in &self.bag {
            let diff = adapter - last_input_joltage;
            let count = differences.entry(diff).or_insert_with(|| 0);
            *count += 1;

            last_input_joltage = *adapter;
        }

        (
            *differences.entry(1).or_insert_with(|| 0),
            *differences.entry(2).or_insert_with(|| 0),
            *differences.entry(3).or_insert_with(|| 0) + 1, // built-in adapter is always 3 jolts higher rated
        )
    }

    pub fn combine(&self) -> usize {
        let res = Self::find_compatible(vec![*self.bag.iter().min().unwrap()], &self.bag);

        0
    }

    fn find_compatible(adapters: Vec<u8>, bag: &Vec<u8>) -> Vec<u8> {
        let mut compatible = vec![];

        for adapter in adapters {
            let index = bag.iter().position(|&thing| thing == adapter).unwrap();

            for i in (index + 1).. {
                if let Some(&thing) = bag.get(i) {
                    if thing - adapter > 3 {
                        break;
                    }
                    compatible.push(thing);
                } else {
                    break;
                }
            }
        }

        vec![]
    }

    fn parse_adapter(line: &str) -> Result<u8, Error> {
        line.parse().map_err(|_| "Could not parse adapter")
    }
}
