use anyhow::Result;
use std::fs;

fn most_food() -> Result<usize> {
    let calories_stashed: Vec<usize> = fs::read_to_string("./data/1.input")?
        .split("\n\n")
        .map(|bundle| {
            bundle
                .split("\n")
                .filter_map(|val| val.parse::<usize>().ok())
                .sum()
        }).collect();

    let most_calories = calories_stashed.iter().max();

    if let Some(elf) = most_calories {
        Ok(elf.clone())
    } else {
        Ok(0)
    }
}

fn main() -> Result<()> {
    println!("part one: {}", most_food()?);

    Ok(())
}
