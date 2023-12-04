pub mod day_04;
use anyhow::Result;

fn main() -> Result<()> {
    let res = day_04::run()?;
    println!("Answer \n->{res}");

    Ok(())
}
