use std::collections::HashSet;

#[derive(Debug)]
struct Motion {
    direction: Direction,
    distance: u8,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (i32, i32);

fn parse_line(line: &str) -> Motion {
    let (direction, distance) = line
        .split_once(" ")
        .expect("direction followed by distance");
    Motion {
        direction: match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!("invalid direction"),
        },
        distance: distance.parse().expect("numeric distance"),
    }
}

fn do_move((pos_x, pos_y): Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (pos_x, pos_y + 1),
        Direction::Down => (pos_x, pos_y - 1),
        Direction::Left => (pos_x - 1, pos_y),
        Direction::Right => (pos_x + 1, pos_y),
    }
}

fn follow((pos_x, pos_y): Position, (target_x, target_y): Position) -> Position {
    let diff_x = target_x - pos_x;
    let diff_y = target_y - pos_y;

    // don't move if within one tile of target
    if diff_x >= -1 && diff_x <= 1 && diff_y >= -1 && diff_y <= 1 {
        return (pos_x, pos_y);
    }

    // move by up to (+-1, +-1), prefer diagonally
    let move_x = diff_x.clamp(-1, 1);
    let move_y = diff_y.clamp(-1, 1);

    (pos_x + move_x, pos_y + move_y)
}

// ........
// .....T..
// ....o..H
#[allow(dead_code)]
fn print(head: Position, tail: Position) {
    let max = head.0.max(head.1).max(tail.0).max(tail.1).abs();
    let min = head.0.min(head.1).min(tail.0).min(tail.1).abs();
    let size = (max + min + 1) as usize;

    for y in (0..size).rev() {
        let mut line = ".".repeat(size);
        
        if y == 0 { // reference point
            line.replace_range((min as usize)..=(min as usize), "o");
        }
        
        let tail_y = (tail.1 + min) as usize;
        if tail_y == y { // tail
            let tail_x = (tail.0 + min) as usize;
            line.replace_range(tail_x..=tail_x, "T");
        }

        let head_y = (head.1 + min) as usize;
        if head_y == y { // head
            let head_x = (head.0 + min) as usize;
            line.replace_range(head_x..=head_x, "H");
        }
        println!("{}", line);
    }
}

// rope with 2 knots
fn a(input: &Vec<Motion>) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::with_capacity(2 * input.len());
    visited.insert(tail);

    for motion in input {
        for _ in 0..motion.distance {
            head = do_move(head, &motion.direction);
            // print(head, tail);
            tail = follow(tail, head);
            // println!("->");
            // print(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

// rope with 10 knots
fn b(input: &Vec<Motion>) -> usize {
    let mut knots = [(0, 0); 10];
    let mut visited = HashSet::with_capacity(input.len());
    visited.insert(knots[9]);

    for motion in input {
        for _ in 0..motion.distance {
            for i in 0..10 {
                if i == 0 {
                    knots[i] = do_move(knots[i], &motion.direction);
                } else {
                    knots[i] = follow(knots[i], knots[i - 1]);
                }

                if i == 9 {
                    visited.insert(knots[i]);
                }
            }
        }
    }

    visited.len()
}

pub fn main() {
    let input: Vec<_> = include_str!("./input.txt")
        .lines()
        .map(parse_line)
        .collect();

    println!("day09a: {}", a(&input));
    println!("day09b: {}", b(&input));
}
