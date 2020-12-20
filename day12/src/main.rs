static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

struct Coord {
    x: isize,
    y: isize,
}

fn part_one(data: &str) -> usize {
    let mut pos = Coord { x: 0, y: 0 };
    let mut dir = Coord { x: 1, y: 0 };

    for line in data.lines() {
        let split = line.split_at(1);
        let cmd = split.0;
        let n = split.1.parse::<isize>().unwrap();
        match cmd {
            "N" => pos.y += n,
            "S" => pos.y -= n,
            "E" => pos.x += n,
            "W" => pos.x -= n,
            "F" => {
                pos.x += dir.x * n;
                pos.y += dir.y * n;
            }
            _ => {
                let mut angle = n;
                while angle > 0 {
                    let tmp_x = dir.x;
                    dir.x = if cmd == "L" { -dir.y } else { dir.y };
                    dir.y = if cmd == "L" { tmp_x } else { -tmp_x };
                    angle -= 90;
                }
            }
        }
    }
    (pos.x.abs() + pos.y.abs()) as usize
}

fn part_two(data: &str) -> usize {
    let mut pos = Coord { x: 0, y: 0 };
    let mut w_pos = Coord { x: 10, y: 1 };

    for line in data.lines() {
        let split = line.split_at(1);
        let cmd = split.0;
        let n = split.1.parse::<isize>().unwrap();
        match cmd {
            "N" => w_pos.y += n,
            "S" => w_pos.y -= n,
            "E" => w_pos.x += n,
            "W" => w_pos.x -= n,
            "F" => {
                pos.x += w_pos.x * n;
                pos.y += w_pos.y * n;
            }
            _ => {
                let mut angle = n;
                while angle > 0 {
                    let tmp_x = w_pos.x;
                    w_pos.x = if cmd == "L" { -w_pos.y } else { w_pos.y };
                    w_pos.y = if cmd == "L" { tmp_x } else { -tmp_x };
                    angle -= 90;
                }
            }
        }
    }
    (pos.x.abs() + pos.y.abs()) as usize
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(998, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(71586, super::part_two(&data));
    }
}
