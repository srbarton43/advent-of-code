use miette::miette;
use nom::{character::complete, multi::separated_list1, IResult};

const N: u8 = 25;

pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, mut stones) = parse_input(_input).map_err(|e| miette!("Error parsing: {e}"))?;
    dbg!(&stones);
    for i in 0..N {
        println!("iteration {}...", i + 1);
        stones = blink(stones);
        println!("count: {}", stones.len());
    }
    //dbg!(&stones);
    Ok(stones.len().to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, stones) = separated_list1(complete::space1, complete::u64)(input)?;
    Ok((input, stones))
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().chars().count() % 2 == 0 {
            let as_str = stone.to_string();
            let slen = as_str.chars().count();
            let (left, right) = as_str.split_at(slen / 2);
            let (left, right) = (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
