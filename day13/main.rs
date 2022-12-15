#![feature(iter_array_chunks)]

use std::{cmp::Ordering, iter::Peekable};

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Integer(u8),
    BeginList,
    EndList,
    Separator,
}

fn next_token<I>(packet: &mut Peekable<I>, next: &mut Option<Token>) -> Token
where
    I: Iterator<Item = char>,
{
    if let Some(token) = next.take() {
        return token;
    }

    let ch = packet.next().unwrap();
    match ch {
        '0'..='9' => {
            let val = ch as u8 - 48;
            let next = packet.peek().unwrap();
            match &next {
                '0'..='9' => Token::Integer(val * 10 + (packet.next().unwrap() as u8 - 48)),
                _ => Token::Integer(val),
            }
        }
        '[' => Token::BeginList,
        ']' => Token::EndList,
        ',' => Token::Separator,
        _ => unreachable!("invalid token"),
    }
}

fn correct_order(left: &str, right: &str) -> bool {
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();

    let mut next_l = None;
    let mut next_r = None;
    let mut l = next_token(&mut left, &mut next_l);
    let mut r = next_token(&mut right, &mut next_r);

    loop {
        if l == Token::Separator {
            l = next_token(&mut left, &mut next_l);
        }
        if r == Token::Separator {
            r = next_token(&mut right, &mut next_r);
        }
        // println!("{:?} vs {:?}", l, r);

        match (&l, &r) {
            (Token::Integer(lval), Token::Integer(rval)) => {
                if lval == rval {
                    l = next_token(&mut left, &mut next_l);
                    r = next_token(&mut right, &mut next_r);
                    continue;
                }
                return lval < rval;
            }

            (Token::EndList, Token::Integer(_) | Token::BeginList) => return true, // left side ran out of items
            (Token::Integer(_) | Token::BeginList, Token::EndList) => return false, // right side ran out of items

            (Token::BeginList, Token::BeginList) | (Token::EndList, Token::EndList) => {
                l = next_token(&mut left, &mut next_l);
                r = next_token(&mut right, &mut next_r);
                continue;
            }

            (Token::Integer(_), Token::BeginList) => {
                r = next_token(&mut right, &mut next_r);
                next_l = Some(Token::EndList);
                continue;
            }
            (Token::BeginList, Token::Integer(_)) => {
                l = next_token(&mut left, &mut next_l);
                next_r = Some(Token::EndList);
                continue;
            }

            (Token::Separator, _) => unreachable!(),
            (_, Token::Separator) => unreachable!(),
        }
    }
}

fn compare(left: &str, right: &str) -> std::cmp::Ordering {
    if correct_order(left, right) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn main() {
    let input = include_str!("./input.txt");

    let mut sum_indices = 0;
    for (index, [left, right, _]) in input.lines().array_chunks().enumerate() {
        let correct = correct_order(left, right);
        if correct {
            sum_indices += index + 1;
        }
    }
    println!("day13a: {sum_indices}");

    let mut packets: Vec<_> = input.lines().filter(|&line| !line.is_empty()).collect();
    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|left, right| compare(left, right));
    let idx_1 = packets
        .iter()
        .position(|&packet| packet == "[[2]]")
        .unwrap()
        + 1;
    let idx_2 = packets
        .iter()
        .position(|&packet| packet == "[[6]]")
        .unwrap()
        + 1;
    println!("day13b: {}", idx_1 * idx_2);
}

#[test]
fn test_input() {
    let input = include_str!("./test_input.txt");
    let mut sum_indices = 0;
    for (index, [left, right, _]) in input.lines().array_chunks().enumerate() {
        if correct_order(left, right) {
            sum_indices += index + 1;
        }
    }
    assert_eq!(13, sum_indices);
}
