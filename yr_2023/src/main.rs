pub mod day_07;
use anyhow::Result;
use day_07::{part1, part2};

fn main() -> Result<()> {
    println!("Part1:");
    let _ = part1();
    println!("Part2:");
    let _ = part2();

    Ok(())
}
