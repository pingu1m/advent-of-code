#![allow(unused)]

use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{read, read_to_string, File},
    io::BufRead,
    io::BufReader,
};

pub fn run() -> Result<i64> {
    // let file = File::open("./src/day_03.txt")?;
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     let line = line?;
    // }
    let els = read_to_string("./src/sud.txt")?.chars().collect::<Vec<_>>();
    let t = &els[5..260];

    // dbg!(&els[3..6]);
    // dbg!(&els[144..147]); // plus 141
    // dbg!(&els[285..288]); // plus 141

    let (mut num_start, mut num_end) = (0, 0);

    let mut valid_nums = vec![];

    let mut gear_map: HashMap<usize, Vec<i64>> = HashMap::new();
    while num_end < els.len() {
        if "0123456789".contains(els[num_start]) {
            while "0123456789".contains(els[num_end]) {
                num_end += 1;
            }
            // dbg!(num_start, num_end, &els[num_start..num_end]);
            let (is_part, maybe_gear, gear_i) = is_engine_part(num_start, num_end, &els);
            if is_part {
                let parsed_num = &els[num_start..num_end].iter().collect::<String>();
                let parsed_num = parsed_num.parse::<i64>()?;
                if maybe_gear {
                    gear_map
                        .entry(gear_i)
                        .and_modify(|x| x.push(parsed_num))
                        .or_insert(vec![parsed_num]);
                }
                valid_nums.push(parsed_num);
            }

            num_end += 1;
            num_start = num_end;
        } else {
            num_start += 1;
            num_end += 1;
        }
    }
    let gear_ratio = gear_map
        .iter()
        .map(|pair| {
            if pair.1.len() != 2 {
                0
            } else {
                pair.1.iter().product()
            }
        })
        .sum::<i64>();

    // dbg!(valid_nums.clone());
    // dbg!(gear_map);
    // dbg!(gear_ratio);

    // Ok(valid_nums.iter().sum())
    Ok(gear_ratio)
}

fn is_engine_part(num_start: usize, num_end: usize, els: &[char]) -> (bool, bool, usize) {
    let mut ils: Vec<usize> = vec![];
    let els_len = els.len();
    // account for new lines indices cap by using mod len string
    if num_end > num_start {
        if num_start > 0 {
            ils.extend((num_start - 1)..=num_end);
        } else {
            ils.extend(num_start..=num_end);
        }
    } else {
        ils.push(num_start);
    }
    let mut res = vec![];
    for i in ils {
        res.push(i);
        if (i as i64 - 141) >= 0 {
            res.push(i - 141);
        }
        if i + 141 <= els_len {
            res.push(i + 141);
        }
    }
    for el in res {
        let c = els[el];
        if !"0123456789.".contains(c) && c != '\n' {
            if c == '*' {
                return (true, true, el);
            }
            return (true, false, 0);
        }
        // dbg!(els[el]);
    }
    (false, false, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = 21;
        assert_eq!(run().expect("err"), input)
    }
}
