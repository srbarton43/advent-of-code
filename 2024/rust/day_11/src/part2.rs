use num_traits::ops::euclid::Euclid;
use std::collections::HashMap;

use miette::miette;
use nom::{character::complete, multi::separated_list1, IResult};

const N: u8 = 75;

pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, stones) = parse_input(_input).map_err(|e| miette!("Error parsing: {e}"))?;
    let mut memo: HashMap<(u8, u64), u64> = HashMap::new();
    dbg!(&stones);
    let mut count: u64 = 0;
    for stone in stones {
        count += get_count(stone, &mut memo, N);
    }
    //dbg!(&stones);
    Ok(count.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, stones) = separated_list1(complete::space1, complete::u64)(input)?;
    Ok((input, stones))
}

fn get_count(stone: u64, memo: &mut HashMap<(u8, u64), u64>, remaining_blinks: u8) -> u64 {
    if let Some(count) = memo.get(&(remaining_blinks, stone)) {
        return *count;
    }
    if remaining_blinks == 0 {
        return 1;
    }
    let mut count = 0;
    if stone == 0 {
        count = get_count(1, memo, remaining_blinks - 1);
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let num_digits = stone.ilog10() + 1;
        let (left, right) = stone.div_rem_euclid(&10u64.pow(num_digits / 2));
        count = get_count(left, memo, remaining_blinks - 1)
            + get_count(right, memo, remaining_blinks - 1);
    } else {
        count = get_count(stone * 2024, memo, remaining_blinks - 1);
    }
    memo.insert((remaining_blinks, stone), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
