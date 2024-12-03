use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::*,
    combinator::eof,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn process(_input: &str) -> miette::Result<String> {
    let mut result: u32 = 0;
    let mut input = _input;
    let mut to_parse: &str;
    loop {
        // parse input in "do()" block and sum the multiplications
        let do_result = parse_do(input);
        if do_result.is_ok() {
            // read until next "don't()" bloc
            (input, to_parse) = do_result.unwrap();
            let (_, cur_mults) = parse_mults(to_parse)
                .map_err(|err| miette!("Error parsing multiplications: {err}"))?;
            result += cur_mults.iter().map(|(a, b)| a * b).sum::<u32>();
        } else {
            // read to end of file
            let (_, cur_mults) = parse_mults(input)
                .map_err(|err| miette!("Error parsing multiplications: {err}"))?;
            result += cur_mults.iter().map(|(a, b)| a * b).sum::<u32>();
            break;
        }
        // throw away the stuff in the "don't()" block until the next "do()" block
        (input, _) = parse_dont(input).map_err(|err| miette!("Error parsing until do: {err}"))?;
    }
    Ok(result.to_string())
}

fn parse_do(input: &str) -> IResult<&str, &str> {
    take_until("don't()")(input)
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
    take_until("do()")(input)
}

type Multiplication = (u32, u32);

fn parse_mults(input: &str) -> IResult<&str, Vec<Multiplication>> {
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
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
