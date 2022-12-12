use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// explicitly implement Ord so the queue becomes a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip ordering on costs
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Coords = (usize, usize);

type TileAtFn = dyn Fn(Coords) -> char;

fn tile_height(tile: char) -> u32 {
    match tile {
        'a'..='z' => tile as u32 - 97,
        'S' => tile_height('a'),
        'E' => tile_height('z'),
        _ => unreachable!("invalid height"),
    }
}

fn is_reachable_from(current_tile: char, target_tile: char) -> bool {
    tile_height(current_tile) + 1 >= tile_height(target_tile)
}

fn reachable_tiles(
    tile_at: &TileAtFn,
    (x, y): Coords,
    num_rows: usize,
    num_cols: usize,
) -> Vec<Coords> {
    let current_tile = tile_at((x, y));
    let mut reachable_tiles = vec![];
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let (Some(tile_x), Some(tile_y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            if tile_x >= num_cols || tile_y >= num_rows {
                continue;
            }
            let tile = tile_at((tile_x, tile_y));
            if is_reachable_from(current_tile, tile) {
                reachable_tiles.push((tile_x, tile_y));
            }
        }
    }
    reachable_tiles
}

fn shortest_path(
    heightmap: Vec<char>,
    start: usize,
    end: usize,
    num_rows: usize,
    num_cols: usize,
) -> Option<usize> {
    let coords_to_pos = move |(x, y): (usize, usize)| y * num_cols + x;
    let pos_to_coords = move |pos: usize| (pos % num_cols, pos / num_cols);

    let mut dist: Vec<_> = (0..heightmap.len()).map(|_| usize::MAX).collect();
    let tile_at = move |(x, y): (usize, usize)| heightmap[coords_to_pos((x, y))];

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for (x, y) in reachable_tiles(&tile_at, pos_to_coords(position), num_rows, num_cols) {
            let pos = coords_to_pos((x, y));
            let next = State {
                cost: cost + 1,
                position: pos,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

pub fn main() {
    let input = include_str!("./input.txt");
    let num_rows = input.lines().count();
    let num_cols = input
        .lines()
        .next()
        .expect("non-empty input")
        .chars()
        .count();
    let input = input.to_string().replace("\n", "");
    let heightmap: Vec<_> = input.chars().collect();

    let start = input.find('S').expect("start exists");
    let end = input.find('E').expect("end exists");

    let shortest_from_start = shortest_path(heightmap.clone(), start, end, num_rows, num_cols);
    println!("day12a: {:?}", shortest_from_start);

    let shortest_from_any_a = input
        .match_indices('a')
        .filter_map(|(idx, _)| shortest_path(heightmap.clone(), idx, end, num_rows, num_cols))
        .min()
        .expect("path exists");
    println!("day12b: {shortest_from_any_a}");
}
