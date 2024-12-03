use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::*,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, mults) = parse_input(_input).map_err(|err| miette!("parse failed: {err}"))?;
    let result: u32 = mults.iter().map(|(a, b)| a * b).sum();
    Ok(result.to_string())
}

type Multiplication = (u32, u32);

fn parse_input(input: &str) -> IResult<&str, Vec<Multiplication>> {
    many1(many_till(anychar, parse_mul).map(|(_discard, mult)| mult))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Multiplication> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")"))(input)?;
    Ok((input, pair))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
