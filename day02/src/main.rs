use regex::Regex;

fn part_one(data: &Vec<String>) {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;
    let res: u32 = data
        .iter()
        .map(|l| {
            let game: Vec<&str> = l.split(":").collect();
            let game_id_prefix: Vec<&str> = game
                .get(0)
                .expect("retrieved game id prefix")
                .split("Game ")
                .collect();
            let game_id: u32 = game_id_prefix
                .get(1)
                .expect("retrieved game id")
                .parse::<u32>()
                .expect("parsed game id into u32");
            let cube_sets: Vec<&str> = game
                .get(1)
                .expect("retrieved cube sets")
                .trim()
                .split(";")
                .collect();

            return if
                cube_sets.iter().any(|s| {
                    let cubes: Vec<&str> = s.split(",").collect();
                    return cubes.iter().any(|c| {
                        let cube: Vec<&str> = c.trim().split(" ").collect();
                        let amount = cube
                            .get(0)
                            .expect("retrieved cube amount")
                            .parse::<u32>()
                            .expect("parsed cube amount to u32");
                        let color: &str = cube.get(1).expect("retrieved cube color");
                        match color {
                            "red" => {
                                return amount > RED_MAX;
                            }
                            "green" => {
                                return amount > GREEN_MAX;
                            }
                            "blue" => {
                                return amount > BLUE_MAX;
                            }
                            _ => {
                                return false;
                            }
                        }
                    });
                })
            {
                0
            } else {
                game_id
            };
        })
        .sum();

    println!("{res}");
}

fn part_two(data: &Vec<String>) {
    let res: u32 = data
        .iter()
        .map(|l| {
            let game: Vec<&str> = l.split(":").collect();
            let cube_sets: Vec<&str> = game
                .get(1)
                .expect("retrieved cube sets")
                .trim()
                .split(";")
                .collect();
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;
            cube_sets.iter().for_each(|s| {
                let cubes: Vec<&str> = s.split(",").collect();
                cubes.iter().for_each(|c| {
                    let cube: Vec<&str> = c.trim().split(" ").collect();
                    let amount = cube
                        .get(0)
                        .expect("retrieved cube amount")
                        .parse::<u32>()
                        .expect("parsed cube amount to u32");
                    let color: &str = cube.get(1).expect("retrieved cube color");
                    match color {
                        "red" => {
                            red_max = u32::max(red_max, amount);
                        }
                        "green" => {
                            green_max = u32::max(green_max, amount);
                        }
                        "blue" => {
                            blue_max = u32::max(blue_max, amount);
                        }
                        _ => {
                            //noop
                        }
                    }
                });
            });
            return red_max * green_max * blue_max;
        })
        .sum();

    println!("{res}");
}

// just curious if this other idea is faster or slower
fn efficiency_test(data: &Vec<String>) {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;
    let regex = Regex::new(r"(\d+)\s(red|blue|green)").expect("created cube regex");
    let mut valid_game_ids: Vec<u32> = Vec::new();
    let mut powers: Vec<u32> = Vec::new();
    data.iter().for_each(|l| {
        let (game_prefix, game_str) = l.split_once(":").expect("split");
        let (_, id) = game_prefix.trim().split_once(" ").expect("split");
        let id_as_u32 = id.parse::<u32>().expect("u32");
        let mut is_valid = true;
        let mut red_local_max: Option<u32> = None;
        let mut green_local_max: Option<u32> = None;
        let mut blue_local_max: Option<u32> = None;
        let mut cubes: Vec<(u32, &str)> = regex
            .find_iter(game_str.trim())
            .map(|m| {
                let (amt, clr) = m.as_str().split_once(" ").expect("split to amt and clr");
                return (amt.parse::<u32>().expect("u32"), clr);
            })
            .collect();
        cubes.sort_by(|a, b| b.0.cmp(&a.0));
        cubes.iter().for_each(|c| {
            match c.1 {
                "red" => {
                    if c.0 > RED_MAX {
                        is_valid = false;
                    }
                    if red_local_max.is_none() {
                        red_local_max = Some(c.0);
                    }
                }
                "green" => {
                    if c.0 > GREEN_MAX {
                        is_valid = false;
                    }
                    if green_local_max.is_none() {
                        green_local_max = Some(c.0);
                    }
                }
                "blue" => {
                    if c.0 > BLUE_MAX {
                        is_valid = false;
                    }
                    if blue_local_max.is_none() {
                        blue_local_max = Some(c.0);
                    }
                }
                _ => {}
            }
        });
        if is_valid {
            valid_game_ids.push(id_as_u32);
        }
        powers.push(red_local_max.unwrap() * green_local_max.unwrap() * blue_local_max.unwrap())
    });

    let p1_res: u32 = valid_game_ids.iter().sum();
    let p2_res: u32 = powers.iter().sum();

    println!("part 1: {p1_res}");
    println!("part 2: {p2_res}");
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let data: Vec<String> = input.lines().map(String::from).collect();

    let v1_timer = std::time::Instant::now();
    part_one(&data);
    part_two(&data);
    println!("Elapsed time for v1: {:.2?}", v1_timer.elapsed());
    // debug build times (ms): ~ 1.7-1.9

    let v2_timer = std::time::Instant::now();
    efficiency_test(&data);
    println!("Elapsed time for v2: {:.2?}", v2_timer.elapsed());
    // debug build times (ms): ~ 4.5-8.0 (with and without sort)
}
