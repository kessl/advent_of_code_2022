use std::ops::RangeInclusive;

type Position = (i32, i32);
type Positions = Vec<(Position, Position)>;

fn load_input(input: &str, limit: Option<RangeInclusive<i32>>) -> Positions {
    input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(|ch| ch == '=' || ch == ',' || ch == ':');
            split.next();
            let sensor_x = split
                .next()
                .expect("sensor x coord")
                .parse::<i32>()
                .expect("numeric");
            split.next();
            let sensor_y = split
                .next()
                .expect("sensor y coord")
                .parse::<i32>()
                .expect("numeric");
            split.next();
            let beacon_x = split
                .next()
                .expect("beacon x coord")
                .parse::<i32>()
                .expect("numeric");
            split.next();
            let beacon_y = split
                .next()
                .expect("beacon y coord")
                .parse::<i32>()
                .expect("numeric");

            if let Some(limit) = &limit {
                if &sensor_x < limit.start() || &beacon_x < limit.start() || &sensor_y > limit.end() || &beacon_y > limit.end() {
                    return None
                }
            }

            Some(((sensor_x, sensor_y), (beacon_x, beacon_y)))
        })
        .collect()
}

fn distance((x1, y1): &Position, (x2, y2): &Position) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by_key(|range| *range.end());
    ranges.sort_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    for i in 1..ranges.len() {
        let last_range = merged_ranges.last_mut().unwrap();
        if ranges[i].start() <= last_range.end() {
            if ranges[i].end() > last_range.end() {
                // extend previous range
                *last_range = *last_range.start()..=*ranges[i].end();
            }
            // else range is contained in previous
        } else {
            // not contiguous
            merged_ranges.push(ranges[i].clone());
        }
    }

    merged_ranges
}

fn intersects_row(positions: &Positions, target_row: i32) -> Vec<RangeInclusive<i32>> {
    let mut covered_tiles = vec![];
    for (sensor, beacon) in positions {
        let radius = distance(sensor, beacon);

        // if sensor area intersects target row
        if ((sensor.1 - radius)..=(sensor.1 + radius)).contains(&target_row) {
            // find intersection x coords
            let x1 = sensor.0 - radius + (sensor.1 - target_row).abs();
            let x2 = sensor.0 + radius - (sensor.1 - target_row).abs();
            covered_tiles.push(x1..=x2);
        }
    }

    // merge overlapping ranges
    merge_ranges(covered_tiles)
}

pub fn main() {
    let input = include_str!("./input.txt");

    // let positions = load_input(input);
    // let covered_ranges = intersects_row(&positions, 2_000_000);
    // let num_covered_tiles: i32 = covered_ranges
    //     .iter()
    //     .map(|range| range.end() - range.start())
    //     .sum();
    // println!("day15a: {num_covered_tiles}");

    let positions = load_input(input, Some(0..=4_000_000));
    for row in 0..4_000_000 {
        let covered_ranges = intersects_row(&positions, row);
        if covered_ranges.len() > 1 {
            println!("day15b: found in row {row}: {:?}, frequency: {}", covered_ranges, (covered_ranges[0].end() + 1) * 4_000_000 + row);
            break;
        }
    }
}

#[test]
pub fn test_input_a() {
    let input = include_str!("./test_input.txt");
    let positions = load_input(input, None);
    let covered_ranges = intersects_row(&positions, 10);
    let num_covered_tiles: i32 = covered_ranges
        .iter()
        .map(|range| range.end() - range.start())
        .sum();
    assert_eq!(26, num_covered_tiles);
}
