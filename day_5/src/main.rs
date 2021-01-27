fn load_passes(filename: &str) -> Result<Vec<String>, &'static str> {
    let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load boarding passes")?;
    Ok(file
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>())
}

const ROW_LENGTH: usize = 7;

fn read_pass(pass: &str) -> (i32, i32) {
    let (row, col) = pass.split_at(ROW_LENGTH);
    (decode_position(row), decode_position(col))
}

fn decode_position(position: &str) -> i32 {
    let mut min = 0;
    let mut max = 2i32.pow(position.len() as u32);

    for letter in position.chars() {
        let half = (max - min + 1) / 2;
        match letter {
            'F' | 'L' => max -= half,
            'B' | 'R' => min += half,
            _ => panic!("Invalid position character: {}", letter),
        }
    }

    min
}

fn seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

fn main() {
    let passes = load_passes("./boarding_passes.txt").unwrap();
    let ids = passes.iter().map(|pass| {
        let (row, col) = read_pass(&pass);
        seat_id(row, col)
    }).collect::<Vec<_>>();

    let lowest = *ids.iter().min_by(|a, b| a.cmp(&b)).unwrap();
    let highest = *ids.iter().max_by(|a, b| a.cmp(&b)).unwrap();

    println!("Lowest ID {}", lowest);
    println!("Highest ID {}", highest);

    for i in lowest..highest {
        if !ids.contains(&i) {
            println!("Missing ID: {}", i);
        }
    }
}
