enum Version {
    Old,
    New
}

fn game_outcome(game_sheet: &str) -> usize {
    match game_sheet {
        "A Y" | "B Z" | "C X" => 6, // winning game states
        "A X" | "B Y" | "C Z" => 3, // drawing board states
        _ => 0 // losing board states, or invalid state
    }
}

fn hand_played(game_sheet: &str) -> usize {
    match game_sheet {
        "A Y" | "B X" | "C Z" => 1, // "you play rock" states
        "A Z" | "B Y" | "C X" => 2, // "you play paper" states
        "A X" | "B Z" | "C Y" => 3, // "you play scissors" states
        _ => 0 // invalid state
    }
}

fn game_score(version: Version) -> usize {
    include_str!("../data/2.input")
        .split("\n")
        .map(|game_sheet| {
            if let Version::Old = version {
                // calculate the hand to use and add it to the outcome
                let hand_used = match game_sheet.chars().nth(2) {
                    Some('X') => 1,
                    Some('Y') => 2,
                    Some('Z') => 3,
                    _ => 0
                };

                hand_used + game_outcome(game_sheet)
            } else {
                // calculate the outcome and add it to the hand you play to get that outcome
                let outcome = match game_sheet.chars().nth(2) {
                    Some('X') => 0,
                    Some('Y') => 3,
                    Some('Z') => 6,
                    _ => 0
                };

                outcome + hand_played(game_sheet)
            }
        }).sum()
}


fn main() {
    println!("part one: {}", game_score(Version::Old));
    println!("part two: {}", game_score(Version::New));
}
