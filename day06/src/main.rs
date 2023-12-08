#[derive(Debug)]
struct Race {
    duration: u64,
    distance: u64,
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let part_one_res = part_one(&input);
    let part_two_res = part_two(&input);
    println!("part 1: {part_one_res}");
    println!("part 2: {part_two_res}");
}

// note, the ways to win are actually symmetrical so we could probably just check half of the values and double
fn calculate_wins(race: &Race) -> u64 {
    let mut wins = 0;
    for i in 0..race.duration {
        let hold_duration = i;
        let remaining_duration = race.duration - hold_duration;
        let travel_distance = remaining_duration * i;
        if travel_distance > race.distance {
            wins += 1;
        }
    }
    return wins;
}

fn part_one(input: &String) -> u64 {
    let (times, distances) = input.split_once("\n").unwrap();
    let times = times
        .strip_prefix("Time:")
        .and_then(|s| Some(s.trim()))
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = distances
        .strip_prefix("Distance:")
        .and_then(|s| Some(s.trim()))
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let races = (0..times.len())
        .map(|i| Race { duration: times[i], distance: distances[i] })
        .collect::<Vec<_>>();
    let win_product = races
        .iter()
        .map(|r| calculate_wins(r))
        .fold(1, |acc, x| acc * x);
    return win_product;
}

fn part_two(input: &String) -> u64 {
    let (times, distances) = input.split_once("\n").unwrap();
    let duration = times
        .strip_prefix("Time:")
        .and_then(|s| Some(s.trim()))
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .strip_prefix("Distance:")
        .and_then(|s| Some(s.trim()))
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .unwrap();

    let race = Race { duration, distance };
    let win_product = calculate_wins(&race);
    return win_product;
}
