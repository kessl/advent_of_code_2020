use regex::Regex;
use std::collections::HashMap;

fn read_passports(filename: &str) -> Vec<String> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.split("\n\n").map(|s| s.to_string()).collect()
}

type Fields<'k, 'v> = HashMap<&'k str, &'v str>;

fn parse_fields(passport: &str) -> Fields {
    passport
        .split_whitespace()
        .map(|field| {
            let mut split = field.split(":");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Fields>()
}

fn check_fields(fields: &Fields) -> bool {
    let mandatory_fields = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*, "cid" */
    ];
    mandatory_fields
        .iter()
        .all(|field_name| fields.contains_key(field_name))
}

fn validate_fields(fields: &Fields) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = fields
        .get("byr")
        .unwrap()
        .parse::<i32>()
        .map(|val| val >= 1920 && val <= 2002)
        .unwrap_or(false);
    if !byr {
        return false;
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = fields
        .get("iyr")
        .unwrap()
        .parse::<i32>()
        .map(|val| val >= 2010 && val <= 2020)
        .unwrap_or(false);
    if !iyr {
        return false;
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = fields
        .get("eyr")
        .unwrap()
        .parse::<i32>()
        .map(|val| val >= 2020 && val <= 2030)
        .unwrap_or(false);
    if !eyr {
        return false;
    }

    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    if let Some(cap) = hgt_re.captures(fields.get("hgt").unwrap()) {
        let hgt = cap[1]
            .parse::<i32>()
            .map(|val| match &cap[2] {
                "cm" => val >= 150 && val <= 193,
                "in" => val >= 59 && val <= 76,
                _ => false,
            })
            .unwrap_or(false);
        if !hgt {
            return false;
        }
    } else {
        return false
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let hcl = hcl_re.is_match(fields.get("hcl").unwrap());
    if !hcl {
        return false;
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let ecl_values = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let ecl = ecl_values.contains(fields.get("ecl").unwrap());
    if !ecl {
        return false;
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let pid = pid_re.is_match(fields.get("pid").unwrap());
    if !pid {
        return false;
    }

    // cid (Country ID) - ignored, missing or not.

    true
}

fn main() {
    let passports = read_passports("./passports.txt");

    let valid = passports
        .iter()
        .map(|passport| {
            let fields = parse_fields(passport);
            check_fields(&fields) && validate_fields(&fields)
        })
        .filter(|valid| *valid)
        .count();

    println!(
        "{} valid passports out of {} total",
        valid,
        passports.iter().count()
    );
}
