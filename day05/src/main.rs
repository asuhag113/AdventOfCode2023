use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");

    let part_one_res = part_one(&input);
    let part_two_res = part_two(&input);

    println!("part 1: {part_one_res}");
    println!("part 2: {part_two_res}");
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    src_start: u64,
    dst_start: u64,
    range: u64,
}

impl Mapping {
    fn from_str(s: &str) -> Mapping {
        let data = s.split(" ").collect::<Vec<_>>();
        let dst_start = data.get(0).unwrap().parse::<u64>().unwrap();
        let src_start = data.get(1).unwrap().parse::<u64>().unwrap();
        let range = data.get(2).unwrap().parse::<u64>().unwrap();
        return Mapping { src_start, dst_start, range };
    }
    fn contains_src(&self, n: &u64) -> bool {
        return (self.src_start..self.src_start + self.range).contains(n);
    }
    fn contains_dst(&self, n: &u64) -> bool {
        return (self.dst_start..self.dst_start + self.range).contains(n);
    }
}

fn part_one(input: &String) -> u64 {
    let (seeds, rest) = input.split_once("\n\n").unwrap();

    let seeds: Vec<_> = seeds
        .strip_prefix("seeds: ")
        .and_then(|s|
            s
                .split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<_, _>>()
                .ok()
        )
        .expect("parsed seeds");

    let maps = parse_maps(rest);

    let res = seed_to_location(seeds, maps);
    return res;
}

fn part_two(input: &String) -> u64 {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds_and_ranges: Vec<_> = seeds
        .strip_prefix("seeds: ")
        .and_then(|s|
            s
                .split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<_, _>>()
                .ok()
        )
        .expect("parsed seeds");
    let seeds = seeds_and_ranges
        .chunks(2)
        .map(|c| Range { start: c[0], end: c[0] + c[1] })
        .collect::<Vec<_>>();
    let maps = parse_maps(rest);
    // let res = location_to_seed(seeds, maps);
    let res = seed_range_to_location(seeds, maps);
    return res;
}

fn parse_maps(input: &str) -> Vec<Vec<Mapping>> {
    input
        .split("\n\n")
        .map(|s| {
            let (_, conversion_data) = s.split_once(":").unwrap();
            return conversion_data
                .trim()
                .lines()
                .map(|l| { Mapping::from_str(l.trim()) })
                .collect::<Vec<_>>();
        })
        .collect::<Vec<Vec<Mapping>>>()
}

fn seed_to_location(seeds: Vec<u64>, maps: Vec<Vec<Mapping>>) -> u64 {
    seeds
        .iter()
        .map(|s| {
            let mut v = s.clone();
            maps.iter().for_each(|mappings| {
                for m in mappings.iter() {
                    if m.contains_src(&v) {
                        let diff = v - m.src_start;
                        let dst_key = m.dst_start + diff;
                        if m.contains_dst(&dst_key) {
                            v = dst_key;
                            break;
                        }
                    }
                }
            });
            v
        })
        .min()
        .unwrap()
}

// still slow!
// fn location_to_seed(seeds: Vec<Range<u64>>, maps: Vec<Vec<Mapping>>) -> u64 {
//     let max_location = maps
//         .get(maps.len() - 1)
//         .unwrap()
//         .iter()
//         .map(|m| m.dst_start + m.range)
//         .max()
//         .unwrap();
//     for i in 0..=max_location {
//         let mut v = i;
//         maps.iter()
//             .rev()
//             .for_each(|mappings| {
//                 for m in mappings.iter() {
//                     if m.contains_dst(&v) {
//                         let diff = v - m.dst_start;
//                         let src_key = m.src_start + diff;
//                         if m.contains_src(&src_key) {
//                             v = src_key;
//                             break;
//                         }
//                     }
//                 }
//             });
//         if seeds.iter().any(|r| { r.contains(&v) }) {
//             return i;
//         }
//     }

//     return std::u64::MAX;
// }

trait MergeAndPush {
    fn merge_and_push(&mut self, range: Range<u64>);
}

impl MergeAndPush for Vec<Range<u64>> {
    // merge intervals. here, we care if two intervals overlap at exactly one point because we want to combine them into a single larger interval
    // ss           se
    // [------------)
    //              rs         re
    //              [----------)
    // s                       e
    // [-----------------------)
    fn merge_and_push(&mut self, range: Range<u64>) {
        let mut has_overlap = false;
        for r in self.iter_mut() {
            let overlap_start = r.start.max(range.start);
            let overlap_end = r.end.min(range.end);
            let s = r.start.min(range.start);
            let e = r.end.max(range.end);
            if overlap_start <= overlap_end {
                has_overlap = true;
                r.start = s;
                r.end = e;
                break;
            }
        }
        if !has_overlap {
            self.push(range);
        }
    }
}

#[test]
fn test_merge_and_push() {
    let mut vec: Vec<Range<u64>> = vec![
        Range { start: 1, end: 3 },
        Range { start: 6, end: 7 },
        Range { start: 8, end: 10 }
    ];
    let range: Range<u64> = Range { start: 2, end: 5 };
    vec.merge_and_push(range);
    assert_eq!(
        vec,
        vec![Range { start: 1, end: 5 }, Range { start: 6, end: 7 }, Range { start: 8, end: 10 }]
    )
}

fn seed_range_to_location(seeds: Vec<Range<u64>>, maps: Vec<Vec<Mapping>>) -> u64 {
    let mut ranges = seeds.clone();
    // for each transform map
    maps.iter().for_each(|mappings| {
        let mut transformed_ranges: Vec<Range<u64>> = vec![];
        // pass every range through and transform
        while ranges.len() > 0 {
            let range = ranges.pop().unwrap();
            let mut has_overlap = false;
            // check for overlaps against all ranges in map
            for m in mappings.iter() {
                // rs                    re
                // [--------------------)
                //            [----------------)
                //            ms               me
                let overlap_start = range.start.max(m.src_start);
                let overlap_end = range.end.min(m.src_start + m.range);
                // our range end is exclusive, so a valid overlap that can be transformed only occurs if start < end
                if overlap_start < overlap_end {
                    has_overlap = true;
                    // calculate transform
                    //         rs            re
                    //         [-------------)
                    // ms            me
                    // [------------)
                    //        os     oe
                    //         [----)
                    //   os-ms
                    // |-------|
                    //      oe-ms
                    // |------------|
                    //                leftover
                    // .            |---------)
                    let transformed_start = m.dst_start + (overlap_start - m.src_start);
                    let transformed_end = m.dst_start + (overlap_end - m.src_start);
                    transformed_ranges.merge_and_push(Range {
                        start: transformed_start,
                        end: transformed_end,
                    });
                    // re-evaluate leftover range segments
                    if overlap_start > range.start {
                        ranges.push(Range { start: range.start, end: overlap_start });
                    }
                    if range.end > overlap_end {
                        ranges.push(Range { start: overlap_end, end: range.end });
                    }
                    break;
                }
            }
            // if no overlap, pass straight through
            if !has_overlap {
                transformed_ranges.merge_and_push(Range { start: range.start, end: range.end });
            }
        }
        // update ranges with new values and pass to next transform map
        ranges = transformed_ranges;
    });

    ranges.sort_by(|a, b| { a.start.cmp(&b.start) });
    return ranges.first().unwrap().start;
}
