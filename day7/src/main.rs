use std::collections::HashMap;

static FILENAME: &str = "input/data";

type Rules = HashMap<String, Vec<Bag>>;

struct Bag {
    name: String,
    amount: usize,
}

fn main() {
    let mut data = std::fs::read_to_string(FILENAME).expect("could not read file");
    data.push('\n');
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let rules = parse_rules(data);
    count_bags("shiny gold", &rules) - 1
}

fn part_two(data: &str) -> usize {
    let rules = parse_rules(data);
    sum_bags("shiny gold", &rules) - 1
}

fn count_bags(name: &str, bags: &Rules) -> usize {
    bags.keys().filter(|k| contains_bag(name, bags, k)).count()
}

fn contains_bag(name: &str, map: &Rules, bag: &str) -> bool {
    bag == name || map[bag].iter().any(|b| contains_bag(name, map, &b.name))
}

fn sum_bags(bag: &str, bags: &Rules) -> usize {
    1 + bags[bag]
        .iter()
        .map(|b| b.amount * sum_bags(&b.name, bags))
        .sum::<usize>()
}

fn parse_rules(data: &str) -> Rules {
    let mut rules = HashMap::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let split = line.split(" ").collect::<Vec<&str>>();
        let contain_split = line.split(" contain ").collect::<Vec<&str>>();
        let contains = contain_split[1];
        let bags_split = contains.split(", ").collect::<Vec<&str>>();

        let mut bags = Vec::new();
        for bag_s in bags_split {
            if bag_s == "no other bags." {
                continue;
            }
            let properties = bag_s.split(" ").collect::<Vec<&str>>();
            let name = format!("{} {}", properties[1], properties[2]);
            let bag = Bag {
                amount: properties[0].parse::<usize>().unwrap(),
                name: name,
            };
            bags.push(bag);
        }

        let name = format!("{} {}", split[0], split[1]);
        rules.insert(name, bags);
    }
    rules
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(254, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(6006, super::part_two(&data));
    }
}
