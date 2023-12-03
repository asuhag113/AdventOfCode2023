use std::char;

fn is_symbol(char: char) -> bool {
    return !char.is_ascii_digit() && char != '.';
}

fn is_in_bounds(i: isize, j: isize, m: usize, n: usize) -> bool {
    return i >= 0 && i < m.try_into().unwrap() && j >= 0 && j < n.try_into().unwrap();
}

fn find_and_invalidate_number(
    engine_grid: &mut Vec<Vec<char>>,
    i: isize,
    j: isize,
    m: usize,
    n: usize
) -> u32 {
    if !is_in_bounds(i.try_into().unwrap(), j.try_into().unwrap(), m, n) {
        return 0;
    }
    let i_as_usize: usize = i.try_into().unwrap();
    let j_as_usize: usize = j.try_into().unwrap();
    let mut left: isize = (j_as_usize - 1).try_into().unwrap();
    let mut right = j_as_usize + 1;
    let mut char_vec: Vec<char> = vec![];
    if engine_grid[i_as_usize][j_as_usize].is_ascii_digit() {
        char_vec.push(engine_grid[i_as_usize][j_as_usize]);
        engine_grid[i_as_usize][j_as_usize] = '.';
        while left >= 0 && engine_grid[i_as_usize][left as usize].is_ascii_digit() {
            char_vec.insert(0, engine_grid[i_as_usize][left as usize]);
            engine_grid[i_as_usize][left as usize] = '.';
            left -= 1;
        }
        while right < n && engine_grid[i_as_usize][right].is_ascii_digit() {
            char_vec.push(engine_grid[i_as_usize][right]);
            engine_grid[i_as_usize][right] = '.';
            right += 1;
        }
    }
    if char_vec.len() > 0 {
        let num_str: String = char_vec.into_iter().collect();
        let num_as_u32 = num_str.parse::<u32>().expect("parsed number into u32");
        return num_as_u32;
    }
    return 0;
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let data: Vec<String> = input.lines().map(String::from).collect();
    let mut engine_grid: Vec<Vec<char>> = data
        .iter()
        .map(|l| l.chars().collect())
        .collect();
    let mut valid_numbers: Vec<u32> = vec![];
    let mut gear_ratios: Vec<u32> = vec![];
    let m = engine_grid.len();
    for i in 0..m {
        let n = engine_grid[i].len();
        for j in 0..n {
            let c = engine_grid[i][j];
            if is_symbol(c) {
                let top = find_and_invalidate_number(
                    &mut engine_grid,
                    (i - 1).try_into().unwrap(),
                    j.try_into().unwrap(),
                    m,
                    n
                );

                let bottom = find_and_invalidate_number(
                    &mut engine_grid,
                    (i + 1).try_into().unwrap(),
                    j.try_into().unwrap(),
                    m,
                    n
                );

                let left = find_and_invalidate_number(
                    &mut engine_grid,
                    i.try_into().unwrap(),
                    (j - 1).try_into().unwrap(),
                    m,
                    n
                );

                let right = find_and_invalidate_number(
                    &mut engine_grid,
                    i.try_into().unwrap(),
                    (j + 1).try_into().unwrap(),
                    m,
                    n
                );

                let top_left = find_and_invalidate_number(
                    &mut engine_grid,
                    (i - 1).try_into().unwrap(),
                    (j - 1).try_into().unwrap(),
                    m,
                    n
                );

                let bottom_left = find_and_invalidate_number(
                    &mut engine_grid,
                    (i + 1).try_into().unwrap(),
                    (j - 1).try_into().unwrap(),
                    m,
                    n
                );

                let top_right = find_and_invalidate_number(
                    &mut engine_grid,
                    (i - 1).try_into().unwrap(),
                    (j + 1).try_into().unwrap(),
                    m,
                    n
                );

                let bottom_right = find_and_invalidate_number(
                    &mut engine_grid,
                    (i + 1).try_into().unwrap(),
                    (j + 1).try_into().unwrap(),
                    m,
                    n
                );
                let non_zero_nums: Vec<u32> = vec![
                    top,
                    bottom,
                    left,
                    right,
                    top_left,
                    bottom_left,
                    top_right,
                    bottom_right
                ]
                    .into_iter()
                    .filter(|n| *n != 0)
                    .collect();

                if non_zero_nums.len() == 2 && c == '*' {
                    let gear_ratio = non_zero_nums.iter().fold(1, |acc, e| acc * e);
                    gear_ratios.push(gear_ratio);
                }
                valid_numbers.push(top);
                valid_numbers.push(bottom);
                valid_numbers.push(left);
                valid_numbers.push(right);
                valid_numbers.push(top_left);
                valid_numbers.push(bottom_left);
                valid_numbers.push(top_right);
                valid_numbers.push(bottom_right);
            }
        }
    }
    let valid_numbers_sum: u32 = valid_numbers.iter().sum();
    let gear_ratios_sum: u32 = gear_ratios.iter().sum();

    println!("part 1: {valid_numbers_sum:?}");
    println!("part 2: {gear_ratios_sum:?}");
}
