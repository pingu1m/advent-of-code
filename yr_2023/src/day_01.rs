#![allow(unused)]

use std::{
    fs::{read, read_to_string, File},
    io::BufRead,
    io::BufReader,
    io::Read,
};

pub fn read_file_diff_ways() {
    // let contents = read_to_string("./src/part1.txt").expect("File should exists");
    // 1
    let contents: String = read_to_string("./src/part1.txt").expect("File should exists");
    // 2
    let contents: Vec<u8> = read("./src/part1.txt").expect("File should exists");
    // 3
    let file = File::open("./src/part1.txt").expect("File should exists");
    let reader = BufReader::new(file);
    // 4
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let filepath = "./src/part1.txt";
    let mut file = File::open(filepath).expect("err");

    loop {
        let read_count = file.read(&mut buffer).expect("err");
        // do_something(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
}

pub fn fix_line(line: String) -> String {
    let replacements = vec![
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut indices = vec![];
    for (w_num, num_num) in replacements {
        let mut v: Vec<_> = line.match_indices(w_num).collect();
        indices.extend(v);
    }
    indices.sort_by(|a, b| a.0.cmp(&b.0));
    // dbg!(indices.clone());
    let res = format!("{}{}", indices[0].1, indices[indices.len() - 1].1);
    res.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

pub fn run() -> i32 {
    let file = File::open("./src/part1.txt").expect("File should exists");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("Should exists");
            let fixed_line = fix_line(line);
            fixed_line.parse::<i32>().expect("should be integer")
        })
        .sum()
}

pub fn run2() -> i32 {
    // let contents = read_to_string("./src/part1.txt").expect("File should exists");
    let contents: String = read_to_string("./src/part1.txt").expect("File should exists");
    let contents: Vec<u8> = read("./src/part1.txt").expect("File should exists");
    let file = File::open("./src/part1.txt").expect("File should exists");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("Should exists");
            let nums: Vec<&str> = line.matches(char::is_numeric).collect();

            format!("{}{}", nums[0], nums[nums.len() - 1])
                .parse::<i32>()
                .expect("should be integer")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(run(), 2)
    }
}
