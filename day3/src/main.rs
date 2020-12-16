static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let vec = str_to_vec(&data);
    println!("part one: {}", part_one(&vec));
    println!("part two: {}", part_two(&vec));
}

fn part_one(vec: &Vec<String>) -> usize {
    count_trees(&vec, 3, 1)
}

fn part_two(vec: &Vec<String>) -> usize {
    count_trees(&vec, 1, 1)
        * count_trees(&vec, 3, 1)
        * count_trees(&vec, 5, 1)
        * count_trees(&vec, 7, 1)
        * count_trees(&vec, 1, 2)
}

fn count_trees(vec: &Vec<String>, x_steps: usize, y_steps: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;

    for (y, row) in vec.iter().enumerate() {
        // Skip row?
        if y == 0 || y % y_steps != 0 {
            continue;
        }
        // Move!
        x += x_steps;
        if x >= row.len() {
            x -= row.len();
        }
        // Look for tree!
        if is_char_at(row, '#', x) {
            trees += 1;
        }
    }
    trees
}

fn is_char_at(s: &str, c: char, pos: usize) -> bool {
    let mut ix = 0;
    for c2 in s.chars() {
        if pos == ix {
            return c2 == c;
        }
        ix += 1;
    }
    false
}

fn str_to_vec(input: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for line in input.lines() {
        vec.push(line.to_string());
    }
    vec
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let vec = super::str_to_vec(&data);
        assert_eq!(278, super::part_one(&vec));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        let vec = super::str_to_vec(&data);
        assert_eq!(9709761600, super::part_two(&vec));
    }
}
