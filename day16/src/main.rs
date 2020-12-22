use std::collections::HashMap;
static FILENAME: &str = "input/data";

type Ticket = Vec<usize>;

struct Rule {
    name: String,
    nums: Vec<(usize, usize)>,
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let rules = parse_rules(&data);
    let nearby_tickets = parse_nearby(&data);
    let ticket = parse_ticket(&data);
    println!("{}", part_one(&nearby_tickets, &rules));
    println!("{}", part_two(&ticket, &nearby_tickets, &rules));
}

fn part_one(nearby: &Vec<Ticket>, rules: &Vec<Rule>) -> usize {
    let mut invalids: Vec<usize> = Vec::new();

    for ticket in nearby {
        for field in ticket {
            if !is_field_valid_for_rules(*field, &rules) {
                invalids.push(*field);
            }
        }
    }

    invalids.iter().sum()
}

fn part_two(my_ticket: &Vec<usize>, nearby_tickets: &Vec<Ticket>, rules: &Vec<Rule>) -> usize {
    let mut nearby = nearby_tickets.clone();
    // Wash away invalid tickets
    nearby.retain(|ticket| is_ticket_valid(ticket.to_vec(), &rules));
    let no_of_fields = nearby[0].len();
    let mut mapper: HashMap<usize, &Rule> = HashMap::new();
    while mapper.len() < no_of_fields {
        for rule in rules {
            let mut count = 0;
            let mut last_valid_ix = 0;
            for x in 0..no_of_fields {
                if !mapper.contains_key(&x) && is_ix_valid(x, &nearby, &rule) {
                    count += 1;
                    last_valid_ix = x;
                }
            }
            if count == 1 {
                mapper.insert(last_valid_ix, rule);
            }
        }
    }

    mapper
        .iter()
        .filter(|(_, v)| v.name.starts_with("departure"))
        .map(|(k, _i)| my_ticket[*k])
        .product::<usize>()
}

fn is_ix_valid(ix: usize, tickets: &Vec<Vec<usize>>, rule: &Rule) -> bool {
    for ticket in tickets {
        if !is_field_valid(ticket[ix], rule) {
            return false;
        }
    }
    true
}

fn is_ticket_valid(ticket: Ticket, sections: &Vec<Rule>) -> bool {
    for field in ticket {
        if !is_field_valid_for_rules(field, sections) {
            return false;
        }
    }
    true
}

fn is_field_valid_for_rules(field: usize, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if is_field_valid(field, rule) {
            return true;
        }
    }
    false
}

fn is_field_valid(field: usize, rule: &Rule) -> bool {
    return (field >= rule.nums[0].0 && field <= rule.nums[0].1)
        || (field >= rule.nums[1].0 && field <= rule.nums[1].1);
}

fn parse_nearby(data: &str) -> Vec<Ticket> {
    let mut ticket_section = 0;
    let mut nearby_tickets: Vec<Ticket> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            ticket_section += 1;
            continue;
        }
        match ticket_section {
            2 => {
                if line == "nearby tickets:" {
                    continue;
                }
                let num_split: Vec<&str> = line.split(",").collect();
                let mut nearby_ticket: Vec<usize> = Vec::new();
                for num in num_split {
                    nearby_ticket.push(num.parse::<usize>().unwrap());
                }
                nearby_tickets.push(nearby_ticket);
            }
            _ => {}
        }
    }
    nearby_tickets
}

fn parse_ticket(data: &str) -> Ticket {
    let mut ticket_section = 0;
    let mut ticket: Vec<usize> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            ticket_section += 1;
            continue;
        }
        match ticket_section {
            1 => {
                if line == "your ticket:" {
                    continue;
                }
                let num_split: Vec<&str> = line.split(",").collect();
                for num in num_split {
                    ticket.push(num.parse::<usize>().unwrap());
                }
            }
            _ => {}
        }
    }
    ticket
}

fn parse_rules(data: &str) -> Vec<Rule> {
    let mut ticket_section = 0;
    let mut rules: Vec<Rule> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            ticket_section += 1;
            continue;
        }
        match ticket_section {
            0 => {
                let name_split: Vec<&str> = line.split(": ").collect();
                let name = name_split[0];
                let range_split: Vec<&str> = name_split[1].split(" or ").collect();
                let num_split_1: Vec<&str> = range_split[0].split("-").collect();
                let num_split_2: Vec<&str> = range_split[1].split("-").collect();
                let rule = Rule {
                    name: name.to_string(),
                    nums: vec![
                        (
                            num_split_1[0].parse::<usize>().unwrap(),
                            num_split_1[1].parse::<usize>().unwrap(),
                        ),
                        (
                            num_split_2[0].parse::<usize>().unwrap(),
                            num_split_2[1].parse::<usize>().unwrap(),
                        ),
                    ],
                };
                rules.push(rule);
            }
            _ => {}
        }
    }
    rules
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let rules = super::parse_rules(&data);
        let nearby = super::parse_nearby(&data);
        assert_eq!(32835, super::part_one(&nearby, &rules));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let rules = super::parse_rules(&data);
        let nearby = super::parse_nearby(&data);
        let ticket = super::parse_ticket(&data);
        assert_eq!(514662805187, super::part_two(&ticket, &nearby, &rules));
    }
}
