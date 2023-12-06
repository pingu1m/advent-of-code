#![allow(unused)]

use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
    fs::{read, read_to_string, File},
    io::BufRead,
    io::BufReader,
};

pub fn check_map(el: i64, map: &[Vec<i64>]) -> i64 {
    for els in map {
        let start = els[1];
        let range = els[2];
        if el >= start && el <= (start + range) {
            let delta = el - start;
            // dbg!("bingo");
            // dbg!(els.clone(), delta);
            return els[0] + delta;
        }
    }
    el
}

pub fn run() -> Result<i64> {
    let lines = read_to_string("./src/day_05.txt")?;
    let lines = lines.lines().collect::<Vec<_>>();
    let seeds = &lines[0];
    let seeds = &seeds[7..]
        .split(' ')
        .flat_map(|x| x.parse::<i64>())
        .collect::<Vec<i64>>();

    let mut augmented_seeds = seeds.chunks(2).collect::<Vec<_>>();

    let seed_to_soil = &lines[3..12]
        .iter()
        .map(|el| {
            el.split(' ')
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    // dbg!(seed_to_soil);
    let mut step11 = vec![];
    let seed = augmented_seeds[0];
    let start = seed[0];
    let range = seed[1];
    for s in start..=(start + range) {
        step11.push(check_map(s, &seed_to_soil))
    }
    dbg!(step11);

    let step1 = seeds
        .iter()
        .map(|&x| check_map(x, &seed_to_soil))
        .collect::<Vec<i64>>();

    let soil_to_fertilizer = &lines[14..40]
        .iter()
        .map(|el| {
            el.split(' ')
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let step2 = step1
        .iter()
        .map(|&x| check_map(x, &soil_to_fertilizer))
        .collect::<Vec<i64>>();

    Ok(2)
    // let fertilizer_to_water = &lines[42..71]
    //     .iter()
    //     .map(|el| {
    //         el.split(' ')
    //             .flat_map(|x| x.parse::<i64>())
    //             .collect::<Vec<i64>>()
    //     })
    //     .collect::<Vec<_>>();
    // let step3 = step2
    //     .iter()
    //     .map(|&x| check_map(x, &fertilizer_to_water))
    //     .collect::<Vec<i64>>();
    // let water_to_light = &lines[73..94]
    //     .iter()
    //     .map(|el| {
    //         el.split(' ')
    //             .flat_map(|x| x.parse::<i64>())
    //             .collect::<Vec<i64>>()
    //     })
    //     .collect::<Vec<_>>();
    // let step4 = step3
    //     .iter()
    //     .map(|&x| check_map(x, &water_to_light))
    //     .collect::<Vec<i64>>();
    // let light_to_temperature = &lines[96..115]
    //     .iter()
    //     .map(|el| {
    //         el.split(' ')
    //             .flat_map(|x| x.parse::<i64>())
    //             .collect::<Vec<i64>>()
    //     })
    //     .collect::<Vec<_>>();
    // let step5 = step4
    //     .iter()
    //     .map(|&x| check_map(x, &light_to_temperature))
    //     .collect::<Vec<i64>>();
    // let temperature_to_humidity = &lines[117..160]
    //     .iter()
    //     .map(|el| {
    //         el.split(' ')
    //             .flat_map(|x| x.parse::<i64>())
    //             .collect::<Vec<i64>>()
    //     })
    //     .collect::<Vec<_>>();
    // let step6 = step5
    //     .iter()
    //     .map(|&x| check_map(x, &temperature_to_humidity))
    //     .collect::<Vec<i64>>();
    // let humidity_to_location = &lines[162..189]
    //     .iter()
    //     .map(|el| {
    //         el.split(' ')
    //             .flat_map(|x| x.parse::<i64>())
    //             .collect::<Vec<i64>>()
    //     })
    //     .collect::<Vec<_>>();
    // let step7 = step6
    //     .iter()
    //     .map(|&x| check_map(x, &humidity_to_location))
    //     .collect::<Vec<i64>>();
    // dbg!(humidity_to_location);
    // dbg!(step7.clone());

    // Ok(step7.iter().min().unwrap().to_owned())
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
