use miette::miette;
use nom::{
    bytes::complete::tag, character::complete, multi::many1, sequence::pair, IResult, Parser,
};

pub fn process(_input: &str) -> miette::Result<String> {
    let mut eqs = Vec::new();
    for line in _input.lines() {
        let (_, pair) = parse_line(line).map_err(|e| miette!("Error parsing line: {e}"))?;
        eqs.push(pair);
    }
    let out: u64 = eqs
        .iter()
        .filter(|(total, nums)| check_valid(*total, nums))
        .map(|(x, _)| x)
        .sum();
    Ok(out.to_string())
}

fn parse_line(line: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (line, first_num) = complete::u64(line)?;
    let (line, _) = tag(":")(line)?;
    let (line, list_nums) = many1(pair(complete::space1, complete::u64).map(|(_, num)| num))(line)?;
    Ok((line, (first_num, list_nums)))
}

fn check_valid(total: u64, nums: &Vec<u64>) -> bool {
    let num_slots = nums.len() - 1;
    assert!(num_slots >= 1);
    for bitmask in 0..(2u64.pow(num_slots as u32)) {
        let mut cur = nums[0];
        for (i, x) in nums[1..].iter().enumerate() {
            match bitmask >> i & 1 {
                0 => cur += x,
                1 => cur *= x,
                _ => panic!("Should not be non-bit value"),
            };
        }
        if cur == total {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
