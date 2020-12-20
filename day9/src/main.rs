static FILENAME: &str = "input/data";
static PREEMBLE: usize = 25;

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let nums = parse(&data).expect("failed to parse data");
    let num = part_one(&nums, PREEMBLE).unwrap();
    println!("part one: {}", num);
    println!("part two: {}", part_two(&nums, num).unwrap());
}

fn part_one(nums: &[usize], size: usize) -> Option<usize> {
    for x in 0..nums.len() {
        if !is_num_ok(nums[x + size], &nums[x..x + size]) {
            return Some(nums[x + size]);
        }
    }
    None
}

fn part_two(nums: &[usize], num: usize) -> Option<usize> {
    for x in 0..nums.len() {
        match find_match(num, x, &nums) {
            Some(v) => return Some(v),
            None => {}
        }
    }
    None
}

fn is_num_ok(num: usize, nums: &[usize]) -> bool {
    for x in 0..nums.len() {
        for y in 0..nums.len() {
            if x == y {
                continue;
            }
            if nums[x] + nums[y] == num {
                return true;
            }
        }
    }
    false
}

fn find_match(num: usize, from_ix: usize, nums: &[usize]) -> Option<usize> {
    let mut count = 0;
    let mut x = from_ix;
    while count < num && x < nums.len() {
        count += nums[x];
        if count == num {
            let slice = &nums[from_ix..x];
            let sum = slice.iter().min().unwrap() + slice.iter().max().unwrap();
            return Some(sum);
        }
        x += 1;
    }
    None
}

fn parse(data: &str) -> Option<Vec<usize>> {
    data.trim().split("\n").map(|s| s.parse().ok()).collect()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).expect("failed to parse data");
        assert_eq!(
            530627549,
            super::part_one(&nums, super::PREEMBLE).expect("failed part one")
        );
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let nums = super::parse(&data).expect("failed to parse data");
        assert_eq!(
            77730285,
            super::part_two(&nums, 530627549).expect("failed part two")
        );
    }
}
