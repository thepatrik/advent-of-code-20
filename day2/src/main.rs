use std::fs;

static FILENAME: &str = "input/data";

struct Password {
    min_constraint: usize,
    max_constraint: usize,
    char_constraint: char,
    s: String,
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("read error");
    println!("part one: {}", valid_passwords(&input, min_max_checker));
    println!("part two: {}", valid_passwords(&input, pos_checker));
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
    fn test_min_max_checker() {
        assert_eq!(
            true,
            super::min_max_checker(super::Password {
                min_constraint: 1,
                max_constraint: 3,
                char_constraint: 'a',
                s: "abcde".to_string()
            })
        );
    }
    #[test]
    fn test_pos_checker() {
        assert_eq!(
            true,
            super::pos_checker(super::Password {
                min_constraint: 1,
                max_constraint: 3,
                char_constraint: 'a',
                s: "abcde".to_string()
            })
        );
        assert_eq!(
            false,
            super::pos_checker(super::Password {
                min_constraint: 1,
                max_constraint: 2,
                char_constraint: 'b',
                s: "cdefg".to_string()
            })
        );
        assert_eq!(
            false,
            super::pos_checker(super::Password {
                min_constraint: 2,
                max_constraint: 9,
                char_constraint: 'c',
                s: "ccccccccc".to_string()
            })
        );
    }
}
