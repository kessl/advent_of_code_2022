type Section = std::ops::RangeInclusive<u32>;

fn parse_line(line: &str) -> [Section; 2] {
    let (first, second) = line.split_once(",").expect("two ranges");
    let (first_start, first_end) = first.split_once("-").expect("first range");
    let (second_start, second_end) = second.split_once("-").expect("second range");
    [
        (first_start.parse::<u32>().expect("number")..=first_end.parse::<u32>().expect("number")),
        (second_start.parse::<u32>().expect("number")..=second_end.parse::<u32>().expect("number")),
    ]
}

fn fully_contains([first, second]: &&[Section; 2]) -> bool {
    (first.contains(second.start()) && first.contains(second.end()))
        || (second.contains(first.start()) && second.contains(first.end()))
}

fn overlap([first, second]: &&[Section; 2]) -> bool {
    (first.end() >= second.start()) && (first.start() <= second.end())
}

pub fn main() {
    let sections: Vec<_> = include_str!("./input.txt")
        .lines()
        .map(parse_line)
        .collect();
    
    let contained_count = sections.iter().filter(fully_contains).count();
    println!("day04a: {contained_count}");
    
    let overlap_count = sections.iter().filter(overlap).count();
    println!("day04b: {overlap_count}");
}
