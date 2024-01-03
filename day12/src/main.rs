use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let part_one_res = part1(&input);
    let part_two_res = part2(&input);
    println!("part 1: {part_one_res}");
    println!("part 2: {part_two_res}");
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn new(c: &char) -> Spring {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Unknown spring type"),
        }
    }
}

impl core::fmt::Debug for Spring {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        })
    }
}

fn part1(input: &String) -> u64 {
    let data: Vec<(Vec<_>, Vec<_>)> = input
        .lines()
        .map(|l| {
            let (springs, group_sizes) = l.split_once(" ").unwrap();
            let springs = springs
                .chars()
                .map(|c| Spring::new(&c))
                .collect::<Vec<_>>();
            let group_sizes = group_sizes
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (springs, group_sizes)
        })
        .collect();
    let mut cache = HashMap::new();
    return data
        .iter()
        .map(|d| find_arrangements(d.0.clone(), d.1.clone(), &mut cache))
        .sum();
}

fn part2(input: &String) -> u64 {
    let data: Vec<(Vec<_>, Vec<_>)> = input
        .lines()
        .map(|l| {
            let (springs, group_sizes) = l.split_once(" ").unwrap();
            let springs = (0..5)
                .map(|_| springs)
                .collect::<Vec<_>>()
                .join("?")
                .chars()
                .map(|c| Spring::new(&c))
                .collect::<Vec<_>>();
            let group_sizes = group_sizes
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            (springs, group_sizes)
        })
        .collect();
    let mut cache = HashMap::new();
    return data
        .iter()
        .map(|d| find_arrangements(d.0.clone(), d.1.clone(), &mut cache))
        .sum();
}

fn find_arrangements(
    springs: Vec<Spring>,
    group_sizes: Vec<u32>,
    cache: &mut HashMap<(Vec<Spring>, Vec<u32>), u64>
) -> u64 {
    // prioritize cached results
    if let Some(res) = cache.get(&(springs.clone(), group_sizes.clone())) {
        return *res;
    }
    // edge cases
    if group_sizes.len() == 0 {
        // no groups and no springs is still 1 "arrangement"
        if springs.len() == 0 {
            return 1;
        }
        // no groups but all operational/unknown springs is 1 "arrangement"
        if springs.iter().all(|s| (*s == Spring::Operational || *s == Spring::Unknown)) {
            return 1;
        }
        // no groups means no valid placements
        return 0;
    }
    // no springs means no arrangements
    if springs.len() == 0 {
        return 0;
    }
    match springs[0] {
        // continue arranging with remaining springs
        Spring::Operational => {
            return find_arrangements(springs[1..].to_vec(), group_sizes.clone(), cache);
        }
        // start trying to calculate arragments for damaged springs
        Spring::Damaged => {
            return calculate_damage_placements(springs, group_sizes, cache);
        }
        // try to treat this spring as both operational and damaged and record the results
        Spring::Unknown => {
            let mut acc = 0;
            acc += find_arrangements(springs[1..].to_vec(), group_sizes.clone(), cache);
            acc += calculate_damage_placements(springs, group_sizes, cache);
            return acc;
        }
    }
}

fn calculate_damage_placements(
    springs: Vec<Spring>,
    group_sizes: Vec<u32>,
    cache: &mut HashMap<(Vec<Spring>, Vec<u32>), u64>
) -> u64 {
    let group_size = group_sizes[0];
    // make sure we have enough valid springs to be assigned
    if
        (springs.len() as u32) < group_size ||
        springs[..group_size as usize].iter().any(|c| *c == Spring::Operational)
    {
        return 0;
    }
    let mut acc = 0;
    let next_springs = springs[group_size as usize..].to_vec();
    let next_groups = group_sizes[1..].to_vec();
    if next_springs.len() > 0 {
        if next_springs[0] == Spring::Damaged {
            return acc;
        }
        let res = find_arrangements(next_springs[1..].to_vec(), next_groups.clone(), cache);
        cache.insert((next_springs[1..].to_vec(), next_groups.clone()), res);
        acc += res;
    } else {
        let res = find_arrangements(next_springs.clone(), next_groups.clone(), cache);
        cache.insert((next_springs.clone(), next_groups.clone()), res);
        acc += res;
    }
    return acc;
}
