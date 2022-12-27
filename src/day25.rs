type Snafu = Vec<isize>;

fn from_snafu(input: Snafu) -> isize {
    let mut counter = 0;

    for (idx, digit) in input.iter().rev().enumerate() {
        counter += 5_isize.pow(idx as u32) * digit
    }

    return counter;
}

fn distance(permutation: &Vec<isize>, divisors: &Vec<isize>, target: isize) -> usize {
    // get the distance from the total number to the target value
    return permutation
        .iter()
        .zip(divisors.iter())
        .map(|(a, b)| a * b)
        .sum::<isize>()
        .abs_diff(target);
}

fn to_snafu_string(input: isize) -> String {
    let mut divisors = vec![1];

    // until divisor contains an value that's too big
    while divisors.iter().sum::<isize>() < input * 2 {
        divisors.push(divisors[divisors.len()-1] * 5);
    }

    // remove the divisor that is too big and reverse the list
    drop(divisors.pop());
    divisors = divisors.into_iter().rev().collect();

    let mut agenda: Vec<Vec<isize>> = vec![vec![-2], vec![-1], vec![0], vec![1], vec![2]];

    loop {
        // sort by which is nearest the final value
        agenda.sort_by(|a, b| distance(b, &divisors, input).cmp(&distance(a, &divisors, input)));

        if let Some(permutation) = agenda.pop() {
            if permutation.len() == divisors.len() {
                if distance(&permutation, &divisors, input) == 0 {
                    return permutation
                        .iter()
                        .filter_map(|c| {
                            match c {
                                -2 => Some('='),
                                -1 => Some('-'),
                                0 => Some('0'),
                                1 => Some('1'),
                                2 => Some('2'),
                                _ => None
                            }
                        }).collect::<String>();
                }
            } else {
                // create all new permutations of this permutation
                for value in [-2, -1, 0, 1, 2] {
                    let mut to_add = permutation.clone();
                    to_add.push(value);
                    agenda.push(to_add);
                }
            }
        }
    }
}

fn fuel_sum() -> String {
    let snafus: Vec<Snafu> = include_str!("../data/25.input")
        .lines()
        .map(|line| {
            line.chars().filter_map(|c| {
                return match c {
                    '2' => Some(2),
                    '1' => Some(1),
                    '0' => Some(0),
                    '-' => Some(-1),
                    '=' => Some(-2),
                    _ => None
                }
            }).collect()
        }).collect();

    let mut counter = 0;

    // count up through all snafu values
    for snafu in snafus.iter() {
        counter += from_snafu(snafu.clone());
    }

    // convert back into snafu format
    return to_snafu_string(counter);
}

fn main() {
    println!("part one: {}", fuel_sum());
}
