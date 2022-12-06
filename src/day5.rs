use std::collections::VecDeque;

fn generate_actions() ->
    (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>)
{
    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::from([]); 10];

    let input = include_str!("../data/5.input")
        .split_once("\n\n");

    if let Some((crate_cfg, actions)) = input {
        for line in crate_cfg.lines() {
            for (idx, char) in line.chars().enumerate() {
                if char.is_alphabetic() {
                    // euclidian division always gives an integer output
                    crates[idx.div_euclid(4)].push_front(char)
                }
            }
        }


        let actions: Vec<(usize, usize, usize)> = actions.split('\n').filter_map(|action| {
            // parse all integers from each line
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

        return (crates, actions);
    }

    (crates, vec![])

}

fn crates_configuration() -> String {
    let (mut crates, actions) = generate_actions();

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


fn fancy_crates_configuration() -> String {
    let (mut crates, actions) = generate_actions();

    for (quantity, start, end) in actions {
        if start < 10 && end < 10 {
            // setup to additional crate
            while let Some(char) = crates[end-1].pop_back() {
                crates[9].push_front(char);
            }

            for _ in 0..quantity {
                if let Some(char) = crates[start-1].pop_back() {
                    crates[end-1].push_front(char);
                }
            }

            // teardown from additional crate
            while let Some(char) = crates[9].pop_back() {
                crates[end-1].push_front(char);
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


fn main() {
    println!("part 1: {}", crates_configuration());
    println!("part 2: {}", fancy_crates_configuration());
}
