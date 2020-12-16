static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let seats = seats(&data);
    *seats.iter().max().unwrap()
}

fn part_two(data: &str) -> usize {
    let mut seats = seats(&data);
    seats.sort();
    let mut ix = 0;
    while seats[ix] == seats[ix + 1] - 1 {
        ix += 1;
    }
    seats[ix] + 1
}

fn seats(data: &str) -> Vec<usize> {
    let mut seats: Vec<usize> = Vec::new();
    for line in data.lines() {
        let seat_id = seat_id(&line);
        seats.push(seat_id);
    }
    seats
}

fn seat_id(pass: &str) -> usize {
    let row = find(pass[..7].chars(), 127, 'F', 'B');
    let col = find(pass[7..].chars(), 7, 'L', 'R');
    row * 8 + col
}

fn find(chars: std::str::Chars<'_>, hi: usize, low_char: char, hi_char: char) -> usize {
    let mut hi_cp = hi;
    let mut lowest_col = 0;
    for c in chars {
        let delta = (hi_cp - lowest_col) / 2 + 1;
        if c == low_char {
            hi_cp -= delta;
        }
        if c == hi_char {
            lowest_col += delta;
        }
    }
    hi_cp
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(951, super::part_one(&data));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(653, super::part_two(&data));
    }
}
