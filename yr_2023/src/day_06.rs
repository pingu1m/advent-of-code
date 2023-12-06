#![allow(unused)]

use anyhow::Result;
use std::fs::read_to_string;

pub fn part1() -> Result<i32> {
    let lines = read_to_string("./src/day_06.txt")?;
    let lines = lines
        .lines()
        .map(|x| x.split_once(": ").unwrap().1)
        .map(|x| {
            x.split_whitespace()
                .flat_map(|xx| xx.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let r1 = (lines[0][0], lines[1][0]);
    // let r2 = (lines[0][1], lines[1][1]);
    // let r3 = (lines[0][2], lines[1][2]);
    // let r4 = (lines[0][3], lines[1][3]);

    // let races = vec![r1, r2, r3, r4];

    let races = vec![r1];

    let mut wins = vec![];
    for race in races {
        let (total_secs, record) = race;
        let mut local_wins = vec![];

        for i in 1..total_secs {
            let remaining_secs = total_secs - i;
            let total = i * remaining_secs;
            if total > record {
                local_wins.push(i);
            }
        }
        wins.push(local_wins.len());
    }
    dbg!(wins.clone());

    Ok(wins.into_iter().map(|x| x as i32).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(part1().unwrap(), 32);
    }
}
