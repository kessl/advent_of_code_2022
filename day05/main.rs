#![feature(iter_next_chunk)]

type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    pub amount: usize,
    pub source: usize,
    pub target: usize,
}

fn parse_move(line: &&str) -> Move {
    let mut tokens = line.split_whitespace();
    tokens.next();
    let amount = tokens.next().expect("move <num>").parse::<usize>().expect("number");
    tokens.next();
    let source = tokens.next().expect("from <num>").parse::<usize>().expect("number") - 1;
    tokens.next();
    let target = tokens.next().expect("to <num>").parse::<usize>().expect("number") - 1;
    Move { amount, source, target }
}

fn parse_input() -> (Vec<Stack>, Vec<Move>) {
    // starting stacks and moves are separated by empty line
    let sections: Vec<_> = include_str!("./input.txt").lines().collect();
    let separator_position = sections.iter().position(|line| *line == "").expect("empty line separator");
    let (stacks_lines, moves_lines) = sections.split_at(separator_position);

    // initialize stacks & fill up stacks with starting crates
    let mut stacks_iter = stacks_lines.into_iter().rev();
    let num_stacks = stacks_iter.next().expect("labeled stacks").split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    for line in stacks_iter {
        let mut chars = line.chars().enumerate();
        while let Ok([_, (offset, crate_char), _]) = chars.next_chunk() {
            let index = offset / 4;
            if !crate_char.is_whitespace() {
                stacks[index].push(crate_char);
            }
            chars.next();
        }
    }

    // parse moves
    let moves: Vec<_> = moves_lines.iter().skip(1).map(parse_move).collect();

    (stacks, moves)
}

fn mark_top_crates(stacks: &Vec<Stack>) -> String {
    let mut top_crates = String::with_capacity(stacks.len());
    for stack in stacks {
        top_crates.push(*stack.last().expect("valid result"))
    }
    top_crates
}


// move crates one by one
fn a(mut stacks: Vec<Stack>, moves: &Vec<Move>) -> String {
    for step in moves {
        for _ in 0..step.amount {
            let item = stacks[step.source].pop().expect("valid instructions");
            stacks[step.target].push(item);
        }
    }

    mark_top_crates(&stacks)
}

// move crates all at once
fn b(mut stacks: Vec<Stack>, moves: &Vec<Move>) -> String {
    for step in moves {
        let source = &mut stacks[step.source];
        let mut items: Vec<_> = source.splice(source.len() - step.amount..source.len(), []).collect();
        let target = &mut stacks[step.target];
        target.append(&mut items);
    }

    mark_top_crates(&stacks)
}

pub fn main() {
    let (stacks, moves) = parse_input();
    let top_crates_9000 = a(stacks.clone(), &moves);
    let top_crates_9001 = b(stacks, &moves);
    println!("day05a: {top_crates_9000}");
    println!("day05b: {top_crates_9001}");
}
