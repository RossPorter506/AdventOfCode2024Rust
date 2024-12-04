#![allow(dead_code)]

mod prelude; 
mod day1; use day1 as day;
use anyhow::Result;

fn main() -> Result<()> {
    let out = day::part2()?;
    println!("{out}");
    Ok(())
}
