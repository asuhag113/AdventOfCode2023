use regex::Regex;

fn reverse_str(str: &str) -> String {
    let reversed: String = str.chars().rev().collect();
    return reversed;
}

fn digit_string_to_u32(string: &str) -> u32 {
    match string {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}

fn part_one(data: &Vec<String>) {
    let regex = Regex::new(r"\d").expect("created regex");

    let calibration_sum: u32 = data
        .iter()
        .map(|l| {
            let reverse_l = reverse_str(l);
            let digit_one = digit_string_to_u32(regex.find(l).expect("found digit one").as_str());
            let digit_two = digit_string_to_u32(
                regex.find(&reverse_l).expect("found digit two").as_str()
            );
            return digit_one * 10 + digit_two;
        })
        .sum();

    println!("{calibration_sum}");
}

fn part_two(data: &Vec<String>) {
    let numeric_digit_regex = r"\d|";
    let forward_semantic_digit_regex = "one|two|three|four|five|six|seven|eight|nine";
    let reverse_semantic_digit_regex = reverse_str(forward_semantic_digit_regex);
    let forward_regex = Regex::new(
        &format!("{numeric_digit_regex}{forward_semantic_digit_regex}")
    ).expect("created forward regex");
    let reverse_regex = Regex::new(
        &format!("{numeric_digit_regex}{reverse_semantic_digit_regex}")
    ).expect("created reverse regex");

    let calibration_sum: u32 = data
        .iter()
        .map(|l| {
            let reverse_l: String = l.chars().rev().collect();
            let digit_one = digit_string_to_u32(
                forward_regex.find(l).expect("found digit one").as_str()
            );
            // re-reverse the reverse match to be able to match against the semantic digit strings
            let reverse_match: String = reverse_str(
                reverse_regex.find(&reverse_l).expect("found digit two").as_str()
            );
            let digit_two = digit_string_to_u32(&reverse_match);
            return digit_one * 10 + digit_two;
        })
        .sum();

    println!("{calibration_sum}");
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let data: Vec<String> = input.lines().map(String::from).collect();

    part_one(&data);
    part_two(&data);
}
