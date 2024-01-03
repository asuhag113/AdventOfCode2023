use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1_res = part1(&input);
    let p2_res = part2(&input);
    println!("{p1_res}");
    println!("{p2_res}");
}

fn part1(input: &String) -> u32 {
    input
        .split(",")
        .map(|step| hash(step))
        .sum::<u32>()
}

fn part2(input: &String) -> u32 {
    let mut boxes: HashMap<u32, Vec<String>> = HashMap::from_iter((0..256).map(|i| (i, vec![])));
    let steps = input
        .split(",")
        .map(|s| Step::new(s))
        .collect::<Vec<_>>();
    steps.iter().for_each(|s| {
        let box_num = hash(&s.label);
        let box_lenses = boxes.get_mut(&box_num).unwrap();
        if s.operation == Operation::Remove {
            if let Some(idx) = box_lenses.iter().position(|l| l.contains(&s.label)) {
                box_lenses.remove(idx);
            }
        } else {
            let new_lens = format!("{0} {1}", s.label, s.focal_length.unwrap());
            if let Some(idx) = box_lenses.iter().position(|l| l.contains(&s.label)) {
                box_lenses.remove(idx);
                box_lenses.insert(idx, new_lens);
            } else {
                box_lenses.push(new_lens)
            }
        }
    });
    return boxes
        .iter()
        .map(|(box_number, lenses)|
            lenses
                .iter()
                .enumerate()
                .map(|(slot_number, l)|
                    calculate_focusing_power(*box_number, slot_number as u32, l)
                )
                .sum::<u32>()
        )
        .sum::<_>();
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc: u32, x| {
        let v = (((acc + (x as u32)) * 17) as u32) % 256;
        return v;
    })
}

#[derive(PartialEq)]
enum Operation {
    Add,
    Remove,
}

struct Step {
    label: String,
    operation: Operation,
    focal_length: Option<u32>,
}

impl Step {
    fn new(s: &str) -> Step {
        let add_regex = Regex::new("([a-zA-Z]+)=([0-9])").unwrap();
        let remove_regex = Regex::new("([a-zA-Z]+)-").unwrap();
        if add_regex.is_match(s) {
            let captures = add_regex.captures(s).unwrap();
            let label = captures.get(1).unwrap().as_str().to_string();
            let focal_length = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            return Step { label, operation: Operation::Add, focal_length: Some(focal_length) };
        }
        let captures = remove_regex.captures(s).unwrap();
        let label = captures.get(1).unwrap().as_str().to_string();
        return Step { label, operation: Operation::Remove, focal_length: None };
    }
}

fn calculate_focusing_power(box_number: u32, slot_number: u32, lens: &String) -> u32 {
    let (_, focal_length) = lens.split_once(" ").unwrap();
    let focal_length = focal_length.parse::<u32>().unwrap();
    return (1 + box_number) * (1 + slot_number) * focal_length;
}
