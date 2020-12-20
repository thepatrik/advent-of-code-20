static FILENAME: &str = "input/data";
use std::collections::HashSet;

type Operations<'a> = Vec<(&'a str, isize)>;

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let ops = parse_operations(&data);
    sum(&ops).0
}

fn part_two(data: &str) -> usize {
    let mut ops = parse_operations(&data);
    let mut counter = 0;
    loop {
        match find_instr_at("jmp", counter, &ops) {
            None => return 0,
            Some(ix) => {
                let replaced = ops[ix];
                ops[ix] = ("nop", 0);
                counter += 1;
                let res = sum(&ops);
                if res.1 == ops.len() {
                    return res.0;
                }
                ops[ix] = replaced;
            }
        }
    }
}

fn sum(ops: &Operations) -> (usize, usize) {
    let mut steps: HashSet<isize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut step: isize = 0;
    while !steps.contains(&step) && step < ops.len() as isize {
        steps.insert(step);
        let op = ops[step as usize];
        match op.0 {
            "acc" => {
                step += 1;
                accumulator += op.1;
            }
            "jmp" => step += op.1,
            _ => step += 1,
        }
    }
    (accumulator as usize, step as usize)
}

fn find_instr_at(instr: &str, ix: usize, ops: &Operations) -> Option<usize> {
    let mut counter = 0;
    for x in 0..ops.len() - 1 {
        if ops[x].0 == instr {
            if counter == ix {
                return Some(x);
            }
            counter += 1
        }
    }
    None
}

fn parse_operations(data: &str) -> Operations {
    let mut ops = Vec::new();
    for line in data.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        ops.push((split[0], split[1].parse::<isize>().unwrap()));
    }
    ops
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(1337, super::part_one(&data)); // l33t!
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(1358, super::part_two(&data));
    }
}
