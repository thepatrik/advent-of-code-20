use std::collections::HashSet;

static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let nums = parse(&data).unwrap();
    println!("part one: {}", part_one(&nums, 2020).unwrap());
    println!("part two: {}", part_two(&nums, 2020).unwrap());
}

fn part_one(nums: &Vec<isize>, magic_num: isize) -> Option<isize> {
    let dict: HashSet<_> = nums.iter().collect();
    for num_1 in nums {
        match dict.get(&(magic_num - num_1)) {
            Some(num_2) => return Some(num_1 * *num_2),
            None => {}
        }
    }
    None
}

fn part_two(nums: &Vec<isize>, magic_num: isize) -> Option<isize> {
    let dict: HashSet<_> = nums.iter().collect();
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            let num_1 = nums[i];
            let num_2 = nums[j];

            match dict.get(&(magic_num - (num_1 + num_2))) {
                Some(num_3) => return Some(num_1 * num_2 * *num_3),
                None => {}
            }
        }
    }
    None
}

fn parse(data: &str) -> Option<Vec<isize>> {
    data.trim().split("\n").map(|s| s.parse().ok()).collect()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).unwrap();
        assert_eq!(539851, super::part_one(&nums, 2020).unwrap());
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).unwrap();
        assert_eq!(212481360, super::part_two(&nums, 2020).unwrap());
    }
}
