use std::collections::{HashMap, HashSet};

static FILENAME: &str = "input/data";
static DIRECTIONS: [Coord; 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

type Coord = (isize, isize);

enum Direction {
    E,  // (1, 0)
    SE, // (1, -1)
    SW, // (0, -1)
    W,  // (-1, 0)
    NW, // (-1, 1)
    NE, // (0, 1)
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let tiles = flip_tiles(&data);

    tiles
        .iter()
        .map(|(_, is_black)| if *is_black { 1 } else { 0 })
        .sum::<usize>()
}

fn part_two(data: &str) -> usize {
    let mut tiles = flip_tiles(&data);

    fn flip(tiles: &HashMap<Coord, bool>) -> HashMap<Coord, bool> {
        let mut mem = HashMap::new();
        for coord in find_neighbours(&tiles) {
            let b = tiles.get(&coord).unwrap_or(&false);
            if is_black(coord, *b, &tiles) {
                mem.insert(coord, true);
            }
        }
        mem
    }

    let mut count = 0;
    for _ in 0..100 {
        tiles = flip(&tiles);
        count = tiles
            .iter()
            .map(|(_, is_black)| if *is_black { 1 } else { 0 })
            .sum();
    }
    count
}

fn is_black(coord: Coord, is_black: bool, mem: &HashMap<Coord, bool>) -> bool {
    let mut black_tile_neighbours = 0;
    for d in DIRECTIONS.iter() {
        if *mem.get(&(coord.0 + d.0, coord.1 + d.1)).unwrap_or(&false) {
            black_tile_neighbours += 1;
        }
    }
    match is_black {
        true => return black_tile_neighbours > 0 && black_tile_neighbours <= 2,
        false => return black_tile_neighbours == 2,
    }
}

fn find_neighbours(tiles: &HashMap<Coord, bool>) -> HashSet<Coord> {
    let mut neighbours = HashSet::new();
    for (coord, _) in tiles.clone() {
        neighbours.insert(coord);
        for d in DIRECTIONS.iter() {
            neighbours.insert((coord.0 + d.0, coord.1 + d.1));
        }
    }
    neighbours
}

fn flip_tiles(data: &str) -> HashMap<Coord, bool> {
    let tile_paths = parse_tile_paths(&data);
    let mut mem = HashMap::new();

    for tile in tile_paths {
        let mut current_pos = (0, 0);
        for path in tile {
            let pos;
            match path {
                Direction::E => pos = (1, 0),
                Direction::SE => pos = (1, -1),
                Direction::SW => pos = (0, -1),
                Direction::W => pos = (-1, 0),
                Direction::NW => pos = (-1, 1),
                Direction::NE => pos = (0, 1),
            }
            current_pos.0 += pos.0;
            current_pos.1 += pos.1;
        }
        match !mem.get(&current_pos).unwrap_or(&false).clone() {
            true => mem.insert(current_pos, true),
            false => mem.remove(&current_pos),
        };
    }

    mem
}

fn parse_tile_paths(data: &str) -> Vec<Vec<Direction>> {
    let mut paths = Vec::new();
    for line in data.lines() {
        let mut previous_char = '-';
        let mut dirs = Vec::new();
        for c in line.chars() {
            let mut dir_str = "".to_string();
            match previous_char {
                'n' | 's' => dir_str.push(previous_char),
                _ => {}
            }
            dir_str.push(c);
            match dir_str.as_str() {
                "e" => dirs.push(Direction::E),
                "se" => dirs.push(Direction::SE),
                "sw" => dirs.push(Direction::SW),
                "w" => dirs.push(Direction::W),
                "nw" => dirs.push(Direction::NW),
                "ne" => dirs.push(Direction::NE),
                _ => {}
            }
            previous_char = c;
        }
        paths.push(dirs);
    }
    paths
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(388, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(4002, super::part_two(&data));
    }
}
