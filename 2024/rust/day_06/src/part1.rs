use std::collections::HashSet;

pub fn process(_input: &str) -> miette::Result<String> {
    let map: Vec<Vec<Tile>> = _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Dot,
                    '^' => Tile::Direction(Dir::Up),
                    '>' => Tile::Direction(Dir::Right),
                    'v' => Tile::Direction(Dir::Down),
                    '<' => Tile::Direction(Dir::Left),
                    '#' => Tile::Obstacle,
                    c => panic!("Unsupported tile: {c}"),
                })
                .collect()
        })
        .collect();
    let (mut coord, mut dir) = get_start(&map);
    let mut visited = HashSet::from([coord]);
    loop {
        if let Ok(res) = update_position(&map, coord, dir) {
            (coord, dir) = res;
            visited.insert(coord);
        } else {
            break;
        }
        dbg!(coord);
        dbg!(dir);
    }
    Ok(visited.len().to_string())
}

type Coord = (usize, usize);

fn get_start(map: &Vec<Vec<Tile>>) -> (Coord, Dir) {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let Tile::Direction(direction) = tile {
                return ((i, j), *direction);
            }
        }
    }
    panic!("No starting direction!")
}

fn off_screen(coord: Coord, rows: usize, cols: usize) -> bool {
    let (r, c) = coord;
    r >= rows || r < 0 || c >= cols || c < 0
}

fn update_position(
    map: &Vec<Vec<Tile>>,
    mut coord: Coord,
    mut dir: Dir,
) -> Result<(Coord, Dir), &str> {
    let (r, c) = coord;
    let new_pos = match dir {
        Dir::Up => (r - 1, c),
        Dir::Right => (r, c + 1),
        Dir::Down => (r + 1, c),
        Dir::Left => (r, c - 1),
    };
    if let Ok(tile) = get_tile(map, new_pos) {
        match tile {
            Tile::Dot | Tile::Direction(_) => {
                coord = new_pos;
            }
            Tile::Obstacle => match dir {
                Dir::Up => dir = Dir::Right,
                Dir::Right => dir = Dir::Down,
                Dir::Down => dir = Dir::Left,
                Dir::Left => dir = Dir::Up,
            },
        }
    } else {
        return Err("Out of bounds");
    }

    Ok((coord, dir))
}

fn get_tile(map: &Vec<Vec<Tile>>, coord: Coord) -> Result<Tile, &str> {
    if let Some(row) = map.get(coord.0) {
        if let Some(tile) = row.get(coord.1) {
            return Ok(*tile);
        } else {
            return Err("Out of bounds on col index");
        }
    } else {
        return Err("Out of bounds on the row index");
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    Dot,
    Direction(Dir),
    Obstacle,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
