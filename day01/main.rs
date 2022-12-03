pub fn main() {
    let lines: Vec<_> = include_str!("./input.txt").lines().collect();
    let mut totals: Vec<_> = lines
        .split(|&line| line == "")
        .map(|food_items| {
            food_items
                .iter()
                .map(|item| item.parse::<i32>().expect("should be a number"))
                .sum::<i32>()
        })
        .collect();
    totals.sort_unstable();

    let max = totals.iter().rev().next().unwrap();
    println!("day01a: {max}");

    let top_3_sum: i32 = totals.iter().rev().take(3).sum();
    println!("dayO1b: {top_3_sum}");
}

