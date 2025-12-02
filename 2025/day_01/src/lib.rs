use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, usize},
    combinator::eof,
    multi::many1,
};

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    LEFT,
    RIGHT,
}

fn parse_direction(_input: &str) -> IResult<&str, Dir> {
    alt((
        tag("L").map(|_: &str| Dir::LEFT),
        tag("R").map(|_: &str| Dir::RIGHT),
    ))
    .parse(_input)
}

fn parse_line(_input: &str) -> IResult<&str, (Dir, usize)> {
    (
        parse_direction,
        usize,
        alt((newline.map(|_| ()), eof.map(|_| ()))),
    )
        .map(|(dir, positions, _)| (dir, positions))
        .parse(_input)
}

fn parse_input(_input: &str) -> IResult<&str, Vec<(Dir, usize)>> {
    many1(parse_line).parse(_input)
}

pub fn part1(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, parsed_input) = parse_input(input)?;
    let signed_dirs: Vec<isize> = parsed_input
        .iter()
        .map(|(dir, pos)| match dir {
            Dir::LEFT => -(*pos as isize),
            Dir::RIGHT => *pos as isize,
        })
        .collect();
    let positions: Vec<isize> = std::iter::once(50)
        .chain(signed_dirs.iter().scan(50, |position, signed_delta| {
            let new_position = (100 + *position + signed_delta) % 100;
            *position = new_position;
            Some(new_position)
        }))
        .collect();
    Ok(positions.iter().filter(|pos| **pos == 0).count())
}

pub fn part2(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, parsed_input) = parse_input(input)?;
    let signed_dirs: Vec<isize> = parsed_input
        .iter()
        .map(|(dir, pos)| match dir {
            Dir::LEFT => -(*pos as isize),
            Dir::RIGHT => *pos as isize,
        })
        .collect();

    let (count, _pos): (usize, isize) = signed_dirs.iter().fold((0, 50), |(acc, pos), delta| {
        let new_pos = (100 + pos + (*delta % 100)) % 100;
        let new_acc = acc
            + ((new_pos == 0)
                || (pos != 0 && *delta < 0 && new_pos > pos)
                || (*delta > 0 && new_pos < pos)) as usize
            + (((*delta).abs() as usize) / 100);
        (new_acc, new_pos)
    });

    return Ok(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_ex() {
        dbg!(EX_INPUT);
        assert_eq!(part1(EX_INPUT), Ok(3));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(part2(EX_INPUT), Ok(6));
    }
}
