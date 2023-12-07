#![allow(unused)]
use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use anyhow::Result;

pub fn part1() -> Result<i32> {
    let raw_lines = read_to_string("./src/day_07.txt")?;
    let _lines = raw_lines.lines();
    let sample = r#"
AAAAA 324
AA8AA 233
23332 323
TTT98 333
23432 233
A23A4 234 
23456 122
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    let sample = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
8JJKJ 222
QQQJA 483"#;

    pub fn get_kind(map: &HashMap<char, i32>) -> i32 {
        let max_count = map.values().max().unwrap();
        let mut joker = 0;
        if let Some(v) = map.get(&'J') {
            joker += v;
        }
        // dbg!(map.clone());
        let c = match map.len() {
            1 => 1,
            2 => {
                if max_count == &3 {
                    3
                } else {
                    2
                }
            }
            3 => {
                if max_count == &2 {
                    5
                } else {
                    4
                }
            }
            4 => 6,
            _ => 7,
        };

        match c {
            1 => 1,
            2 => {
                if joker > 0 {
                    1
                } else {
                    2
                }
            }
            3 => {
                if joker > 0 {
                    1
                } else {
                    3
                }
            }
            4 => {
                if joker > 0 {
                    c - 2
                } else {
                    c
                }
            }
            5 => {
                if joker > 0 {
                    c - joker - 1
                } else {
                    c
                }
            }
            6 => {
                if joker > 0 {
                    4
                } else {
                    6
                }
            }
            _ => c - joker,
        }
    }

    pub fn cmp_hands(a: &str, b: &str) -> Ordering {
        let mut card_value = HashMap::new();
        card_value.insert('2', 2);
        card_value.insert('3', 3);
        card_value.insert('4', 4);
        card_value.insert('5', 5);
        card_value.insert('6', 6);
        card_value.insert('7', 7);
        card_value.insert('8', 8);
        card_value.insert('9', 9);
        card_value.insert('T', 10);
        card_value.insert('J', 1);
        card_value.insert('Q', 12);
        card_value.insert('K', 13);
        card_value.insert('A', 14);
        let a = a.chars().collect::<Vec<char>>();
        let b = b.chars().collect::<Vec<char>>();

        for i in 0..a.len() {
            let c_a = card_value.get(&a[i]).unwrap();
            let c_b = card_value.get(&b[i]).unwrap();
            if c_a > c_b {
                return Ordering::Greater;
            }
            if c_b > c_a {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    pub fn parse_line(raw: &str) -> (String, String, i32) {
        let raw_parsed: Vec<_> = raw.split_whitespace().collect();
        let (hand, _bid) = (raw_parsed[0], raw_parsed[1]);

        let mut c_map = HashMap::new();
        for c in hand.chars() {
            c_map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        let kind = get_kind(&c_map);
        (hand.into(), _bid.into(), kind)
    }
    let mut hands = vec![];
    // for line in sample[1..].lines() {
    //     let (hand, bid, kind) = parse_line(line);
    //     hands.push((hand, bid, kind))
    // }
    for line in _lines {
        let (hand, bid, kind) = parse_line(line);
        hands.push((hand, bid, kind))
    }

    hands.sort_by(|a, b| {
        let (h_a, _, a_val) = a;
        let (h_b, _, b_val) = b;
        if a_val < b_val {
            return Ordering::Greater;
        } else if a_val > b_val {
            return Ordering::Less;
        } else {
            cmp_hands(h_a, h_b)
        }
    });

    // dbg!(hands.clone());
    let mut total = 0;
    for (i, h) in hands.iter().enumerate() {
        total += (i + 1) as i32 * h.1.parse::<i32>().unwrap();
    }
    println!("Answer: {total}");

    Ok(total)
}

pub fn part2() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(part1().unwrap_or(0), 251735672)
    }
}
