use anyhow::Result;
use std::fs;

fn rucksack_priorities() -> Result<usize> {
    Ok(
        fs::read_to_string("./data/3.input")?
            .split("\n")
            .filter_map(|rucksack| {
                // split rucksack into two even halves
                let (lhs, rhs) = rucksack.split_at(rucksack.len() / 2);

                for val in lhs.chars() {
                    if rhs.contains(val) {
                        // convert ASCII value to annoying AOC priority system
                        if val as usize > 96 {
                            return Some((val as usize) - 96);
                        } else {
                            return Some((val as usize) - 38);
                        }
                    }
                };

                // if nothing is found, ignore
                return None;
            }).sum()
    )
}

fn main() -> Result<()> {
    println!("part 1: {}", rucksack_priorities()?);

    Ok(())
}
