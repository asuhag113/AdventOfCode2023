fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let part_one_res = part1(&input);
    let part_two_res = part2(&input);
    println!("part 1: {part_one_res}");
    println!("part 2: {part_two_res}");
}

fn part1(input: &String) -> usize {
    let notes = input
        .split("\n\n")
        .map(|n|
            n
                .split("\n")
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let res = notes
        .iter()
        .map(|n| { (get_vertical_aos(n), get_horizontal_aos(n)) })
        .fold(0, |acc, x| {
            // rows left of vaos
            if x.0.is_some() {
                return acc + x.0.unwrap().0;
                // rows above haos
            } else if x.1.is_some() {
                return acc + 100 * x.1.unwrap().0;
            }
            return acc;
        });
    return res;
}

fn part2(input: &String) -> usize {
    let notes = input
        .split("\n\n")
        .map(|n|
            n
                .split("\n")
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let old_aos = notes
        .iter()
        .enumerate()
        .map(|(i, n)| { (i, get_vertical_aos(n), get_horizontal_aos(n)) })
        .collect::<Vec<_>>();
    let res = notes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            (i, try_find_vertical_aos(n, old_aos[i].1), try_find_horizontal_aos(n, old_aos[i].2))
        })
        // this could probably be more readable if I replaced tuples with structs with named properties
        .fold(0, |acc, x| {
            // if a new vaos exists
            if x.1.is_some() {
                // if old and new are same, we are guaranteed to have a new haos
                if old_aos[x.0].1.is_some_and(|a| a.1 == x.1.unwrap().0) {
                    return acc + 100 * x.2.unwrap().0;
                }
                // rows above vaos
                return acc + x.1.unwrap().0;
                // otherwise, return rows left of haos
            } else if x.2.is_some() {
                return acc + 100 * x.2.unwrap().0;
            }
            return acc;
        });
    return res;
}

fn get_row_str(note: &Vec<Vec<char>>, index: usize) -> String {
    note[index].iter().collect::<_>()
}

fn get_col_str(note: &Vec<Vec<char>>, index: usize) -> String {
    note.iter()
        .map(|n| n[index])
        .collect::<Vec<_>>()
        .iter()
        .collect::<_>()
}

fn get_vertical_aos(note: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut aos = None;
    // pick a point and expand while columns match
    for i in 0..note[0].len() - 1 {
        let mut l = i as isize;
        let mut r = i + 1;
        while l >= 0 && r < note[0].len() && get_col_str(note, l as usize) == get_col_str(note, r) {
            l -= 1;
            r += 1;
            if l < 0 || r >= note[0].len() {
                aos = Some((i + 1, i + 2));
            }
        }
    }
    return aos;
}

fn get_horizontal_aos(note: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut aos = None;
    for i in 0..note.len() - 1 {
        let mut l = i as isize;
        let mut r = i + 1;
        // pick a point and expand while rows match
        while l >= 0 && r < note.len() && get_row_str(note, l as usize) == get_row_str(note, r) {
            l -= 1;
            r += 1;
            if l < 0 || r >= note.len() {
                aos = Some((i + 1, i + 2));
            }
        }
    }
    return aos;
}

fn get_vertical_aos_with_swap(note: &Vec<Vec<char>>, sj: usize) -> Option<(usize, usize)> {
    let mut aos = None;
    for i in 0..note[0].len() - 1 {
        let mut l = i as isize;
        let mut r = i + 1;
        // pick a point and expand while columns match
        while l >= 0 && r < note[0].len() && get_col_str(note, l as usize) == get_col_str(note, r) {
            let contains_swap = (l as usize) <= sj && r >= sj;
            l -= 1;
            r += 1;
            // we need to make sure that the new aos contains the swapped point
            if (l < 0 || r >= note[0].len()) && contains_swap {
                aos = Some((i + 1, i + 2));
            }
        }
    }
    return aos;
}

fn get_horizontal_aos_with_swap(note: &Vec<Vec<char>>, si: usize) -> Option<(usize, usize)> {
    let mut aos = None;
    for i in 0..note.len() - 1 {
        let mut l = i as isize;
        let mut r = i + 1;
        // pick a point and expand while rows match
        while l >= 0 && r < note.len() && get_row_str(note, l as usize) == get_row_str(note, r) {
            let contains_swap = (l as usize) <= si && r >= si;
            l -= 1;
            r += 1;
            // we need to make sure that the new aos contains the swapped point
            if (l < 0 || r >= note.len()) && contains_swap {
                aos = Some((i + 1, i + 2));
            }
        }
    }
    return aos;
}

fn swap(note: &mut Vec<Vec<char>>, i: usize, j: usize) {
    match note[i][j] {
        '.' => {
            note[i].remove(j);
            note[i].insert(j, '#')
        }
        '#' => {
            note[i].remove(j);
            note[i].insert(j, '.')
        }
        _ => panic!("Unknown char in note"),
    }
}

fn try_find_horizontal_aos(
    note: &Vec<Vec<char>>,
    old_aos: Option<(usize, usize)>
) -> Option<(usize, usize)> {
    let mut mod_note = note.clone();
    // try to swap every point in the grid and check if a new aos exists
    for i in 0..mod_note.len() {
        for j in 0..mod_note[i].len() {
            swap(&mut mod_note, i, j);
            if let Some(aos) = get_horizontal_aos_with_swap(&mod_note, i) {
                // verify new aos is different from old
                if old_aos.is_none() || old_aos.is_some_and(|a| a.1 != aos.1) {
                    return Some(aos);
                }
            }
            // restore grid
            swap(&mut mod_note, i, j);
        }
    }
    return None;
}

// note, I think this function could be combined with the above function and modified to return both aos since they both
// swap all points in the grid
fn try_find_vertical_aos(
    note: &Vec<Vec<char>>,
    old_aos: Option<(usize, usize)>
) -> Option<(usize, usize)> {
    let mut mod_note = note.clone();
    // try to swap every point in the grid and check if a new aos exists
    for i in 0..mod_note.len() {
        for j in 0..mod_note[i].len() {
            swap(&mut mod_note, i, j);
            if let Some(aos) = get_vertical_aos_with_swap(&mod_note, j) {
                // verify that new aos is different from old
                if old_aos.is_none() || old_aos.is_some_and(|a| a.0 != aos.0) {
                    return Some(aos);
                }
            }
            // restore grid
            swap(&mut mod_note, i, j);
        }
    }
    return None;
}
