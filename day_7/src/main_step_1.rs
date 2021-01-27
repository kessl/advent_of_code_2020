use regex::Regex;
use std::collections::HashMap;

// bag color & what color can hold it
type Rules = HashMap<String, Vec<String>>;

fn load_rules(filename: &str) -> Result<Rules, &'static str> {
    let file = std::fs::read_to_string(filename).map_err(|_| "Failed to load rules")?;

    let re_rule = Regex::new(r"^(.*) bags? contain (\d+.*|no.*).$").unwrap();
    let re_bags = Regex::new(r"\d+ (.*?) bags?").unwrap();

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
            .map(|c| c[1].to_string());

        for bag in bags {
            if let Some(v) = map.get_mut(&bag) {
                v.push(color.to_string());
            } else {
                map.insert(bag, vec![color.to_string()]);
            }
        }
    }

    Ok(map)
}

fn nest_bags(color: &str, rules: &mut Rules) -> Vec<String> {
    let mut colors = vec![vec![color.to_string()]];

    for step in 0usize.. {
        let current_colors = colors.get(step).unwrap();
        let mut next_colors = vec![];

        // for each color that holds the original color, find what can hold it
        for color in current_colors {
            if let Some(holders) = rules.get_mut(&color.to_string()) {
                next_colors.append(holders);
            }
        }

        if next_colors.len() == 0 {
            break;
        }

        colors.push(next_colors);
    }

    let mut result = colors.into_iter().skip(1).flatten().collect::<Vec<_>>();
    result.sort();
    result.dedup();
    result
}

fn main() {
    let mut rules = load_rules("./rules.txt").unwrap();
    let color = "shiny gold";

    // println!("What can hold what: {:#?}", rules);
    let nested = nest_bags(color, &mut rules);
    // println!("These colors can ultimately hold {}: {:#?}", color, nested);
    println!(
        "Total {} bag colors can ultimately hold {}.",
        nested.len(),
        color
    );
}
