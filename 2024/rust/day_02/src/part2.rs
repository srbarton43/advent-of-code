pub fn process(_input: &str) -> miette::Result<String> {
    let rows: Vec<Vec<i32>> = _input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let number_safe = rows
        .iter()
        .filter(|nums| {
            if is_safe(nums.as_slice()) {
                true
            } else {
                for i in 0..nums.len() {
                    let mut new_row = (*nums).clone();
                    new_row.remove(i);
                    if is_safe(new_row.as_slice()) {
                        return true;
                    }
                }
                false
            }
        })
        .count();
    Ok(number_safe.to_string())
}

fn is_safe(nums: &[i32]) -> bool {
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
        1 3 6 7 9
        28 28 27 26 23
        30 29 28 25 24 24
        30 29 28 25 24 24 24
        26 23 28 30 31";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
