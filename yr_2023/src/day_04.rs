#![allow(unused)]

use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
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
    let lines = read_to_string("./src/day_04.txt")?;
    let lines = lines.lines();

    let mut cards = HashMap::new();

    for (i, line) in lines.enumerate() {
        let winning_nums_str = &line[10..39]
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let guess_nums_str = &line[42..]
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        let mut winning_count: i32 = 0;
        guess_nums_str.iter().for_each(|x| {
            if winning_nums_str.contains(x) {
                winning_count += 1;
            }
        });
        cards.insert(i, winning_count);
    }
    dbg!(cards.clone());

    let cards_num = cards.len();
    let mut queue: VecDeque<usize> = (0..cards_num).collect::<VecDeque<_>>();
    let mut map = HashMap::new();

    while let Some(el) = queue.pop_front() {
        // map.entry(el).and_modify(|e| *e += 1).or_insert(1);
        // dbg!("processing", el);
        if let Some(winning_count) = cards.get(&el) {
            if winning_count > &0 {
                let upper_bound = cards_num.min(*winning_count as usize + el as usize + 1);

                let range = (el + 1)..upper_bound;
                // dbg!(range.clone());
                for num_to_add in range {
                    queue.push_back(num_to_add);
                    map.entry(num_to_add).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
    }
    // el 2 w 4
    // 3 ,4 ,5, 6
    // 3 .. 7
    // 2,3,4,5,6
    // 3,4,5,6
    // dbg!(map.clone());

    Ok(map.values().sum::<i64>() + cards_num as i64)
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
