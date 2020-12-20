use std::collections::HashMap;

static FILENAME: &str = "input/data";

struct Bus {
    no: usize,
    departure: usize,
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let time = parse_time(&data);
    let buses: Vec<Bus> = parse_buses(&data);
    println!("{}", part_one(time, &buses));
    println!("{}", part_two(&buses));
}

fn part_one(time: usize, buses: &Vec<Bus>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for bus in buses {
        let modulus = time as f32 % bus.no as f32;
        mem.insert(bus.no, bus.no - modulus as usize);
    }

    let mut lowest = (0, std::usize::MAX);
    for (k, v) in mem {
        if v < lowest.1 {
            lowest = (k, v);
        }
    }

    lowest.0 * lowest.1
}

fn part_two(buses: &Vec<Bus>) -> usize {
    let mut time = 0;
    let mut lcd = 1;
    for bus in buses.iter() {
        while (time + bus.departure) % bus.no != 0 {
            time += lcd;
        }
        lcd *= bus.no;
    }
    time
}

fn parse_time(data: &str) -> usize {
    let mut time = 0;
    let mut i = 0;
    for line in data.lines() {
        match i {
            0 => time = line.parse::<usize>().unwrap(),
            _ => (),
        }
        i += 1;
    }
    time
}

fn parse_buses(data: &str) -> Vec<Bus> {
    let mut buses: Vec<Bus> = Vec::new();
    let mut i = 0;
    for line in data.lines() {
        match i {
            1 => {
                let opt: Option<Vec<String>> =
                    line.trim().split(",").map(|s| s.parse().ok()).collect();

                let vec = opt.unwrap();
                let mut count = 0;
                for s in vec {
                    if s != "x" {
                        let bus = Bus {
                            no: s.parse::<usize>().unwrap(),
                            departure: count,
                        };
                        buses.push(bus);
                    }
                    count += 1;
                }
            }
            _ => (),
        }
        i += 1;
    }
    buses
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let time = super::parse_time(&data);
        let buses = super::parse_buses(&data);
        assert_eq!(3385, super::part_one(time, &buses));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let buses = super::parse_buses(&data);
        assert_eq!(600689120448303, super::part_two(&buses));
    }
}
