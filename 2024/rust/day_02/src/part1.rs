pub fn process(_input: &str) -> miette::Result<String> {
    let number_safe = _input.lines().filter(|l| is_safe(l)).count();
    Ok(number_safe.to_string())
}

fn is_safe(line: &str) -> bool {
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let mut failed_increasing = false;
    let mut failed_decreasing = false;

    // check increasing
    let mut prev = nums[0];
    for x in &nums[1..] {
        if *x <= prev || (x - prev > 3) {
            failed_increasing = true;
            break;
        }
        prev = *x;
    }

    // check decreasing
    let mut prev = nums[0];
    for x in &nums[1..] {
        if *x >= prev || (prev - x > 3) {
            failed_decreasing = true;
            break;
        }
        prev = *x;
    }
    !(failed_increasing && failed_decreasing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
