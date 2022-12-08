use std::collections::{VecDeque, HashSet};

fn visible_trees() -> usize {
    let mut found_trees: HashSet<(usize, usize)> = HashSet::new();

    let trees: Vec<Vec<usize>> = include_str!("../data/8.input")
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|char| char as usize)
                .collect::<Vec<usize>>()
        }).collect();

    // creates a 2D array with y-axis the size of tree x_axis
    let mut rotated_trees: Vec<Vec<usize>> = vec![vec![]; trees[0].len()];

    for (y_axis, tree_strip) in trees.iter().enumerate() {
        let mut largest: isize = -1;

        let mut inverse_strip: VecDeque<usize> = VecDeque::new();

        // from left
        for (x_axis, tree) in tree_strip.iter().enumerate() {
            // build up inverse tree strips
            inverse_strip.push_front(*tree);

            rotated_trees[x_axis].push(*tree);

            if *tree as isize > largest {
                largest = *tree as isize;
                found_trees.insert((x_axis, y_axis));
            }
        }

        largest = -1;

        // from right
        for (x_axis, tree) in inverse_strip.iter().enumerate() {
            if *tree as isize > largest {
                largest = *tree as isize;
                // need to invert the x_axis
                found_trees.insert((inverse_strip.len() - 1 - x_axis, y_axis));
            }

        }
    }

    // TODO: code deduplication
    for (y_axis, tree_strip) in rotated_trees.iter().enumerate() {
        let mut largest: isize = -1;

        let mut inverse_strip: VecDeque<usize> = VecDeque::new();

        // from top
        for (x_axis, tree) in tree_strip.iter().enumerate() {
            // build up inverse tree strips
            inverse_strip.push_front(*tree);

            if *tree as isize > largest {
                largest = *tree as isize;
                found_trees.insert((y_axis, x_axis));
            }
        }

        largest = -1;

        // from bottom
        for (x_axis, tree) in inverse_strip.iter().enumerate() {
            if *tree as isize > largest {
                largest = *tree as isize;
                // need to invert the y_axis
                found_trees.insert((y_axis, inverse_strip.len() - 1 - x_axis));
            }

        }
    }


    found_trees.len()
}

fn main() {
    println!("part one: {}", visible_trees());
}

