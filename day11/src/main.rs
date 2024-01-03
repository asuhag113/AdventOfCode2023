use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut enlarged_rows = HashSet::new();
    let mut enlarged_cols = HashSet::new();
    let mut galaxies = Vec::new();
    let universe = input
        .lines()
        .map(|l| { l.chars().collect::<Vec<_>>() })
        .collect::<Vec<Vec<_>>>();
    // track which rows and cols should be enlarged
    for (i, r) in universe.iter().enumerate() {
        if r.iter().all(|c| *c == '.') {
            enlarged_rows.insert(i);
        }
        let c = get_column(&universe, i);
        if c.iter().all(|c| *c == '.') {
            enlarged_cols.insert(i);
        }
    }
    // track positions of galaxies
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    let p1_res = get_distance_sum(&galaxies, &enlarged_rows, &enlarged_cols, 2);
    let p2_res = get_distance_sum(&galaxies, &enlarged_rows, &enlarged_cols, 100000);
    println!("p1: {p1_res}");
    println!("p2: {p2_res}");
}

fn get_column(matrix: &Vec<Vec<char>>, column_index: usize) -> Vec<char> {
    matrix
        .iter()
        .map(|row| row[column_index])
        .collect()
}

fn get_distance_sum(
    galaxies: &Vec<(usize, usize)>,
    enlarged_rows: &HashSet<usize>,
    enlarged_cols: &HashSet<usize>,
    scale: u64
) -> u64 {
    let mut distances = vec![];
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (ai, aj) = (galaxies[i].0, galaxies[i].1);
            let (bi, bj) = (galaxies[j].0, galaxies[j].1);
            let row_range: HashSet<usize> = HashSet::from_iter(
                if ai < bi {
                    ai..bi
                } else {
                    bi..ai
                }
            );
            let col_range: HashSet<usize> = HashSet::from_iter(
                if aj < bj {
                    aj..bj
                } else {
                    bj..aj
                }
            );
            // these are all of the rows/cols that need to be scaled up
            let row_overlap = row_range.intersection(enlarged_rows).collect::<HashSet<_>>();
            let col_overlap = col_range.intersection(enlarged_cols).collect::<HashSet<_>>();
            let distance =
                // scale up the overlapping rows/cols
                (row_overlap.len() as u64) * scale +
                (col_overlap.len() as u64) * scale +
                // add to the normal rows/cols
                ((row_range.len() as u64) - (row_overlap.len() as u64)) +
                ((col_range.len() as u64) - (col_overlap.len() as u64));
            distances.push(distance);
        }
    }
    return distances.iter().sum();
}
