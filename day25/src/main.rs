static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
}

fn part_one(data: &str) -> usize {
    let pub_key_door = data.lines().next().unwrap().parse::<usize>().unwrap();
    let pub_key_card = data.lines().nth(1).unwrap().parse::<usize>().unwrap();
    let loop_size = determine_loop_size(pub_key_door);
    (0..loop_size).fold(1, |x, _| x * pub_key_card % 20201227)
}

fn determine_loop_size(pub_key: usize) -> usize {
    let mut x = 1;
    let mut loop_size = 0;
    let sub_no = 7;
    while x != pub_key {
        x = x * sub_no % 20201227;
        loop_size += 1;
    }
    loop_size
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(3015200, super::part_one(&data));
    }
}
