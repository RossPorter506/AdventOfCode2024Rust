#![allow(dead_code)]

mod prelude; 
mod day2; use day2 as day;
use anyhow::Result;

fn main() -> Result<()> {
    let out = day::part2()?;
    println!("{out}");
    Ok(())
}
