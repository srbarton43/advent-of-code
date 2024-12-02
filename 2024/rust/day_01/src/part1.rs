use std::iter::zip;

use itertools::Itertools;

pub fn process(_input: &str) -> miette::Result<String> {
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = _input
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
    list1.sort();
    list2.sort();
    let sum_differences: u32 = zip(list1.iter(), list2.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();
    Ok(sum_differences.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
