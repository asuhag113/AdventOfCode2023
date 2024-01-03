use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1_res = part1(&input);
    let p2_res = part2(&input);
    println!("p1: {p1_res}");
    println!("p2: {p2_res}");
}

fn part1(input: &String) -> u32 {
    let mut grid = create_grid(input);
    travel_and_energize(&mut grid, (0, 0), Direction::Right);
    return calculate_energized_tiles(&mut grid);
}

fn part2(input: &String) -> u32 {
    let mut grid = create_grid(input);
    let m = (grid.len() - 1) as i32;
    let n = (grid[0].len() - 1) as i32;
    // eh, there's probably a more concise/clever way to do this, but I'll opt to be explicit and lazy
    // send from corners
    travel_and_energize(&mut grid, (0, 0), Direction::Right);
    let tl1 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (0, 0), Direction::Down);
    let tl2 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (0, n), Direction::Left);
    let tr1 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (0, n), Direction::Down);
    let tr2 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (m, 0), Direction::Up);
    let bl1 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (m, 0), Direction::Right);
    let bl2 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (m, n), Direction::Left);
    let br1 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    travel_and_energize(&mut grid, (m, n), Direction::Up);
    let br2 = calculate_energized_tiles(&mut grid);
    grid = create_grid(input);
    let corner_sums = [tl1, tl2, tr1, tr2, bl1, bl2, br1, br2];
    let corner_max = corner_sums.iter().max().unwrap();

    // send from left
    let left_max = (1..grid.len() - 1)
        .map(|i| {
            travel_and_energize(&mut grid, (i as i32, 0), Direction::Right);
            let res = calculate_energized_tiles(&mut grid);
            grid = create_grid(input);
            return res;
        })
        .max()
        .unwrap();
    // send from top
    let top_max = (1..grid[0].len() - 1)
        .map(|j| {
            travel_and_energize(&mut grid, (0, j as i32), Direction::Down);
            let res = calculate_energized_tiles(&mut grid);
            grid = create_grid(input);
            return res;
        })
        .max()
        .unwrap();
    // send from right
    let right_max = (1..grid.len() - 1)
        .map(|i| {
            travel_and_energize(&mut grid, (i as i32, n), Direction::Left);
            let res = calculate_energized_tiles(&mut grid);
            grid = create_grid(input);
            return res;
        })
        .max()
        .unwrap();
    // send from bottom
    let bottom_max = (1..grid[0].len() - 1)
        .map(|j| {
            travel_and_energize(&mut grid, (m, j as i32), Direction::Up);
            let res = calculate_energized_tiles(&mut grid);
            grid = create_grid(input);
            return res;
        })
        .max()
        .unwrap();

    return *[*corner_max, left_max, top_max, right_max, bottom_max].iter().max().unwrap();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

struct Tile {
    identifier: char,
    visits: HashSet<Direction>,
}

impl Tile {
    fn new(c: &char) -> Tile {
        Tile { identifier: *c, visits: HashSet::new() }
    }
}

fn create_grid(input: &String) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l|
            l
                .chars()
                .map(|c| Tile::new(&c))
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

fn in_bounds(m: usize, n: usize, i: i32, j: i32) -> bool {
    i >= 0 && i < (m as i32) && j >= 0 && j < (n as i32)
}

fn travel_and_energize(grid: &mut Vec<Vec<Tile>>, cur_pos: (i32, i32), dir: Direction) {
    // end laser if out of bounds
    if !in_bounds(grid.len(), grid[0].len(), cur_pos.0, cur_pos.1) {
        return;
    }
    // prevent infinite cycles
    if grid[cur_pos.0 as usize][cur_pos.1 as usize].visits.contains(&dir) {
        return;
    }
    // track visit progress
    grid[cur_pos.0 as usize][cur_pos.1 as usize].visits.insert(dir);
    // vertical split
    if
        grid[cur_pos.0 as usize][cur_pos.1 as usize].identifier == '|' &&
        (dir == Direction::Left || dir == Direction::Right)
    {
        travel_and_energize(grid, (cur_pos.0 - 1, cur_pos.1), Direction::Up);
        travel_and_energize(grid, (cur_pos.0 + 1, cur_pos.1), Direction::Down);
        return;
    }
    // horizontal split
    if
        grid[cur_pos.0 as usize][cur_pos.1 as usize].identifier == '-' &&
        (dir == Direction::Up || dir == Direction::Down)
    {
        travel_and_energize(grid, (cur_pos.0, cur_pos.1 - 1), Direction::Left);
        travel_and_energize(grid, (cur_pos.0, cur_pos.1 + 1), Direction::Right);
        return;
    }
    // bend laser or travel as normal
    let mut new_dir = dir;
    if grid[cur_pos.0 as usize][cur_pos.1 as usize].identifier == '/' {
        match dir {
            Direction::Left => {
                new_dir = Direction::Down;
            }
            Direction::Up => {
                new_dir = Direction::Right;
            }
            Direction::Right => {
                new_dir = Direction::Up;
            }
            Direction::Down => {
                new_dir = Direction::Left;
            }
        }
    }
    if grid[cur_pos.0 as usize][cur_pos.1 as usize].identifier == '\\' {
        match dir {
            Direction::Left => {
                new_dir = Direction::Up;
            }
            Direction::Up => {
                new_dir = Direction::Left;
            }
            Direction::Right => {
                new_dir = Direction::Down;
            }
            Direction::Down => {
                new_dir = Direction::Right;
            }
        }
    }
    match new_dir {
        Direction::Left => {
            travel_and_energize(grid, (cur_pos.0, cur_pos.1 - 1), new_dir);
        }
        Direction::Up => {
            travel_and_energize(grid, (cur_pos.0 - 1, cur_pos.1), new_dir);
        }
        Direction::Right => {
            travel_and_energize(grid, (cur_pos.0, cur_pos.1 + 1), new_dir);
        }
        Direction::Down => {
            travel_and_energize(grid, (cur_pos.0 + 1, cur_pos.1), new_dir);
        }
    }
}

fn calculate_energized_tiles(grid: &mut Vec<Vec<Tile>>) -> u32 {
    grid.iter()
        .map(|r|
            r
                .iter()
                .map(|c| if c.visits.len() > 0 { 1 } else { 0 })
                .sum::<u32>()
        )
        .sum::<u32>()
}
