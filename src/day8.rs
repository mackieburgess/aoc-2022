use std::collections::{VecDeque, HashSet};

fn get_forest_data() -> Vec<Vec<usize>> {
    include_str!("../data/8.input")
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|char| char as usize)
                .collect::<Vec<usize>>()
        }).collect()
}

fn visible_trees() -> usize {
    let mut found_trees: HashSet<(usize, usize)> = HashSet::new();

    let trees = get_forest_data();

    // creates a 2D array with y-axis the size of tree x_axis
    let mut rotated_trees: Vec<Vec<usize>> = vec![vec![]; trees[0].len()];

    for (y_axis, strip) in trees.iter().enumerate() {
        let mut largest: isize = -1;

        let mut inverse_strip: VecDeque<usize> = VecDeque::new();

        // from left
        for (x_axis, tree) in strip.iter().enumerate() {
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
    for (y_axis, strip) in rotated_trees.iter().enumerate() {
        let mut largest: isize = -1;

        let mut inverse_strip: VecDeque<usize> = VecDeque::new();

        // from top
        for (x_axis, tree) in strip.iter().enumerate() {
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

fn most_scenic_tree() -> usize {
    let trees = get_forest_data();

    let mut best_tree_score = 1;

    for (x, strip) in trees.iter().enumerate() {
        for (y, tree) in strip.iter().enumerate() {
            // moves along each cardinal direction until it hits the forest wall or a larger tree
            let (mut n, mut e, mut s, mut w) = (0,0,0,0);

            // up
            while y > n {
                n += 1;

                if trees[x][y-n] >= *tree {
                    break
                }
            }

            // right
            // +1 accommodates for len offset
            while x + e + 1 < strip.len() {
                e += 1;

                if trees[x+e][y] >= *tree {
                    break
                }
            }

            // down
            // +1 accommodates for len offset
            while y + s + 1 < trees.len() {
                s += 1;

                if trees[x][y+s] >= *tree {
                    break
                }
            }

            // left
            while x > w {
                w += 1;

                if trees[x-w][y] >= *tree {
                    break
                }
            }

            let tree_score = n * e * s * w;

            if tree_score > best_tree_score {
                best_tree_score = tree_score;
            }

        }
    }

    best_tree_score
}

fn main() {
    println!("part one: {}", visible_trees());
    println!("part two: {}", most_scenic_tree());
}

