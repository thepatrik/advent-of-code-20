static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let cups = parse_input(&data);

    println!("Part one: {:?}", part_one(&cups));
    println!("Part two: {:?}", part_two(&cups));
}

fn part_one(input_cups: &Vec<usize>) -> usize {
    let cups = play(&input_cups, 100);
    let mut res = 0;
    let mut i = cups[1];
    while i != 1 {
        res = res * 10 + i;
        i = cups[i];
    }
    res
}

fn part_two(input_cups: &Vec<usize>) -> usize {
    let mut cups = input_cups.clone();
    (10..1000000 + 1).for_each(|x| cups.push(x));
    let res = play(&cups, 10000000);
    res[1] * res[res[1]]
}

fn play(input_cups: &Vec<usize>, n: usize) -> Vec<usize> {
    let max = *input_cups.iter().max().unwrap();
    let mut cups = vec![0; max + 1];
    (0..input_cups.len())
        .for_each(|i| cups[input_cups[i]] = input_cups[(i + 1) % input_cups.len()]);

    let mut cur_cup = input_cups[0];
    for _ in 0..n {
        let (n1, n2, n3) = (
            cups[cur_cup],
            cups[cups[cur_cup]],
            cups[cups[cups[cur_cup]]],
        );
        cups[cur_cup] = cups[n3];
        let mut dest_cup = if cur_cup == 1 { max } else { cur_cup - 1 };
        while dest_cup == n1 || dest_cup == n2 || dest_cup == n3 {
            dest_cup = if dest_cup == 1 { max } else { dest_cup - 1 };
        }
        cups[cur_cup] = cups[n3];
        cups[n3] = cups[dest_cup];
        cups[dest_cup] = n1;
        cur_cup = cups[cur_cup];
    }
    cups
}

fn parse_input(data: &str) -> Vec<usize> {
    let mut q: Vec<usize> = Vec::new();
    for line in data.lines() {
        for c in line.chars() {
            q.push(c.to_digit(10).unwrap() as usize);
        }
    }
    q
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let cups = super::parse_input(&data);
        assert_eq!(62934785, super::part_one(&cups));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let cups = super::parse_input(&data);
        assert_eq!(693659135400, super::part_two(&cups));
    }
}
