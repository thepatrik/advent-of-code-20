use std::collections::HashSet;
use std::iter::FromIterator;

static FILENAME: &str = "input/data";
static MONSTER: [(isize, isize); 15] = [
    (-1, 18),
    (0, 0),
    (0, 5),
    (0, 6),
    (0, 11),
    (0, 12),
    (0, 17),
    (0, 18),
    (0, 19),
    (1, 1),
    (1, 4),
    (1, 7),
    (1, 10),
    (1, 13),
    (1, 16),
];
static EDGES: [Edge; 8] = [
    Edge {
        side: Side::TOP,
        flipped: false,
    },
    Edge {
        side: Side::RIGHT,
        flipped: false,
    },
    Edge {
        side: Side::BOTTOM,
        flipped: false,
    },
    Edge {
        side: Side::LEFT,
        flipped: false,
    },
    Edge {
        side: Side::TOP,
        flipped: true,
    },
    Edge {
        side: Side::RIGHT,
        flipped: true,
    },
    Edge {
        side: Side::BOTTOM,
        flipped: true,
    },
    Edge {
        side: Side::LEFT,
        flipped: true,
    },
];

#[derive(Clone, Copy, PartialEq)]
enum Side {
    TOP = 1,
    RIGHT,
    BOTTOM,
    LEFT,
}

#[derive(Clone, Copy)]
struct Edge {
    side: Side,
    flipped: bool,
}

#[derive(Clone, Default)]
struct Tile {
    id: usize,
    image: Vec<Vec<char>>,
}

impl Tile {
    fn edge(&self, edge: Edge) -> Vec<char> {
        let mut image: Vec<char> = Vec::new();
        let len = self.image.len();
        match edge.side {
            Side::TOP => image = self.image[0].clone(),
            Side::RIGHT => {
                for y in 0..len {
                    image.push(self.image[y][len - 1]);
                }
            }
            Side::BOTTOM => {
                for x in 0..len {
                    image.push(self.image[len - 1][x]);
                }
            }
            Side::LEFT => {
                for y in 0..len {
                    image.push(self.image[y][0]);
                }
            }
        }
        if edge.flipped {
            image.reverse();
        }
        image
    }

    fn find_edge(&self, pattern: &Vec<char>) -> Option<Edge> {
        for edge in &EDGES {
            if self.edge(*edge) == *pattern {
                return Some(*edge);
            }
        }
        None
    }
}

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let tiles = parse_tiles(&data);
    let sorted = sort_tiles(&tiles);

    println!("part one: {}", part_one(&sorted));
    println!("part two: {}", part_two(&sorted));
}

fn part_one(tiles: &Vec<Vec<Tile>>) -> usize {
    let corners = [
        tiles[0][0].id,
        tiles[0][tiles[0].len() - 1].id,
        tiles[tiles.len() - 1][0].id,
        tiles[tiles.len() - 1][tiles.len() - 1].id,
    ];

    corners.iter().product()
}

fn part_two(tiles: &Vec<Vec<Tile>>) -> usize {
    let image = build_image(tiles);
    count_water_roughness(image)
}

fn sort_tiles(tiles: &Vec<Tile>) -> Vec<Vec<Tile>> {
    let size = f32::sqrt(tiles.len() as f32) as usize;
    let mut image = vec![vec![Tile::default(); size]; size];

    image[size - 1][0] = get_corner(&tiles, Side::TOP, Side::LEFT);

    // map first row
    for x in 1..size {
        let y = size - 1;
        let left_tile = image[y][x - 1].clone();
        let matched_tile = find_tile(&left_tile, Side::RIGHT, &tiles).unwrap();
        image[y][x] = matched_tile;
    }

    // map the rest...
    for y in (0..size - 1).rev() {
        for x in 0..size {
            let above_tile = image[y + 1][x].clone();
            let matched_tile = find_tile(&above_tile, Side::BOTTOM, &tiles).unwrap();
            image[y][x] = matched_tile;
        }
    }
    image
}

fn get_corner(tiles: &Vec<Tile>, side1: Side, side2: Side) -> Tile {
    let mut top_left = Tile::default();
    for t in tiles {
        // find a corner
        if count_neighbours(t, tiles) == 2 {
            top_left = t.clone();
            break;
        }
    }

    while find_tile(&top_left, side1, tiles).is_some()
        || find_tile(&top_left, side2, tiles).is_some()
    {
        top_left = rotate(&top_left, 90);
    }
    top_left
}

fn count_neighbours(tile: &Tile, tiles: &Vec<Tile>) -> usize {
    let mut count = 0;
    for t in tiles {
        if tile.id == t.id {
            continue;
        }
        if are_neighbours(tile, t) {
            count += 1;
        }
    }
    count
}

fn build_image(tiles: &Vec<Vec<Tile>>) -> Tile {
    let width = tiles[0][0].image.len() - 2;
    let size = tiles.len();
    let mut image: Vec<Vec<char>> = vec![Vec::new(); width * size];

    let mut y_offset = 0;

    for y in (0..size).rev() {
        for x in 0..size {
            let tile = trim(&tiles[y][x]);

            let mut i = 0;
            for row in tile.image.clone() {
                image[(y_offset + i)].extend(row);
                i += 1;
            }
        }
        y_offset += width;
    }

    Tile { id: 0, image }
}

fn count_water_roughness(mut tile: Tile) -> usize {
    let hashes = tile.image.iter().flatten().filter(|&&c| c == '#').count();
    let monsters = HashSet::from_iter(MONSTER.iter().cloned());
    for r in 0..EDGES.len() {
        let monster_sum = count_monsters(&tile.image, &monsters);
        if monster_sum > 0 {
            return hashes - monster_sum * monsters.len();
        }
        tile = rotate(&tile, 90);
        if r == 3 {
            tile = flip(&tile, false);
        }
    }
    unreachable!()
}

fn count_monsters(image: &[Vec<char>], monster_coords: &HashSet<(isize, isize)>) -> usize {
    let hashtags = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect::<HashSet<(isize, isize)>>();

    hashtags
        .iter()
        .filter(|(i, j)| {
            monster_coords
                .iter()
                .map(|(di, dj)| (i + di, j + dj))
                .all(|pos| hashtags.contains(&pos))
        })
        .count()
}

fn find_tile(tile: &Tile, side: Side, tiles: &Vec<Tile>) -> Option<Tile> {
    let edge = tile.edge(Edge {
        side,
        flipped: false,
    });

    for t in tiles {
        if t.id == tile.id {
            continue;
        }
        match t.find_edge(&edge) {
            Some(opts) => {
                let mut matched = t.clone();
                match get_rotation(side, opts.side) {
                    Some(degrees) => matched = rotate(&matched, degrees),
                    None => {}
                }

                if matched.find_edge(&edge).unwrap().flipped {
                    matched = flip(&matched, side == Side::RIGHT || side == Side::LEFT);
                }

                return Some(matched);
            }
            None => {}
        }
    }
    None
}

fn get_rotation(current: Side, wanted: Side) -> Option<usize> {
    let mut n = (current as isize - wanted as isize) + 2;
    if n == 0 {
        return None;
    }
    if n < 0 {
        n += 4;
    }

    Some(n as usize * 90)
}

fn are_neighbours(tile_1: &Tile, tile_2: &Tile) -> bool {
    for edge in &EDGES {
        if tile_1.find_edge(&tile_2.edge(*edge)).is_some() {
            return true;
        }
    }
    false
}

fn rotate(tile: &Tile, degrees: usize) -> Tile {
    let mut image = tile.image.clone();
    for _x in 0..degrees / 90 {
        let (h, w) = (image.len(), image[0].len());
        let mut vec = vec![vec!['.'; w]; h];
        for i in 0..h {
            for j in 0..w {
                vec[j][w - 1 - i] = image[i][j].clone();
            }
        }
        image = vec;
    }
    Tile { id: tile.id, image }
}

fn flip(tile: &Tile, vertical: bool) -> Tile {
    let mut image: Vec<Vec<char>> = Vec::new();
    if vertical {
        for y in (0..tile.image.len()).rev() {
            image.push(tile.image[y].clone());
        }
        return Tile { id: tile.id, image };
    }

    for y in 0..tile.image.len() {
        let mut tmp = tile.image[y].clone();
        tmp.reverse();
        image.push(tmp);
    }
    Tile { id: tile.id, image }
}

fn trim(tile: &Tile) -> Tile {
    let mut image: Vec<Vec<char>> = tile.image.clone();
    image.remove(image.len() - 1);
    image.remove(0);

    for y in 0..image.len() {
        let len = image[y].len();
        image[y].remove(len - 1);
        image[y].remove(0);
    }
    Tile { id: tile.id, image }
}

fn parse_tiles(input_data: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for s in input_data.split("\n\n") {
        let mut lines = s.lines();
        let id = lines.next().unwrap();
        let id = id[5..id.len() - 1].parse::<usize>().unwrap();
        let image = lines
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        tiles.push(Tile { id, image });
    }
    tiles
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string("input/data").expect("could not read file");
        let tiles = super::parse_tiles(&data);
        let sorted = super::sort_tiles(&tiles);

        assert_eq!(2699020245973, super::part_one(&sorted));
    }
    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string("input/data").expect("could not read file");
        let tiles = super::parse_tiles(&data);
        let sorted = super::sort_tiles(&tiles);

        assert_eq!(2012, super::part_two(&sorted));
    }
}
