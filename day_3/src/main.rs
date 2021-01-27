#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn load(filename: &str) -> Result<Self, &'static str> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to open map file")?;
        let mut map: Vec<Vec<_>> = Vec::new();

        for line in file.lines() {
            map.push(line.chars().collect())
        }

        let width = map[0].len();
        let height = map.len();

        Ok(Self { map, width, height })
    }

    fn at(self: &Self, x: usize, y: usize) -> Result<char, &'static str> {
        if y >= self.height {
            return Err("Reached map bottom");
        }

        Ok(self.map[y][x % self.width])
    }
}

fn count_trees(map: &Map, slope: (i32, i32)) -> i32 {
    let mut trees = 0;
    let (x, y) = slope;

    let mut step = 0;
    loop {
        match map.at(step * x as usize, step * y as usize) {
            Ok('#') => trees += 1,
            Ok(_) => (),
            Err(_) => break,
        }
        step += 1;
    }

    trees
}

fn main() {
    let map = Map::load("map.txt").unwrap();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = slopes
        .iter()
        .map(|&slope| {
            let trees = count_trees(&map, slope);
            println!(
                "Slope ({}, {}): {} trees on the way",
                slope.0, slope.1, trees
            );
            trees as i64
        })
        .fold(1i64, |acc, trees| acc * trees);

    println!("Multiplied: {}", result);
}
