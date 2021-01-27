use std::collections::HashSet;

fn load_answers(filename: &str) -> Result<Vec<String>, &'static str> {
    let file = std::fs::read_to_string(filename).map_err(|_| "Failed to open answers file")?;
    Ok(file
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>())
}

fn main_step_1() {
    let mut answers = load_answers("./answers.txt").unwrap();
    let groups = answers.iter_mut().map(|group| {
        group.retain(|c| !c.is_whitespace());
        let mut chars = group.chars().collect::<Vec<_>>();
        chars.sort_by(|a, b| a.cmp(b));
        chars.dedup();
        chars.len()
    });

    println!("Sum of questions answered yes per group: {}", groups.sum::<usize>());
}

fn main() {
    let answers = load_answers("./answers.txt").unwrap();
    let groups = answers.iter().map(|group| {
        let mut persons = group.split_whitespace();
        let mut letters = persons.next().unwrap().chars().collect::<HashSet<_>>();

        for person in persons {
            let chars = person.chars().collect::<Vec<_>>();
            letters.retain(|c| chars.contains(c));
        }

        letters.len()
    });

    println!("Sum of questions answered yes per group: {}", groups.sum::<usize>());
}
