use miette::miette;
use nom::{character::complete, multi::many1, sequence::pair, IResult, Parser};

pub fn process(_input: &str) -> miette::Result<String> {
    let (_, files) = parse_input(_input).map_err(|e| miette!("Error parsing input: {e}"))?;
    let mut blocks_arr: Vec<Block> = Vec::new();
    for (id, (size, space)) in files.iter().enumerate() {
        for _ in 0..*size {
            blocks_arr.push(Block::File(id as u32));
        }
        for _ in 0..*space {
            blocks_arr.push(Block::Empty);
        }
    }
    let mut left = 0;
    while blocks_arr[left] != Block::Empty {
        left += 1;
    }

    let mut right = blocks_arr.len() - 1;

    while left < right {
        while blocks_arr[left] != Block::Empty {
            left += 1;
            if left > right {
                break;
            }
        }
        while blocks_arr[right] == Block::Empty {
            right -= 1;
            if right < left {
                break;
            }
        }
        if left < right {
            blocks_arr[left] = blocks_arr[right];
            blocks_arr[right] = Block::Empty;
        }
        left += 1;
        right -= 1;
    }
    let checksum = get_checksum(blocks_arr);
    Ok(checksum.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Empty,
    File(u32),
}

fn get_checksum(blocks_arr: Vec<Block>) -> u64 {
    blocks_arr.iter().enumerate().fold(0, |acc, (slot, block)| {
        if let Block::File(id) = block {
            acc + *id as u64 * slot as u64
        } else {
            acc
        }
    })
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    // strip newline char if exists
    let input = input.strip_suffix('\n').unwrap_or(input);

    let (input, mut pairs) = many1(
        pair(complete::anychar, complete::anychar)
            .map(|(a, b): (char, char)| (a.to_digit(10).unwrap(), b.to_digit(10).unwrap())),
    )(input)?;

    // get trailing char
    let (input, x) = complete::anychar(input)?;
    let x = x.to_digit(10).unwrap();
    pairs.push((x, 0));
    Ok((input, pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        println!("{}", input.len());
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
