use anyhow::Result;
use std::fs;

fn most_plentiful_elves(amount: usize) -> Result<usize> {
    let mut elves: Vec<usize> = fs::read_to_string("./data/1.input")?
        .split("\n\n")
        .map(|bundle| {
            bundle
                .split("\n")
                // filter by values that can be usize
                .filter_map(|val| val.parse::<usize>().ok())
                .sum()
        }).collect();

    elves.sort_by(|a,b| b.cmp(a));

    // TODO: doesn't work when there aren't enough elves. I'm lazy.
    if elves.len() >= amount {
        Ok(elves[0..amount].iter().sum())
    } else {
        Ok(0)
    }
}

fn main() -> Result<()> {
    println!("part one: {}", most_plentiful_elves(1)?);
    println!("part two: {}", most_plentiful_elves(3)?);

    Ok(())
}
