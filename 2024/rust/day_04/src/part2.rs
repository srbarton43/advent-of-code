pub fn process(_input: &str) -> miette::Result<String> {
    let mut arr: Vec<Vec<char>> = Vec::new();
    for line in _input.lines() {
        arr.push(line.chars().collect());
    }
    let mut res = 0;
    for (i, row) in arr.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'A' {
                res += check_x(&arr, i, j) as u32;
            }
        }
    }
    Ok(res.to_string())
}

fn check_x(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i > arr.len() - 2 || j > arr.get(0).unwrap().len() - 2 {
        return false;
    }
    let mut cross = String::from("");
    for offset in 0..=2 {
        cross.push(
            *arr.get(i + offset - 1)
                .unwrap()
                .get(j + offset - 1)
                .unwrap(),
        );
    }
    if cross != "MAS" && cross != "SAM" {
        return false;
    }
    let mut cross = String::from("");
    for offset in 0..=2 {
        cross.push(
            *arr.get(i + 1 - offset)
                .unwrap()
                .get(j + offset - 1)
                .unwrap(),
        );
    }
    if cross != "MAS" && cross != "SAM" {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
