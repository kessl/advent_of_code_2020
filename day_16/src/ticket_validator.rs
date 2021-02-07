use std::collections::HashMap;
use std::ops::RangeInclusive;

type Error = &'static str;
type Range = RangeInclusive<u32>;
type Rules = HashMap<String, (Range, Range)>;
type Ticket = Vec<u32>;

#[derive(Debug)]
pub struct TicketValidator {
    rules: Rules,
    nearby_tickets: Vec<Ticket>,
}

impl TicketValidator {
    pub fn from_notes(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to open notes")?;
        let mut sections = file.split("\n\n");

        let rules = Self::parse_rules(sections.next().ok_or("Missing rules section")?)?;
        let _ticket = sections.next(); // TODO
        let nearby_tickets =
            Self::parse_tickets(sections.next().ok_or("Missing nearby tickets section")?)?;

        Ok(Self {
            rules,
            nearby_tickets,
        })
    }

    // find tickets with values that don't match any rules
    pub fn find_invalid_values(&self) -> Vec<u32> {
        let mut invalid = vec![];

        for ticket in &self.nearby_tickets {
            for field in ticket {
                let mut valid = false;
                for (range1, range2) in self.rules.values() {
                    if range1.contains(field) || range2.contains(field) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    invalid.push(*field);
                }
            }
        }

        invalid
    }

    fn parse_rules(rules: &str) -> Result<Rules, Error> {
        let mut map = HashMap::new();

        for line in rules.lines() {
            let mut tokens = line.split(":");
            let name = tokens.next().ok_or("Missing rule name")?;

            let mut ranges = tokens.next().ok_or("Missing ranges")?.split_whitespace();
            let range1 = Self::parse_range(ranges.next().ok_or("Missing range 1")?)?;
            let range2 = Self::parse_range(ranges.skip(1).next().ok_or("Missing range 2")?)?;

            map.insert(name.to_string(), (range1, range2));
        }

        Ok(map)
    }

    fn parse_tickets(tickets: &str) -> Result<Vec<Ticket>, Error> {
        tickets
            .lines()
            .skip(1)
            .map(|line| {
                line.split(",")
                    .map(|token| token.parse().map_err(|_| "Could not parse ticket value"))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn parse_range(range: &str) -> Result<Range, Error> {
        let mut split = range.split("-");
        let lower = split
            .next()
            .ok_or("Missing lower bound")?
            .parse::<u32>()
            .map_err(|_| "Could not parse lower bound")?;

        let upper = split
            .next()
            .ok_or("Missing upper bound")?
            .parse::<u32>()
            .map_err(|_| "Could not parse upper bound")?;

        Ok(lower..=upper)
    }
}
