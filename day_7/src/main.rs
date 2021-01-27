use regex::Regex;
use std::collections::HashMap;

// bag color & how much of what colors it can hold
type Rules = HashMap<String, Vec<(i32, String)>>;

fn load_rules(filename: &str) -> Result<Rules, &'static str> {
    let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load rules")?;

    let re_rule = Regex::new(r"^(.*) bags? contain (\d+.*|no.*).$").unwrap();
    let re_bags = Regex::new(r"(\d+) (.*?) bags?").unwrap();

    let mut map: Rules = HashMap::new();

    for line in file.lines() {
        let matches = re_rule.captures(line).unwrap();
        let color = &matches[1];
        let bags_string = &matches[2];

        if bags_string == "no other bags" {
            continue; // YOLO
        }

        let bags = bags_string
            .split(",")
            .map(|s| re_bags.captures(s).unwrap())
            .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
            .collect::<Vec<_>>();

        map.insert(color.to_string(), bags);
    }

    Ok(map)
}

fn fill_bag(color: &str, rules: &mut Rules) -> Vec<(i32, String)> {
    let mut bags = vec![vec![(1, color.to_string())]];
    for step in 0usize.. {
        let current_bags = bags.get(step).unwrap();
        let mut new_bags: Vec<(i32, String)> = vec![];

        // see what bags need to go in
        for (amount, bag) in current_bags {
            if let Some(filler) = rules.get_mut(&bag.to_string()) {
                for (filler_amount, color) in filler {
                    new_bags.push((amount * *filler_amount, color.to_string()))
                }
            }
        }

        if new_bags.len() == 0 {
            break;
        }

        bags.push(new_bags);
    }

    bags.into_iter().flatten().skip(1).collect()
}

fn main() {
    let mut rules = load_rules("./rules.txt").unwrap();
    let color = "shiny gold";

    let filler = fill_bag(color, &mut rules);
    println!(
        "Total {} bags need to be stuffed in {}",
        filler.iter().fold(0, |acc, (amount, _)| acc + amount),
        color
    );
}
