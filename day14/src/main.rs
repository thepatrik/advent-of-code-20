use regex::Regex;
use std::collections::HashMap;
static FILENAME: &str = "input/data";
static BITMASK_LEN: usize = 36;

enum Operation {
    Mask((usize, usize)),
    Mem(usize, usize),
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let ops = parse(&data);
    println!("part one: {}", part_one(&ops));
    println!("part two: {}", part_two(&ops));
}

fn part_one(ops: &[Operation]) -> usize {
    let mut mask = (0, 0);
    let mut mem = HashMap::new();
    for op in ops {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Mem(addr, val) => {
                mem.insert(*addr, val & mask.0 | mask.1);
            }
        }
    }
    mem.values().sum()
}

fn part_two(ops: &[Operation]) -> usize {
    let mut mask = (0, 0);
    let mut mem = HashMap::new();
    for op in ops {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Mem(addr, val) => {
                for address in find_addresses(mask, *addr) {
                    mem.insert(address, *val);
                }
            }
        }
    }
    mem.values().sum()
}

fn find_addresses(mask: (usize, usize), addr: usize) -> Vec<usize> {
    let mut addr_base = addr | mask.1;
    let mut tmp_mask = mask.0 ^ mask.1;
    let mut addrs: Vec<usize> = Vec::new();

    for i in 0..BITMASK_LEN {
        let a = addr_base % 2;

        addrs = if tmp_mask % 2 == 1 {
            if addrs.is_empty() {
                vec![0, 1]
            } else {
                addrs
                    .iter()
                    .map(|address| vec![address + 2_usize.pow(i as u32), *address])
                    .flatten()
                    .collect()
            }
        } else if addrs.is_empty() {
            vec![a]
        } else {
            addrs
                .iter()
                .map(|address| address + (a) * 2_usize.pow(i as u32))
                .collect()
        };

        addr_base >>= 1;
        tmp_mask >>= 1;
    }
    addrs
}

fn parse(data: &str) -> Vec<Operation> {
    let re = Regex::new(r"mem\[(?P<addr>[0-9]*)\]").unwrap();
    let mut mask = "";
    data.lines()
        .map(|line| {
            let mut instr = line.split(" = ");
            let (lhs, rhs) = (instr.next().unwrap(), instr.next().unwrap());
            match lhs {
                "mask" => {
                    mask = rhs;
                    let and = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
                    let or = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
                    Operation::Mask((and, or))
                }
                _ => {
                    let addr = re.captures(lhs).unwrap()["addr"].parse::<usize>().unwrap();
                    let val = rhs.parse::<usize>().unwrap();
                    Operation::Mem(addr, val)
                }
            }
        })
        .collect()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let ops = super::parse(&data);
        assert_eq!(8570568288597, super::part_one(&ops));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let ops = super::parse(&data);
        assert_eq!(3289441921203, super::part_two(&ops));
    }
}
