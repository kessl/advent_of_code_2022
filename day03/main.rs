#![feature(iter_next_chunk)]

use std::collections::HashSet;

const LOWERCASE_OFFSET: u32 = 'a' as u32 - 1; // a = 1, .., z = 26
const UPPERCASE_OFFSET: u32 = 'A' as u32 - 27; // A = 27, .., Z = 52

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - LOWERCASE_OFFSET,
        'A'..='Z' => item as u32 - UPPERCASE_OFFSET,
        _ => panic!("invalid item"),
    }
}

fn a() -> u32 {
    let input = include_str!("./input.txt").lines();
    let mut priority_sum = 0;
    for line in input {
        let (first, second) = line.split_at(line.len() / 2);
        let mut set = HashSet::with_capacity(first.len());
        for item in first.chars() {
            set.insert(item);
        }
        for item in second.chars() {
            if set.contains(&item) {
                priority_sum += priority(item);
                break;
            }
        }
    }
    priority_sum
}

fn b() -> u32 {
    let mut input = include_str!("./input.txt").lines();
    let mut priority_sum = 0;
    while let Ok(group) = input.next_chunk::<3>() {
        let mut item_sets = [
            HashSet::with_capacity(group[0].len()),
            HashSet::with_capacity(group[1].len()),
            HashSet::with_capacity(group[2].len()),
        ];
        for i in 0..3 {
            for item in group[i].chars() {
                item_sets[i].insert(item);
            }
        }
        let mut intersection: Vec<_> = item_sets[0].intersection(&item_sets[1]).collect();
        intersection.retain(|item| item_sets[2].contains(item));
        let badge = **intersection
            .first()
            .expect("there should be exactly one intersection");
        priority_sum += priority(badge);
    }
    priority_sum
}

pub fn main() {
    println!("day03a: {}", a());
    println!("day03b: {}", b());
}
