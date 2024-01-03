fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut tiles = input
        .lines()
        .enumerate()
        .map(|(i, l)|
            l
                .chars()
                .enumerate()
                .map(|(j, c)| Tile::new(&c, i as i32, j as i32))
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            let tile = &tiles[i][j];
            if tile.variant == Variant::Start {
                let neighbors = get_neighbors(i as i32, j as i32, tile, &tiles);
                tiles[i][j].visited = true;
                replace_start(&mut tiles[i][j], &neighbors);
                find_loop(neighbors[0].coordinates.0, neighbors[0].coordinates.1, &mut tiles);
            }
        }
    }
    // part 1: find the length of the loop and then divide by 2 (accounting for even/odd)
    let distance = tiles
        .iter()
        .flat_map(|r|
            r
                .iter()
                .map(|c| c.distance)
                .max()
        )
        .max()
        .unwrap();
    let max_distance = if distance % 2 == 0 { distance / 2 } else { distance / 2 + 1 };
    println!("p1: {max_distance}");
    // part 2: this is a ray tracing application! cast a ray across each row in tiles. the ray should insersect an odd # of pipes for points inside the loop and an even # for points outside
    for i in 0..tiles.len() {
        ray_trace_and_mark(&mut tiles[i]);
    }
    let num_within = tiles
        .iter()
        .flat_map(|r| r.iter().filter(|t| t.within))
        .count();
    println!("p2: {num_within}");
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    visited: bool,
    distance: u32,
    variant: Variant,
    coordinates: (i32, i32),
    within: bool,
}

impl Tile {
    fn new(c: &char, x: i32, y: i32) -> Tile {
        Tile {
            visited: false,
            distance: 0,
            variant: Variant::from_char(c),
            coordinates: (x, y),
            within: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Variant {
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

impl Variant {
    fn from_char(c: &char) -> Variant {
        match c {
            'S' => Variant::Start,
            '|' => Variant::NorthSouth,
            '-' => Variant::EastWest,
            'L' => Variant::NorthEast,
            'J' => Variant::NorthWest,
            'F' => Variant::SouthEast,
            '7' => Variant::SouthWest,
            '.' => Variant::Ground,
            _ => panic!("Unknown variant"),
        }
    }
    // fn to_char(&self) -> char {
    //     match self {
    //         Variant::Start => 'S',
    //         Variant::NorthSouth => '|',
    //         Variant::EastWest => '-',
    //         Variant::NorthEast => 'L',
    //         Variant::NorthWest => 'J',
    //         Variant::SouthEast => 'F',
    //         Variant::SouthWest => '7',
    //         Variant::Ground => '.',
    //     }
    // }
    fn is_north_compatible(&self, other: &Variant) -> bool {
        match self {
            Variant::Start =>
                [Variant::NorthSouth, Variant::SouthEast, Variant::SouthWest].contains(other),
            Variant::NorthSouth =>
                [Variant::NorthSouth, Variant::SouthEast, Variant::SouthWest].contains(other),
            Variant::EastWest => false,
            Variant::NorthEast =>
                [Variant::NorthSouth, Variant::SouthEast, Variant::SouthWest].contains(other),
            Variant::NorthWest =>
                [Variant::NorthSouth, Variant::SouthEast, Variant::SouthWest].contains(other),
            Variant::SouthEast => false,
            Variant::SouthWest => false,
            Variant::Ground => false,
        }
    }
    fn is_east_compatible(&self, other: &Variant) -> bool {
        match self {
            Variant::Start =>
                [Variant::EastWest, Variant::NorthWest, Variant::SouthWest].contains(other),
            Variant::NorthSouth => false,
            Variant::EastWest =>
                [Variant::EastWest, Variant::NorthWest, Variant::SouthWest].contains(other),
            Variant::NorthEast =>
                [Variant::EastWest, Variant::NorthWest, Variant::SouthWest].contains(other),
            Variant::NorthWest => false,
            Variant::SouthEast =>
                [Variant::EastWest, Variant::NorthWest, Variant::SouthWest].contains(other),
            Variant::SouthWest => false,
            Variant::Ground => false,
        }
    }
    fn is_south_compatible(&self, other: &Variant) -> bool {
        match self {
            Variant::Start =>
                [Variant::NorthSouth, Variant::NorthEast, Variant::NorthWest].contains(other),
            Variant::NorthSouth =>
                [Variant::NorthSouth, Variant::NorthEast, Variant::NorthWest].contains(other),
            Variant::EastWest => false,
            Variant::NorthEast => false,
            Variant::NorthWest => false,
            Variant::SouthEast =>
                [Variant::NorthSouth, Variant::NorthEast, Variant::NorthWest].contains(other),
            Variant::SouthWest =>
                [Variant::NorthSouth, Variant::NorthEast, Variant::NorthWest].contains(other),
            Variant::Ground => false,
        }
    }
    fn is_west_compatible(&self, other: &Variant) -> bool {
        match self {
            Variant::Start =>
                [Variant::EastWest, Variant::NorthEast, Variant::SouthEast].contains(other),
            Variant::NorthSouth => false,
            Variant::EastWest =>
                [Variant::EastWest, Variant::NorthEast, Variant::SouthEast].contains(other),
            Variant::NorthEast => false,
            Variant::NorthWest =>
                [Variant::EastWest, Variant::NorthEast, Variant::SouthEast].contains(other),
            Variant::SouthEast => false,
            Variant::SouthWest =>
                [Variant::EastWest, Variant::NorthEast, Variant::SouthEast].contains(other),
            Variant::Ground => false,
        }
    }
}

fn is_in_bounds(i: i32, j: i32, m: i32, n: i32) -> bool {
    return i >= 0 && j >= 0 && i < m && j < n;
}

fn get_neighbors(i: i32, j: i32, tile: &Tile, tiles: &Vec<Vec<Tile>>) -> Vec<Tile> {
    let m = tiles.len();
    let n = tiles[0].len();
    let mut neighbors = vec![];
    // north
    if is_in_bounds(i - 1, j, m as i32, n as i32) {
        let neighbor = tiles[(i as usize) - 1][j as usize];
        if !neighbor.visited && tile.variant.is_north_compatible(&neighbor.variant) {
            neighbors.push(neighbor);
        }
    }
    // east
    if is_in_bounds(i, j + 1, m as i32, n as i32) {
        let neighbor = tiles[i as usize][(j as usize) + 1];
        if !neighbor.visited && tile.variant.is_east_compatible(&neighbor.variant) {
            neighbors.push(neighbor);
        }
    }
    // south
    if is_in_bounds(i + 1, j, m as i32, n as i32) {
        let neighbor = tiles[(i as usize) + 1][j as usize];
        if !neighbor.visited && tile.variant.is_south_compatible(&neighbor.variant) {
            neighbors.push(neighbor);
        }
    }
    // west
    if is_in_bounds(i, j - 1, m as i32, n as i32) {
        let neighbor = tiles[i as usize][(j as usize) - 1];
        if !neighbor.visited && tile.variant.is_west_compatible(&neighbor.variant) {
            neighbors.push(neighbor);
        }
    }
    return neighbors;
}

// eh, there's definitely a better way to do this, but for now I just opt to deduce the start pipe from its neighbors explicitly
fn replace_start(start: &mut Tile, neighbors: &Vec<Tile>) {
    let (nx1, ny1) = (
        neighbors[0].coordinates.0 - start.coordinates.0,
        neighbors[0].coordinates.1 - start.coordinates.1,
    );
    let (nx2, ny2) = (
        neighbors[1].coordinates.0 - start.coordinates.0,
        neighbors[1].coordinates.1 - start.coordinates.1,
    );
    let variant = match (nx1, ny1, nx2, ny2) {
        (1, 0, -1, 0) => Variant::NorthSouth,
        (-1, 0, 1, 0) => Variant::NorthSouth,
        (0, 1, -1, 0) => Variant::NorthEast,
        (-1, 0, 0, 1) => Variant::NorthEast,
        (-1, 0, 0, -1) => Variant::NorthWest,
        (0, -1, -1, 0) => Variant::NorthWest,
        (0, -1, 0, 1) => Variant::EastWest,
        (0, 1, 0, -1) => Variant::EastWest,
        (0, -1, 1, 0) => Variant::SouthWest,
        (1, 0, 0, -1) => Variant::SouthWest,
        (1, 0, 0, 1) => Variant::SouthEast,
        (0, 1, 1, 0) => Variant::SouthEast,
        (_, _, _, _) => panic!("Unknown coords"),
    };
    start.variant = variant;
}

fn find_loop(start_i: i32, start_j: i32, tiles: &mut Vec<Vec<Tile>>) {
    let mut stack = vec![(start_i, start_j, 1)];
    while let Some((i, j, current_step)) = stack.pop() {
        tiles[i as usize][j as usize].visited = true;
        // record the current "distance" that we're at. really we just need the total number of pipes in the loop which we can then divide by 2 to get the max distance
        tiles[i as usize][j as usize].distance = current_step;
        let tile = &tiles[i as usize][j as usize];
        let neighbors = get_neighbors(i, j, tile, tiles);
        for neighbor in neighbors {
            stack.push((neighbor.coordinates.0, neighbor.coordinates.1, current_step + 1));
        }
    }
}

// 1
// ----|----->
// 2
//   |   |
// --L---J--->
// 3
//   |
// --L---7--->
//       |
// 4
// --F---7--->
//   |   |
// 5
//       |
// --F---J--->
//   |

fn ray_trace_and_mark(tile_row: &mut Vec<Tile>) {
    let mut pipes = Vec::new();
    // start casting the ray
    let mut intersections = 0;
    for tile in tile_row {
        // non-loop tiles should also be considered
        if tile.variant == Variant::Ground || !tile.visited {
            // even/odd ray intersection
            tile.within = intersections % 2 == 1;
            continue;
        }
        // north-south pipes are always an intersection, east-west tiles are never an intersection (ray travels along edge)
        if tile.variant == Variant::NorthSouth {
            intersections += 1;
            continue;
        }
        // the ray travels along north-east pipes, but there are two possibilities:
        // either the ray exists on the same side of the loop (L--J) or the ray exits to the outside of the loop (L-7)
        // so we track this pipe as the beginning of the ray's "ride" along the loop edge
        if tile.variant == Variant::NorthEast {
            pipes.push(tile);
            continue;
        }
        if tile.variant == Variant::SouthWest {
            // check to see if the last hit pipe was the beginning of a "stair" piece (L-7), if so, then this was an intersection
            if pipes.len() > 0 && pipes.last().unwrap().variant == Variant::NorthEast {
                pipes.pop();
                intersections += 1;
            }
            // we don't need to record this pipe because there is no way for this pipe (7) to start a "ride" while the ray is traveling left to right
            continue;
        }
        // the ray travels along south-east pipes, but there are two possibilities:
        // either the ray exists on the same side of the loop (F--7) or the ray exits to the outside of the loop (F-J)
        // so we track this pipe as the beginning of the ray's "ride" along the loop edge
        if tile.variant == Variant::SouthEast {
            pipes.push(tile);
            continue;
        }
        if tile.variant == Variant::NorthWest {
            // check to see if the last hit pipe was the beginning of a "stair" piece (F-J), if so, then this was an intersection
            if pipes.len() > 0 && pipes.last().unwrap().variant == Variant::SouthEast {
                pipes.pop();
                intersections += 1;
            }
            // we don't need to record this pipe because there is no way for this pipe (J) to start a "ride" while the ray is traveling left to right
            continue;
        }
    }
}
