static FILENAME: &str = "input/data";
use std::collections::HashSet;

fn main() {
    let mut data = std::fs::read_to_string(FILENAME).expect("could not read file");
    data.push('\n');
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let mut count = 0;
    let mut forms: Vec<char> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            forms.sort();
            forms.dedup();
            count += forms.len();
            forms.clear();
        }
        for c in line.chars() {
            forms.push(c);
        }
    }
    forms.sort();
    forms.dedup();
    count + forms.len()
}

fn part_two(data: &str) -> usize {
    let mut count = 0;
    let mut forms = HashSet::new();
    let mut new = true;
    for line in data.lines() {
        if new {
            new = false;
            forms.extend(line.chars());
            continue;
        }
        if line.is_empty() {
            new = true;
            count += forms.len();
            forms.clear();
            continue;
        }
        let line_set: HashSet<char> = line.chars().collect();
        forms.retain(|k| line_set.contains(k));
    }
    count + forms.len()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(6382, super::part_one(&data));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(3197, super::part_two(&data));
    }
}
