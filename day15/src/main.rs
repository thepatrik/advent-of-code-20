fn main() {
    println!("part one: {}", find_last(&[1, 0, 15, 2, 10, 13], 2020));
    println!("part one: {}", find_last(&[1, 0, 15, 2, 10, 13], 30000000));
}

fn find_last(nums: &[usize], nth_num: usize) -> usize {
    let mut cache = vec![std::usize::MAX; nth_num];
    for (i, &n) in nums.iter().enumerate() {
        cache[n] = i;
    }

    let mut spoken = nums[nums.len() - 1];
    for i in nums.len()..nth_num {
        let n = cache[spoken];
        cache[spoken] = i - 1;
        spoken = (i - 1).saturating_sub(n);
    }
    spoken
}

mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(211, super::find_last(&[1, 0, 15, 2, 10, 13], 2020));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(2159626, super::find_last(&[1, 0, 15, 2, 10, 13], 30000000));
    }
}
