pub fn process(_input: &str) -> miette::Result<String> {
    let mut arr: Vec<Vec<char>> = Vec::new();
    for line in _input.lines() {
        arr.push(line.chars().collect());
    }
    //dbg!(arr);
    let copied = arr.clone();
    let mut res = 0;
    for (i, row) in arr.iter_mut().enumerate() {
        for (j, ch) in row.iter_mut().enumerate() {
            if *ch == 'X' {
                res += check_down(&copied, i, j) as u32;
                res += check_right(&copied, i, j) as u32;
                res += check_up(&copied, i, j) as u32;
                res += check_left(&copied, i, j) as u32;
                res += check_up_left(&copied, i, j) as u32;
                res += check_up_right(&copied, i, j) as u32;
                res += check_down_right(&copied, i, j) as u32;
                res += check_down_left(&copied, i, j) as u32;
            }
        }
    }
    Ok(res.to_string())
}

fn check_down(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i > arr.len() - 4 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch != arr.get(i + offset + 1).unwrap().get(j).unwrap() {
            return false;
        }
    }
    return true;
}

fn check_right(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > arr.get(0).unwrap().len() - 4 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch != arr.get(i).unwrap().get(j + offset + 1).unwrap() {
            return false;
        }
    }
    return true;
}

fn check_up(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch != arr.get(i - offset - 1).unwrap().get(j).unwrap() {
            return false;
        }
    }
    return true;
}

fn check_left(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch != arr.get(i).unwrap().get(j - offset - 1).unwrap() {
            return false;
        }
    }
    return true;
}

fn check_up_left(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 || i < 3 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch
            != arr
                .get(i - offset - 1)
                .unwrap()
                .get(j - offset - 1)
                .unwrap()
        {
            return false;
        }
    }
    return true;
}

fn check_up_right(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > arr.get(0).unwrap().len() - 4 || i < 3 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch
            != arr
                .get(i - offset - 1)
                .unwrap()
                .get(j + offset + 1)
                .unwrap()
        {
            return false;
        }
    }
    return true;
}

fn check_down_right(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j > arr.get(0).unwrap().len() - 4 || i > arr.len() - 4 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch
            != arr
                .get(i + offset + 1)
                .unwrap()
                .get(j + offset + 1)
                .unwrap()
        {
            return false;
        }
    }
    return true;
}

fn check_down_left(arr: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 || i > arr.len() - 4 {
        return false;
    }
    for (offset, ch) in ['M', 'A', 'S'].iter().enumerate() {
        if ch
            != arr
                .get(i + offset + 1)
                .unwrap()
                .get(j - offset - 1)
                .unwrap()
        {
            return false;
        }
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
