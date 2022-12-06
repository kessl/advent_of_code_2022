fn find_marker(input: &Vec<char>, length: usize) -> usize {
    let mut windows = input.windows(length).enumerate();
    let mut uniq = std::collections::HashSet::with_capacity(length);
    while let Some((i, chars)) = windows.next() {
        let mut all_unique = true;
        for j in 0..length {
            all_unique &= uniq.insert(chars[j]);
        }
        if all_unique {
            return i + length;
        }
        uniq.clear();
    }
    unreachable!("no marker found");
}

pub fn main() {
    let input: Vec<_> = include_str!("./input.txt").chars().collect();
    println!("day06a: {}", find_marker(&input, 4));
    println!("day06b: {}", find_marker(&input, 14));
}
