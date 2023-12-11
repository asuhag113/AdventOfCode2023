use std::{ collections::HashMap, fs, cmp::Ordering };

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut hands = input
        .lines()
        .map(|l| Hand::from_str(l))
        .collect::<Vec<_>>();
    hands.iter_mut().for_each(|h| h.evaluate_rank());
    hands.sort_by(|a, b| {
        if a.rank == b.rank {
            return Hand::compare_cards(&a.cards, &b.cards);
        }
        return a.rank.cmp(&b.rank);
    });
    let score = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * ((i as u64) + (1 as u64)))
        .sum::<u64>();
    println!("{score}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    // J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'J' => Card::J,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            // 'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Unknown card type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    cards: Vec<Card>,
    map: HashMap<char, u64>,
    rank: Option<Rank>,
}

impl Hand {
    fn from_str(str: &str) -> Hand {
        let (cards_str, bid_str) = str.split_once(" ").unwrap();

        let bid = bid_str.parse::<u64>().unwrap();

        let cards = cards_str
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Vec<_>>();

        let mut map = HashMap::new();

        cards_str.chars().for_each(|c| {
            map.entry(c)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        });

        // part 2: add joker count to most common non-joker card and remove joker count to avoid duplicate counts when identifying rank
        if map.get(&'J').is_some() {
            let joker_count = *map.get(&'J').unwrap();
            let highest_non_joker = map
                .iter()
                .filter(|(k, _)| { **k != 'J' })
                .max_by_key(|(_, v)| { **v })
                .map(|(key, _)| *key);
            if highest_non_joker.is_some() {
                map.entry(highest_non_joker.unwrap()).and_modify(|v| {
                    *v += joker_count;
                });
            }
            map.remove(&'J');
        }

        Hand { bid, cards, map, rank: None }
    }
    fn evaluate_rank(&mut self) {
        // Five of a kind
        if
            self.map
                .values()
                .filter(|v| **v == 5)
                .collect::<Vec<_>>()
                .len() >= 1 ||
            // edge case when hand is JJJJJ, this could likely be made more straightforward
            self.map.values().len() == 0
        {
            self.rank = Some(Rank::FiveOfAKind);
            return;
        }
        // 4 of a kind
        if
            self.map
                .values()
                .filter(|v| **v == 4)
                .collect::<Vec<_>>()
                .len() >= 1
        {
            self.rank = Some(Rank::FourOfAKind);
            return;
        }
        // full house or three of a kind
        if
            self.map
                .values()
                .filter(|v| **v == 3)
                .collect::<Vec<_>>()
                .len() >= 1
        {
            if
                self.map
                    .values()
                    .filter(|v| **v == 2)
                    .collect::<Vec<_>>()
                    .len() >= 1
            {
                self.rank = Some(Rank::FullHouse);
                return;
            }
            self.rank = Some(Rank::ThreeOfAKind);
            return;
        }
        // two pair
        if
            self.map
                .values()
                .filter(|v| **v == 2)
                .collect::<Vec<_>>()
                .len() >= 2
        {
            self.rank = Some(Rank::TwoPair);
            return;
        }
        // one pair
        if
            self.map
                .values()
                .filter(|v| **v == 2)
                .collect::<Vec<_>>()
                .len() >= 1
        {
            self.rank = Some(Rank::OnePair);
            return;
        }
        // // high card
        self.rank = Some(Rank::HighCard);
        return;
    }
    fn compare_cards(a: &Vec<Card>, b: &Vec<Card>) -> Ordering {
        for i in 0..a.len() {
            if a[i] != b[i] {
                return a[i].cmp(&b[i]);
            }
        }
        return Ordering::Equal;
    }
}
