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

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("processed input file");
    let data: Vec<String> = input.lines().map(String::from).collect();

    part_one(&data);
    part_two(&data);
}
