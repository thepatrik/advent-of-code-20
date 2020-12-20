use std::collections::HashMap;

static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let mut nums = parse(&data).expect("failed to parse data");
    nums.sort();

    let mut dict: std::collections::HashMap<usize, usize> = HashMap::new();
    let mut current = 0;
    for num in nums {
        let mut val = 1;
        let diff = num - current;
        match dict.get(&diff) {
            None => {}
            Some(v) => val = v + 1,
        };
        dict.insert(diff, val);
        current = num;
    }
    dict.get(&1).unwrap() * (dict.get(&3).unwrap() + 1)
}

fn part_two(data: &str) -> usize {
    let mut nums = parse(&data).expect("failed to parse data");
    nums.push(0);
    nums.sort();

    let mut ways: Vec<usize> = vec![1; nums.len()];

    if nums[nums.len() - 1] - nums[nums.len() - 3] <= 3 {
        ways[nums.len() - 3] = 2;
    }

    for x in (0..nums.len() - 3).rev() {
        let mut sum = ways[x + 1];
        for y in 2..4 {
            if nums[x + y] - nums[x] <= y {
                sum += ways[x + y]
            }
        }
        ways[x] = sum;
    }
    ways[0]
}

fn parse(data: &str) -> Option<Vec<usize>> {
    data.trim().split("\n").map(|s| s.parse().ok()).collect()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(1856, super::part_one(&data));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(2314037239808, super::part_two(&data));
    }
}
