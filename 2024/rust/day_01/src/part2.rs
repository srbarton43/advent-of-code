use std::collections::HashMap;

use itertools::Itertools;

pub fn process(_input: &str) -> miette::Result<String> {
    let (list1, list2): (Vec<u32>, Vec<u32>) = _input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .into_iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    assert!(list1.len() == list2.len());
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for x in list2 {
        *counts.entry(x).or_default() += 1;
    }
    let score: u32 = list1
        .into_iter()
        .map(|x| *counts.entry(x).or_default() * x)
        .sum();
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
