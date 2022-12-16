#[derive(PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

type Cavern = Vec<Vec<Tile>>;

fn print_tile(tile: &Tile) -> char {
    match tile {
        Tile::Air => '.',
        Tile::Rock => '#',
        Tile::Sand => 'o',
    }
}

fn print(cavern: &Cavern) {
    print!("\x1B[2J\x1B[1;1H"); // clear screen & reset cursor
    for row in cavern.iter().take(50) {
        let row_string: String = row.iter().skip(400).take(200).map(print_tile).collect();
        println!("{row_string}");
    }
}

// God help us

fn build_cavern(input: &str) -> Cavern {
    let mut cavern: Vec<Vec<_>> = (0..200)
        .map(|_| (0..1000).map(|_| Tile::Air).collect())
        .collect();
    for line in input.lines() {
        let points: Vec<_> = line.split(" -> ").collect();
        let mut windows = points.windows(2);
        while let Some(&[start, end]) = windows.next() {
            let (start_x, start_y) = start.split_once(",").expect("valid segment");
            let (end_x, end_y) = end.split_once(",").expect("valid segment");

            let start_x = start_x.parse::<usize>().expect("numeric");
            let end_x = end_x.parse::<usize>().expect("numeric");
            let start_y = start_y.parse::<usize>().expect("numeric");
            let end_y = end_y.parse::<usize>().expect("numeric");

            // println!("{start_x},{start_y} -> {end_x},{end_y}");

            let dist_x = end_x.max(start_x) - end_x.min(start_x);
            let dist_y = end_y.max(start_y) - end_y.min(start_y);

            let start_x = start_x.min(end_x);
            let start_y = start_y.min(end_y);

            // println!("{dist_x} {dist_y}");

            if dist_x > dist_y {
                for dx in 0..=dist_x {
                    cavern[start_y][start_x + dx] = Tile::Rock;
                }
            } else {
                for dy in 0..=dist_y {
                    cavern[start_y + dy][start_x] = Tile::Rock;
                }
            }
        }
        println!("");
    }
    cavern
}

fn drop_sand(cavern: &mut Cavern, lowest_row: usize) -> bool {
    let (mut sand_x, mut sand_y) = (500, 0);
    let possible_moves = [(0, 1), (-1, 1), (1, 1)].iter();

    'fall: loop {
        // std::thread::sleep(std::time::Duration::from_millis(3));
        // print(&cavern);
        // std::io::stdout().flush().unwrap();
        // let mut line = String::new();
        // std::io::stdin()
        //     .read_line(&mut line)
        //     .expect("Error: Could not read a line");

        let mut moves = possible_moves.clone();
        while let Some((dx, dy)) = moves.next() {
            if let Some(target_row) = cavern.get(sand_y + dy) {
                if let Some(target_tile) = target_row.get((sand_x + dx) as usize) {
                    if *target_tile == Tile::Air {
                        cavern[sand_y][sand_x as usize] = Tile::Air;
                        sand_x += dx;
                        sand_y += dy;
                        cavern[sand_y][sand_x as usize] = Tile::Sand;

                        if sand_y == lowest_row {
                            return false
                        }
                        continue 'fall;
                    }
                }
            }
        }

        if sand_y == 0 {
            return false
        }

        return true
    }
}

pub fn main() {
    let input = include_str!("./input.txt");

    // let mut c = build_cavern(input);
    // let lowest_row = c
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, row)| {
    //         if row.iter().any(|tile| *tile == Tile::Rock) {
    //             idx
    //         } else {
    //             0
    //         }
    //     })
    //     .max()
    //     .unwrap();
    // println!("lowest: {lowest_row}");

    // let mut sand_dropped = 0;
    // while drop_sand(&mut c, lowest_row) {
    //     sand_dropped += 1;
    // }

    // println!("day14a: {sand_dropped}");

    let mut c = build_cavern(input);
    let lowest_row = c
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            if row.iter().any(|tile| *tile == Tile::Rock) {
                idx
            } else {
                0
            }
        })
        .max()
        .unwrap();
    println!("lowest: {lowest_row}");
    c[lowest_row + 2] = (0..1000).map(|_| Tile::Rock).collect();

    let mut sand_dropped = 0;
    while drop_sand(&mut c, 199) {
        sand_dropped += 1;
    }

    println!("day14b: {}", sand_dropped + 1);
}

#[test]
pub fn test_input_b() {
    let input = include_str!("./test_input.txt");
    let mut c = build_cavern(input);
    let lowest_row = c
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            if row.iter().any(|tile| *tile == Tile::Rock) {
                idx
            } else {
                0
            }
        })
        .max()
        .unwrap();
    c[lowest_row + 2] = (0..1000).map(|_| Tile::Rock).collect();

    let mut sand_dropped = 0;
    while drop_sand(&mut c, 199) {
        sand_dropped += 1;
    }
    println!("{sand_dropped}");
    // assert_eq!(24, sand_dropped);
    assert_eq!(93, sand_dropped + 1);
}
