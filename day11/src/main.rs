use std::convert::TryFrom;

static FILENAME: &str = "input/data";

const OCCUPIED: char = '#';
const EMPTY: char = 'L';

type Matrix = Vec<Vec<char>>;

trait MatrixMethods {
    fn get_char(&self, x: isize, y: isize) -> Option<char>;
}

impl MatrixMethods for Matrix {
    fn get_char(&self, x: isize, y: isize) -> Option<char> {
        fn in_bounds(matrix: &Matrix, x: isize, y: isize) -> bool {
            return (y >= 0 && y < matrix.len() as isize)
                && (x >= 0 && x < matrix[y as usize].len() as isize);
        }
        if !in_bounds(self, x, y) {
            return None;
        }
        Some(self[y as usize][x as usize])
    }
}

fn main() {
    let data = read_input(FILENAME);
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    find_occupied(data, 4, occupied_adjacent_seats)
}

fn part_two(data: &str) -> usize {
    find_occupied(data, 5, occupied_seats)
}

fn find_occupied(
    data: &str,
    occupied_seats_threshold: usize,
    occupied_seats_fn: fn(y: usize, x: usize, matrix: &Matrix) -> usize,
) -> usize {
    let mut seatings = data
        .split('\n')
        .map(|row| row.chars().collect())
        .collect::<Matrix>();
    let y_size = seatings.len();
    let x_size = seatings[0].len();
    let mut count = 0;
    let mut equilibrium = false;
    while !equilibrium {
        seatings = shift(
            &seatings,
            y_size,
            x_size,
            occupied_seats_threshold,
            occupied_seats_fn,
        );
        let occupied = seatings.iter().flatten().fold(0, |count, &seat| {
            count + if seat == OCCUPIED { 1 } else { 0 }
        });
        equilibrium = occupied == count;
        count = occupied;
    }
    count
}

fn shift(
    matrix: &Matrix,
    y_size: usize,
    x_size: usize,
    occupied_seats_threshold: usize,
    occupied_seats_fn: fn(y: usize, x: usize, matrix: &Matrix) -> usize,
) -> Matrix {
    let mut next_matrix: Vec<Vec<char>> = Vec::new();

    for y in 0..y_size {
        let mut nxt_vec: Vec<char> = Vec::new();
        for x in 0..x_size {
            let occupied_seats = occupied_seats_fn(y, x, &matrix);
            match matrix[y][x] {
                EMPTY => {
                    if occupied_seats == 0 {
                        nxt_vec.push(OCCUPIED);
                    } else {
                        nxt_vec.push(matrix[y][x])
                    }
                }
                OCCUPIED => {
                    if occupied_seats >= occupied_seats_threshold {
                        nxt_vec.push(EMPTY);
                    } else {
                        nxt_vec.push(matrix[y][x])
                    }
                }
                _ => nxt_vec.push(matrix[y][x]),
            }
        }
        next_matrix.push(nxt_vec);
    }
    next_matrix
}

fn occupied_adjacent_seats(y: usize, x: usize, matrix: &Matrix) -> usize {
    let ix = isize::try_from(x).unwrap();
    let iy = isize::try_from(y).unwrap();
    let mut occupied = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if let Some(c) = matrix.get_char(ix + dx, iy + dy) {
                if c == OCCUPIED {
                    occupied += 1
                }
            }
        }
    }
    occupied
}

fn occupied_seats(y: usize, x: usize, matrix: &Matrix) -> usize {
    let ix = isize::try_from(x).unwrap();
    let iy = isize::try_from(y).unwrap();
    let mut occupied = 0;

    fn occupied_seat(y: isize, x: isize, y_dir: isize, x_dir: isize, matrix: &Matrix) -> usize {
        match matrix.get_char(x, y) {
            Some(c) => match c {
                EMPTY => return 0,
                OCCUPIED => return 1,
                _ => return occupied_seat(y + y_dir, x + x_dir, y_dir, x_dir, matrix),
            },
            None => return 0,
        }
    };

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            occupied += occupied_seat(iy + dy, ix + dx, dy, dx, matrix);
        }
    }
    occupied
}

fn read_input(filename: &str) -> String {
    let mut data = std::fs::read_to_string(filename).expect("could not read file");
    data.truncate(data.len() - 1);
    data
}

mod tests {
    #[test]
    fn test_occupied_seats() {
        assert_eq!(
            8,
            super::occupied_seats(
                4,
                3,
                &vec![
                    vec!['.', '.', '.', '.', '.', '.', '.', '#', '.'],
                    vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
                    vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '#', 'L', '.', '.', '.', '.', '#'],
                    vec!['.', '.', '.', '.', '#', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
                ],
            )
        );
    }

    #[test]
    fn test_part_one() {
        let data = super::read_input("input/data");
        assert_eq!(2406, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = super::read_input("input/data");
        assert_eq!(2149, super::part_two(&data));
    }
}
