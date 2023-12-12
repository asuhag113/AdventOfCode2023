fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let sequences = input
        .lines()
        .map(|l| sequence_from_str(l))
        .collect::<Vec<_>>();
    let part_one_res = sequences
        .iter()
        .map(|s| extrapolate_forward(s.clone()))
        .sum::<i32>();
    println!("{part_one_res}");
    let part_two_res = sequences
        .iter()
        .map(|s| extrapolate_backward(s.clone()))
        .sum::<i32>();
    println!("{part_two_res}");
}

fn sequence_from_str(str: &str) -> Vec<i32> {
    str.split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn get_deltas(sequence: Vec<i32>) -> Vec<Vec<i32>> {
    let mut deltas = vec![sequence];
    loop {
        let seq = deltas.last().unwrap();
        let numbers = (1..seq.len()).map(|i| seq[i] - seq[i - 1]).collect::<Vec<_>>();
        let finished = numbers.iter().all(|n| *n == 0);
        deltas.push(numbers);
        if finished {
            break;
        }
    }
    return deltas;
}

fn extrapolate_forward(sequence: Vec<i32>) -> i32 {
    let deltas = get_deltas(sequence);
    let mut extrapolations: Vec<i32> = vec![];
    for i in (0..deltas.len() - 1).rev() {
        let prev_last = *extrapolations.last().unwrap_or_else(|| &0);
        let curr_last = *deltas[i].last().unwrap();
        extrapolations.push(curr_last + prev_last);
    }
    return *extrapolations.last().unwrap();
}

fn extrapolate_backward(sequence: Vec<i32>) -> i32 {
    let deltas = get_deltas(sequence);
    let mut extrapolations: Vec<i32> = vec![];
    for i in (0..deltas.len() - 1).rev() {
        let prev_last = *extrapolations.first().unwrap_or_else(|| &0);
        let curr_last = *deltas[i].first().unwrap();
        extrapolations.insert(0, curr_last - prev_last);
    }
    return *extrapolations.first().unwrap();
}
