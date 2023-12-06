use std::collections::{ HashSet, HashMap };

fn count_total_cards(game_match_map: HashMap<u32, u32>) -> u32 {
    let mut card_count_map: HashMap<u32, u32> = (1..=game_match_map.len())
        .map(|n: usize| (n as u32, 1 as u32))
        .collect();
    for i in 1..=card_count_map.len() {
        let matches = game_match_map.get(&(i as u32)).unwrap();
        let start = i + 1;
        let end = i + (*matches as usize);
        let num_cards = card_count_map
            .get(&(i as u32))
            .unwrap()
            .clone();
        for j in start..=end {
            card_count_map.entry(j as u32).and_modify(|v| {
                *v += num_cards;
            });
        }
    }
    return card_count_map.values().into_iter().sum();
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let data: Vec<String> = input.lines().map(String::from).collect();
    let mut game_match_map: HashMap<u32, u32> = HashMap::new();
    let points: u32 = data
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let (_, numbers) = l.split_once(": ").unwrap();
            let (winning_numbers, card_numbers) = numbers.split_once(" | ").unwrap();

            let winning_number_set: HashSet<u32> = winning_numbers
                .split(" ")
                .map(|s| s.parse::<u32>())
                .filter_map(Result::ok)
                .collect();
            let card_number_set: HashSet<u32> = card_numbers
                .split(" ")
                .map(|s| s.parse::<u32>())
                .filter_map(Result::ok)
                .collect();

            let matches: HashSet<_> = winning_number_set.intersection(&card_number_set).collect();
            let match_num = matches.len();
            game_match_map.insert((i + 1) as u32, match_num as u32);
            return if match_num > 0 { (2 as u32).pow((match_num as u32) - 1) } else { 0 };
        })
        .sum();

    let total_cards = count_total_cards(game_match_map);

    println!("part 1: {points}");
    println!("part 2: {total_cards}");
}
