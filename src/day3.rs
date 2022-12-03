fn rucksack_priorities() -> usize {
    include_str!("../data/3.input")
        .split('\n')
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
}

fn team_rucksack_priorities() -> usize {
    let rucksacks = include_str!("../data/3.input")
        .split('\n')
        .collect::<Vec<&str>>();

    let mut priority_key_sum = 0;

    for (idx, rucksack) in rucksacks.iter().enumerate() {
        // only search on every third rucksack
        if idx % 3 == 2 {
            for val in rucksack.chars() {
                // check if the item type is shared amongst all three rucksacks in the set
                if rucksacks[idx-1].contains(val) &&
                    rucksacks[idx-2].contains(val)
                {
                    // convert to priority system and add to keys
                    if val as usize > 96 {
                        priority_key_sum += (val as usize) - 96;
                    } else {
                        priority_key_sum += (val as usize) - 38;
                    }

                    break;
                }
            }
        }
    }

    priority_key_sum
}

fn main() {
    println!("part 1: {}", rucksack_priorities());
    println!("part 2: {}", team_rucksack_priorities());
}
