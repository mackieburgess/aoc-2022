use std::collections::VecDeque;

fn crate_configurations() -> String {
    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::from([]); 9];

    let input = include_str!("../data/5.input")
        .split_once("\n\n");

    if let Some((crate_cfg, actions)) = input {
        for line in crate_cfg.lines() {
            for (idx, char) in line.chars().enumerate() {
                if char.is_alphabetic() {
                    crates[idx.div_euclid(4)].push_front(char)
                }
            }
        }


        let actions: Vec<(usize, usize, usize)> = actions.split('\n').filter_map(|action| {
            let numbers: Vec<usize> = action
                .split(' ')
                .filter_map(|word| word.parse::<usize>().ok())
                .collect();

            if numbers.len() == 3 {
                Some((numbers[0], numbers[1], numbers[2]))
            } else {
                None
            }
        }).collect();

        for (quantity, start, end) in actions {
            if start < 10 && end < 10 {
                for _ in 0..quantity {
                    if let Some(char) = crates[start-1].pop_back() {
                        crates[end-1].push_back(char);
                    }
                }
            }
        }

        let mut output = "".to_string();

        for mut crate_ in crates {
            if let Some(letter) = crate_.pop_back() {
                output.push(letter)
            }
        }

        return output;
    }

    "".to_string()
}

fn main() {
    println!("part 1: {}", crate_configurations());
}
