use std::cmp::Ordering;

// either a list of signals, or a usize, or nothing
#[derive(PartialEq, Clone)]
enum Signal {
    List(Vec<Signal>),
    Item(usize),
}

impl Signal {
    fn len(&self) -> usize {
        // delve down into the object to count all items included
        if let Signal::List(signal) = self {
            let mut counter = 0;

            for subsignal in signal.iter() {
                if let Signal::Item(_) = subsignal {
                    counter += 1;
                }

                if let Signal::List(_) = subsignal {
                    counter += subsignal.len();
                }
            }

            return counter;
        }

        return 1;
    }

    fn from(mut input: Vec<String>) -> Signal {
        let mut signal = vec![];

        for idx in 0..input.len() {
            if let Some(value) = input.get(idx) {
                // push one layer deeper
                if *value == "[" {
                    let subsignal = Signal::from(input[idx+1..].to_vec());

                    // remove all elements that were already added to the list
                    // plus one for the `]`
                    for _ in 0..subsignal.len() + 1 {
                        drop(input.remove(idx+1))
                    }

                    signal.push(subsignal)

                } else if *value == "]" {
                    // pop one layer out
                    return Signal::List(signal);
                } else {
                    if let Some(value) = value.parse::<usize>().ok() {
                        // add to the current layer
                        signal.push(Signal::Item(value))
                    }
                }
            }
        }

        return Signal::List(signal);
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Signal::Item(lhs), Signal::Item(rhs)) => lhs.partial_cmp(rhs),
            (Signal::List(_), Signal::Item(_)) => self.partial_cmp(&Signal::List(vec![other.clone()])),
            (Signal::Item(_), Signal::List(_)) => Signal::List(vec![self.clone()]).partial_cmp(other),
            (Signal::List(lhs), Signal::List(rhs)) => {
                if lhs.len() == 0 && rhs.len() == 0 {
                    return Some(Ordering::Equal);
                }

                // check each value of lhs against each value of rhs
                for (idx, l_value) in lhs.iter().enumerate() {
                    if let Some(r_value) = rhs.get(idx) {
                        if l_value < r_value {
                            return Some(Ordering::Less);
                        } else if l_value > r_value {
                            return Some(Ordering::Greater);
                        }
                    } else {
                        // rhs ran out
                        return Some(Ordering::Greater);
                    }
                }

                // lhs ran out
                return Some(Ordering::Less);
            }
        }
    }
}

fn process_input(input: &str) -> Vec<String> {
    // lots of replacing and flattening to separate out all the `[` and `]`
    let mut input = input
        .replace("[", "*[*")
        .replace("]", "*]*")
        .split(",")
        .flat_map(|item| {
            item.split("*")
                .map(|x| x.to_string())
                .filter(|x| x != "")
                .collect::<Vec<String>>()
        }).collect::<Vec<String>>();

    // remove leading `[`
    input.remove(0);

    return input;
}

fn ordered_pair_indices() -> usize {
    // get each pair as a tuple of strings
    let signals = include_str!("../data/13.input")
        .split("\n\n")
        .filter_map(|pair| {
            if let Some((lhs, rhs)) = pair.split_once('\n') {
                // build vecs
                let lhs = process_input(lhs);
                let rhs = process_input(rhs);

                Some((Signal::from(lhs), Signal::from(rhs)))
            } else {
                None
            }
        })
        .collect::<Vec<(Signal, Signal)>>();

    // count the indices (+1) of all ordered pairs
    return signals
        .iter()
        .enumerate()
        .map(|(idx, (left, right))| {
            if left < right { idx+1 } else { 0 }
        }).sum();
}

fn main() {
    println!("part one: {}", ordered_pair_indices());
}
