use regex::Regex;

fn main() {
    let valid = check_database_sled().unwrap();
    println!("Part 1: Valid passwords: {}", valid);

    let valid = check_database_toboggan().unwrap();
    println!("Part 2: Valid passwords: {}", valid);
}

fn check_database_toboggan() -> Result<i32, &'static str> {
    let file = std::fs::read_to_string("./passwords.txt")
        .map_err(|_| "Error opening password database")?;

    let re = Regex::new(r"^(.*)-(.*) (.): (.*)$").unwrap();
    let mut valid = 0;

    for line in file.lines() {
        for cap in re.captures_iter(line) {
            let position1 = cap[1]
                .parse::<usize>()
                .map_err(|_| "Could not parse position 1")?;
            let position2 = cap[2]
                .parse::<usize>()
                .map_err(|_| "Could not parse position 2")?;

            let letter = cap[3].chars().nth(0).unwrap();
            let password = &cap[4];

            let letter1 = password.chars().nth(position1 - 1).unwrap();
            let letter2 = password.chars().nth(position2 - 1).unwrap();

            if letter == letter1 && letter != letter2 {
                valid += 1;
            }

            if letter != letter1 && letter == letter2 {
                valid += 1;
            }
        }
    }

    Ok(valid)
}

fn check_database_sled() -> Result<i32, &'static str> {
    let file = std::fs::read_to_string("./passwords.txt")
        .map_err(|_| "Error opening password database")?;

    let re = Regex::new(r"^(.*)-(.*) (.): (.*)$").unwrap();
    let mut valid = 0;

    for line in file.lines() {
        for cap in re.captures_iter(line) {
            let min = cap[1]
                .parse::<usize>()
                .map_err(|_| "Could not parse minimum")?;
            let max = cap[2]
                .parse::<usize>()
                .map_err(|_| "Could not parse maximum")?;

            let letter = &cap[3];
            let password = &cap[4];

            let occurences = password.matches(letter).count();

            if occurences >= min && occurences <= max {
                valid += 1;
            }
        }
    }

    Ok(valid)
}
