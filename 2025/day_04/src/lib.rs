use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline},
    multi::{many1, separated_list1},
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn part1(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, arr) = parse_input(input)?;
    let count: usize = arr
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(c, ch)| **ch == '@' && is_valid(&arr, r, *c))
                .count()
        })
        .sum();
    Ok(count as u32)
}

fn parse_input(_input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, many1(alt((char('.'), char('@'))))).parse(_input)
}

fn is_valid(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbor_ct = 0;
    for (dr, dc) in neighbors.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            if grid[new_row][new_col] == '@' {
                neighbor_ct += 1;
            }
        }
    }
    //print_grid(grid);
    //dbg!(row, col, neighbor_ct);
    neighbor_ct < 4
}

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|ch| print!("{ch}"));
        print!("\n")
    });
}

pub fn part2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, mut arr) = parse_input(input)?;
    let mut count = 0;
    while let Some(first_valid) = arr.iter().enumerate().find_map(|(r, row)| {
        let hit = row
            .iter()
            .enumerate()
            .find(|(c, ch)| **ch == '@' && is_valid(&arr, r, *c));
        if let Some(hit) = hit {
            return Some((r, hit.0));
        }
        None
    }) {
        let (r, c) = first_valid;
        arr[r][c] = '.';
        count += 1
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INP: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_ex() {
        assert_eq!(part1(EX_INP), Ok(13));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(part2(EX_INP), Ok(43));
    }
}
