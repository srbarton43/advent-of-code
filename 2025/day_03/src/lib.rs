use nom::{
    IResult, Parser,
    character::complete::{digit1, newline},
    multi::separated_list1,
};

pub fn part1(input: &str) -> Result<u16, nom::Err<nom::error::Error<&str>>> {
    dbg!(&input);
    let (_, banks) = parse_input(&input)?;
    Ok(banks.iter().map(|b| get_max_joltage(b)).sum())
}

pub fn part2(input: &str) -> Result<u64, nom::Err<nom::error::Error<&str>>> {
    dbg!(&input);
    let (_, banks) = parse_input(&input)?;
    Ok(banks.iter().map(|b| get_max_joltage2(b)).sum())
}

fn parse_input(_input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (_input, lines) = separated_list1(newline, digit1).parse(_input)?;
    dbg!(&lines);
    let res: Vec<Vec<u8>> = lines
        .iter()
        .map(|s| s.chars().map(|ch| (ch as u32 - '0' as u32) as u8).collect())
        .collect();
    Ok((_input, res))
}

fn get_max_joltage(joltages: &Vec<u8>) -> u16 {
    let mut max_left_idx = 0;
    for (i, x) in (&joltages[..joltages.len() - 1]).iter().enumerate() {
        if x > &joltages[max_left_idx] {
            max_left_idx = i;
        }
    }
    let mut max_right_idx = max_left_idx + 1;
    for (i, x) in (&joltages[..]).iter().enumerate().skip(max_left_idx + 1) {
        if x > &joltages[max_right_idx] {
            max_right_idx = i;
        }
    }

    let res = 10 * joltages[max_left_idx] as u16 + joltages[max_right_idx] as u16;
    res
}

const DIGITS: usize = 12;

fn get_max_joltage2(joltages: &Vec<u8>) -> u64 {
    let mut indices: [u8; DIGITS] = [0; DIGITS];

    // get first index
    for (i, x) in (&joltages[..joltages.len() - (DIGITS - 1)])
        .iter()
        .enumerate()
    {
        if x > &joltages[indices[0] as usize] {
            indices[0] = i as u8;
        }
    }

    for index in 1..DIGITS {
        indices[index] = indices[index - 1] + 1;
        for (i, x) in (&joltages[..joltages.len() - (DIGITS - 1 - index)])
            .iter()
            .enumerate()
            .skip(indices[index - 1] as usize + 1)
        {
            if x > &joltages[indices[index] as usize] {
                indices[index] = i as u8;
            }
        }
    }

    let mut res: u64 = 0;
    for index in 0..DIGITS {
        res = 10 * res + (joltages[indices[index] as usize] as u64);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INP: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_ex() {
        assert_eq!(part1(EX_INP), Ok(357));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(part2(EX_INP), Ok(3121910778619));
    }

    #[test]
    fn line3() {
        assert_eq!(
            get_max_joltage2(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
    }
}
