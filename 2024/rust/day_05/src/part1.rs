use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn process(_input: &str) -> miette::Result<String> {
    let (_input, rules) = parse_rules(_input).map_err(|e| miette!("Parsing rules failed: {e}"))?;
    let sequences =
        parse_sequences(_input).map_err(|e| miette!("Parsing sequences failed: {e}"))?;
    let rules_map = map_rules(rules);
    let valid_middle_nums = check_sequences(rules_map, sequences);
    let output: u32 = valid_middle_nums.iter().sum();
    Ok(output.to_string())
}

type Rule = (u32, u32);

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    let (input, (rules, _)) = many_till(get_tuple, tag("\n"))(input)?;
    Ok((input, rules))
}

fn get_tuple(input: &str) -> IResult<&str, Rule> {
    let (input, rul) = separated_pair(complete::u32, tag("|"), complete::u32)(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, rul))
}

fn parse_sequences(input: &str) -> Result<Vec<Vec<u32>>, &'static str> {
    let seqs = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Ok(seqs)
}

fn map_rules(rules: Vec<Rule>) -> HashMap<u32, HashSet<u32>> {
    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules.iter().for_each(|(a, b)| {
        if let Some(set) = rules_map.get_mut(a) {
            set.insert(*b);
        } else {
            let set = HashSet::from([*b]);
            rules_map.insert(*a, set);
        }
    });
    rules_map
}

fn check_sequences(rules_map: HashMap<u32, HashSet<u32>>, sequences: Vec<Vec<u32>>) -> Vec<u32> {
    let mut valid;
    let mut output = Vec::new();
    for seq in sequences {
        valid = true;
        for (i, x) in seq.iter().enumerate().rev() {
            println!("i: {i}");
            let set = rules_map.get(x);
            for j in (0u32..i as u32).rev() {
                println!("j: {j}");
                if set.is_some() && set.unwrap().contains(seq.get(j as usize).unwrap()) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            output.push(*seq.get(seq.len() / 2).unwrap());
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
