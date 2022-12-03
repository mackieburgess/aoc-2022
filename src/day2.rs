use anyhow::Result;
use std::fs;

fn game_outcome(game_sheet: &str) -> usize {
    match game_sheet {
        "A Y" | "B Z" | "C X" => 6,
        "A X" | "B Y" | "C Z" => 3,
        _ => 0
    }
}

fn game_score() -> Result<usize> {
    Ok(
        fs::read_to_string("./data/2.input")?
            .split("\n")
            .map(|game_sheet| {
                match game_sheet.chars().nth(2) {
                    Some('X') => 1 + game_outcome(game_sheet),
                    Some('Y') => 2 + game_outcome(game_sheet),
                    Some('Z') => 3 + game_outcome(game_sheet),
                    _ => 0
                }
            }).sum()
    )
}

fn main() -> Result<()> {
    println!("part one: {}", game_score()?);

    Ok(())
}
