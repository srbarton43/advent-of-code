use nom::{
    IResult, Parser, bytes::complete::tag, multi::separated_list1, sequence::separated_pair,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    left: usize,
    right: usize,
}

pub fn part1(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, ranges) = parse_input(input)?;
    Ok(ranges.iter().map(|range| count_invalid(*range)).sum())
}

pub fn part2(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, ranges) = parse_input(input)?;
    let factors_lut: [Vec<usize>; 10] = [
        vec![1],
        vec![2],
        vec![3],
        vec![2, 4],
        vec![5],
        vec![2, 3, 6],
        vec![7],
        vec![2, 4, 8],
        vec![3, 9],
        vec![2, 5, 10],
    ];
    Ok(ranges
        .iter()
        .map(|range| count_invalid2(&factors_lut, *range))
        .sum())
}

fn parse_input(_input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(tag(","), parse_range).parse(_input)
}

fn parse_range(_input: &str) -> IResult<&str, Range> {
    separated_pair(
        nom::character::complete::usize,
        tag("-"),
        nom::character::complete::usize,
    )
    .map(|(left, right)| Range { left, right })
    .parse(_input)
}

fn count_invalid(range: Range) -> usize {
    (range.left..=range.right)
        .filter(|x| check_invalid(*x))
        .sum()
}

fn count_invalid2(factors_lut: &[Vec<usize>; 10], range: Range) -> usize {
    (range.left..=range.right)
        .filter(|x| check_invalid2(&factors_lut[x.ilog10() as usize], *x))
        .sum()
}

fn check_invalid(num: usize) -> bool {
    (num.ilog10() % 2 != 0) && halves_match(num)
}

fn halves_match(num: usize) -> bool {
    let s = num.to_string();
    s[..s.len() / 2] == s[s.len() / 2..]
}

fn check_invalid2(factors: &Vec<usize>, num: usize) -> bool {
    factors.iter().any(|n| check_n_groups(*n, num))
}

fn check_n_groups(n: usize, num: usize) -> bool {
    let s = num.to_string();
    if s.len() == 1 {
        return false;
    }
    if n == s.len() {
        let mut chars_iter = s.chars().into_iter();
        let head = chars_iter.next().unwrap();
        return chars_iter.all(|ch| ch == head);
    }
    let mut chunks_iter = s.as_bytes().chunks_exact(n).into_iter();
    let head = chunks_iter.next().unwrap();
    chunks_iter.all(|chunk| chunk == head)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INP: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EX_INP), Ok(1227775554));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EX_INP), Ok(4174379265))
    }

    #[test]
    fn invalid() {
        let num = 9191;
        assert!(check_invalid(num));
        let num = 11;
        assert!(check_invalid(num));
    }
    #[test]
    fn valid() {
        let num = 12;
        assert!(!check_invalid(num));
        let num = 111;
        assert!(!check_invalid(num));
    }

    #[test]
    fn invalid2() {
        let num = 9191;
        assert!(check_invalid2(&vec![2, 4], num));
        let num = 919191;
        assert!(check_invalid2(&vec![2, 3, 6], num));
        let num = 11;
        assert!(check_invalid2(&vec![2], num));
        let num = 111;
        assert!(check_invalid2(&vec![3], num));
        let num = 123123;
        assert!(check_invalid2(&vec![2, 3, 6], num));
    }
    #[test]
    fn valid2() {
        let num = 12;
        assert!(!check_invalid2(&vec![2], num));
        let num = 123;
        assert!(!check_invalid2(&vec![3], num));
        let num = 113;
        assert!(!check_invalid2(&vec![3], num));
        let num = 1111117;
        assert!(!check_invalid2(&vec![7], num));
        let num = 3;
        assert!(!check_invalid2(&vec![1], num));
    }
}
