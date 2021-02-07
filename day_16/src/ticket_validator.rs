use std::collections::HashMap;
use std::ops::RangeInclusive;

type Error = &'static str;
type Range = RangeInclusive<u32>;
type Rules = HashMap<String, (Range, Range)>;
type Ticket = Vec<u32>;

#[derive(Debug)]
pub struct TicketValidator {
    rules: Rules,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
    fields: Vec<Vec<String>>,
}

impl TicketValidator {
    pub fn from_notes(filename: &str) -> Result<Self, Error> {
        let file = std::fs::read_to_string(filename).map_err(|_| "Failed to open notes")?;
        let mut sections = file.split("\n\n");

        let rules = Self::parse_rules(sections.next().ok_or("Missing rules section")?)?;
        let ticket = Self::parse_tickets(sections.next().ok_or("Missing your ticket section")?)?
            .into_iter()
            .nth(0)
            .ok_or("Missing ticket")?;
        let nearby_tickets =
            Self::parse_tickets(sections.next().ok_or("Missing nearby tickets section")?)?;

        Ok(Self {
            fields: std::iter::repeat_with(|| vec![])
                .take(rules.len())
                .collect(),
            rules,
            ticket,
            nearby_tickets,
        })
    }

    pub fn name_fields(&mut self, tickets: &Vec<Ticket>) {
        // find fields that fit rules for each slot
        for (name, (range1, range2)) in self.rules.iter() {
            for i in 0..self.rules.len() {
                let mut fits = true;

                for ticket in tickets {
                    if !range1.contains(&ticket[i]) && !range2.contains(&ticket[i]) {
                        fits = false;
                        break;
                    }
                }

                if fits {
                    self.fields.get_mut(i).unwrap().push(name.to_string());
                }
            }
        }

        // eliminiate field names used elsewhere
        for _ in 0..self.rules.len() {
            for i in 0..self.rules.len() {
                let fields = self.fields.get(i).unwrap();
                let len = fields.len();
                let value = fields[0].to_string();
    
                if len == 1 {
                    for j in 0..self.fields.len() {
                        if i == j {
                            continue;
                        };
                        let options = self.fields.get_mut(j).unwrap();
                        options.retain(|option| option != &value);
                    }
                }
            }
        }

        for (index, field) in self.fields.iter().enumerate() {
            println!("{}: {:?}: {}", index, field[0], self.ticket.get(index).unwrap());
        }
    }

    pub fn find_valid(&self) -> Vec<Ticket> {
        // filter tickets, whose fields all match any rule
        self.nearby_tickets
            .iter()
            .filter(|ticket| {
                ticket.iter().all(|field| {
                    self.rules
                        .values()
                        .any(|(range1, range2)| range1.contains(field) || range2.contains(field))
                })
            })
            .map(|ticket| ticket.clone())
            .collect()
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
