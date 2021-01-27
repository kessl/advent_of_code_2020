fn main() {
    if let Ok((number1, number2, number3)) = find_sum() {
        println!("Found numbers: {}, {}, {}", number1, number2, number3);
        println!(
            "{} * {} * {} = {}",
            number1,
            number2,
            number3,
            number1 * number2 * number3
        );
    }
}

fn find_sum() -> Result<(i32, i32, i32), &'static str> {
    let file = std::fs::read_to_string("./expense_report.txt")
        .map_err(|_| "Error opening expense report")?;
    let lines: Vec<_> = file.lines().collect();

    for line in lines.iter() {
        let number1 = line.parse::<i32>().map_err(|_| "Error parsing number")?;

        for line in lines.iter() {
            let number2 = line.parse::<i32>().map_err(|_| "Error parsing number")?;

            for line in lines.iter() {
                let number3 = line.parse::<i32>().map_err(|_| "Error parsing number")?;
                if number1 + number2 + number3 == 2020 {
                    return Ok((number1, number2, number3));
                }
            }
        }
    }

    Err("Sum not found")
}
