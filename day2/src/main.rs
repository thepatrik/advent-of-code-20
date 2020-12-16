use std::fs;

static FILENAME: &str = "input/data";

struct Password {
    min_constraint: usize,
    max_constraint: usize,
    char_constraint: char,
    s: String,
}

fn main() {
    let data = fs::read_to_string(FILENAME).expect("read error");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    valid_passwords(&data, min_max_checker)
}

fn part_two(data: &str) -> usize {
    valid_passwords(&data, pos_checker)
}

fn valid_passwords(input: &str, f: fn(Password) -> bool) -> usize {
    let mut valid_passwords = 0;
    for line in input.lines() {
        let pass = parse_password(line);
        if f(pass) {
            valid_passwords += 1
        }
    }
    valid_passwords
}

fn min_max_checker(pass: Password) -> bool {
    let count = str_char_count(&pass.s, pass.char_constraint);
    count <= pass.max_constraint && count >= pass.min_constraint
}

fn pos_checker(pass: Password) -> bool {
    let mut truths = 0;
    if str_has_char_at(&pass.s, pass.char_constraint, pass.min_constraint) {
        truths += 1;
    }
    if str_has_char_at(&pass.s, pass.char_constraint, pass.max_constraint) {
        truths += 1;
    }
    truths == 1
}

fn str_has_char_at(s: &str, c: char, pos: usize) -> bool {
    let mut ix = 1;
    for c2 in s.chars() {
        if pos == ix {
            return c2 == c;
        }
        ix += 1;
    }
    false
}

fn str_char_count(s: &str, c: char) -> usize {
    let mut count = 0;
    for c2 in s.chars() {
        if c == c2 {
            count += 1;
        }
    }
    count
}

fn parse_password(s: &str) -> Password {
    let v: Vec<&str> = s.split(" ").collect();
    let first: Vec<&str> = v[0].split("-").collect();
    Password {
        min_constraint: first[0].parse::<usize>().unwrap(),
        max_constraint: first[1].parse::<usize>().unwrap(),
        char_constraint: v[1].chars().next().unwrap(),
        s: v[2].to_string(),
    }
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(500, super::part_one(&data));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(313, super::part_two(&data));
    }
}
