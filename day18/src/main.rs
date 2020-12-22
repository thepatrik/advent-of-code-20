static FILENAME: &str = "input/data";

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Number(usize),
    Parenthesis(bool),
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let ops = parse(&data);
    println!("part one: {}", part_one(&ops));
}

fn part_one(ops: &Vec<Vec<Operation>>) -> usize {
    let mut sum = 0;
    for op in ops {
        let res = calc(0, op);
        sum += res.1;
    }
    sum
}

fn calc(ix: usize, ops: &Vec<Operation>) -> (usize, usize) {
    let mut sum = 0;
    let mut op = Operation::Add;
    let mut skip = 0;
    let mut count = 0;
    for x in ix..ops.len() {
        match ops.get(x + skip) {
            Some(v) => match v {
                Operation::Add => op = Operation::Add,
                Operation::Multiply => op = Operation::Multiply,
                Operation::Number(n) => match op {
                    Operation::Add => {
                        sum += n;
                    }
                    Operation::Multiply => {
                        sum *= n;
                    }
                    _ => {}
                },
                Operation::Parenthesis(first) => {
                    if *first {
                        match op {
                            Operation::Add => {
                                let res = calc(x + skip + 1, ops);
                                skip += res.0;
                                sum += res.1;
                            }
                            Operation::Multiply => {
                                let res = calc(x + skip + 1, ops);
                                skip += res.0;
                                sum *= res.1;
                            }
                            _ => {}
                        }
                    } else {
                        return (count + skip + 1, sum);
                    }
                }
            },
            None => {}
        }
        count += 1;
    }
    (0, sum)
}

fn parse(data: &str) -> Vec<Vec<Operation>> {
    let mut matrix = Vec::new();
    for line in data.lines() {
        let mut ops = Vec::new();
        let mut current_val = 0;
        for c in line.chars() {
            match c {
                '(' => ops.push(Operation::Parenthesis(true)),
                ')' => {
                    if current_val > 0 {
                        ops.push(Operation::Number(current_val));
                        current_val = 0;
                    }
                    ops.push(Operation::Parenthesis(false));
                }
                '*' => ops.push(Operation::Multiply),
                '+' => ops.push(Operation::Add),
                '\n' | ' ' => {
                    if current_val > 0 {
                        ops.push(Operation::Number(current_val));
                        current_val = 0;
                    }
                }
                _ => {
                    current_val = if current_val > 0 {
                        current_val * 10 + (c.to_digit(10).unwrap() as usize)
                    } else {
                        c.to_digit(10).unwrap() as usize
                    };
                }
            }
        }
        if current_val > 0 {
            ops.push(Operation::Number(current_val));
        }
        matrix.push(ops);
    }
    matrix
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let ops = super::parse(&data);
        assert_eq!(464478013511, super::part_one(&ops));
    }
}
