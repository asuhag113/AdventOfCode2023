use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1_res = part1(&input);
    let p2_res = part2(&input);
    println!("{p1_res}");
    println!("{p2_res}");
}

fn part1(input: &String) -> usize {
    let mut rock_grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    tilt_north(&mut rock_grid);
    return calculate_load(&rock_grid);
}

fn part2(input: &String) -> usize {
    let mut rock_grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // store previously seen states and the first index they were seen, we use a map here for faster lookups
    let mut grid_states = HashMap::from([(rock_grid.clone(), 0)]);
    let mut loop_end = 0;
    for i in 0..1000000000 {
        rock_grid = spin_cycle(&mut rock_grid);
        // break once we've seen a repeat state
        if grid_states.contains_key(&rock_grid) {
            loop_end = i + 1;
            break;
        }
        grid_states.insert(rock_grid.clone(), i + 1);
    }
    // find the first index at which this state was seen
    let loop_start = grid_states.get(&rock_grid).unwrap();
    let loop_length = loop_end - loop_start;

    // 0 1 2 3 4 5 6 7 8 9 10         1000000000
    // O-O-O-O-O-O-O-O-O-O-O-.....O-O-O
    //       |_____________|
    //      ls            le      %res
    // here, we "iterate" through the remaining spin cycles (1000000000 - loop_start) by chunks of loop_length as much as we can
    // the result of this is the number of remaining cycles that need to be performed which we just add to the starting point
    let final_grid_index = loop_start + ((1000000000 - loop_start) % loop_length);
    let final_grid = grid_states
        .iter()
        .find(|(_, v)| **v == final_grid_index)
        .unwrap().0;
    return calculate_load(&final_grid);
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)|
            row.iter().map(move |cell| if *cell == 'O' { grid.len() - i } else { 0 })
        )
        .sum()
}

fn tilt_north(rock_grid: &mut Vec<Vec<char>>) {
    let mut cube_to_round: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    // store positions of round and cube rocks
    for j in 0..rock_grid[0].len() {
        let mut round_rocks = Vec::new();
        for i in (0..rock_grid.len()).rev() {
            // insert cube rock, reset the round rocks as the round rocks in the next iteration will pile against a different cube rock
            if rock_grid[i][j] == '#' {
                cube_to_round.insert((i as i32, j as i32), round_rocks.clone());
                round_rocks.clear();
                continue;
            }
            // store round rocks for current cube rock
            if rock_grid[i][j] == 'O' {
                round_rocks.push((i as i32, j as i32));
            }
            // if we reach the border and there is no cube rock, assume there's a cube rock at the next tile
            if i == 0 {
                cube_to_round.insert((-1, j as i32), round_rocks.clone());
                round_rocks.clear();
            }
        }
    }
    // tilt north and update grid, we enumerate backwards to "score" the round rocks in order from closest to furthest with respect to the cube rock
    cube_to_round.iter().for_each(|(k, v)|
        v
            .iter()
            .enumerate()
            .rev()
            .for_each(|(i, n)| {
                // reset the current position in case the rock moves
                rock_grid[n.0 as usize][n.1 as usize] = '.';
                // rocks that are pushed to the grid boundary
                if k.0 == -1 {
                    rock_grid[v.len() - i - 1][n.1 as usize] = 'O';
                } else {
                    // rocks that are pushed to a cube rock
                    rock_grid[(k.0 as usize) + v.len() - i][n.1 as usize] = 'O';
                }
            })
    );
}

fn spin_cycle(rock_grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = rock_grid.clone();
    for _ in 0..4 {
        tilt_north(&mut grid);
        grid = rotate_grid(&grid);
    }
    return grid;
}

// rotate +90, this could probably be done in place, but it's easier just to create a new grid
fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            rotated_grid[i][grid[0].len() - 1 - j] = grid[j][i];
        }
    }
    return rotated_grid;
}
