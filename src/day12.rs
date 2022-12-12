use std::collections::{HashSet, VecDeque};


fn build_height_map() -> Vec<Vec<usize>> {
    let map: Vec<Vec<usize>> = include_str!("../data/12.input")
        .lines()
            .map(|line| {
            line.chars().map(|char| {
                match char {
                    'S' => 0,
                    'E' => 27,
                    char if char.is_ascii() => (char as usize) - 96,
                    _ => 0
                }
            }).collect()
        }).collect();

    return map
}

fn get_lowest_point(map: &Vec<Vec<usize>>) -> (usize, usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                return (x, y);
            }
        }
    }

    return (0,0);
}

fn shortest_path() -> usize {

    let height_map = build_height_map();

    // keep track of all the 
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // ((x, y), route_length)
    let mut paths: VecDeque<((usize, usize), usize)> = VecDeque::from([
        (get_lowest_point(&height_map), 0)
    ]);

    // I'm sure people will spin up A* to solve this really neatly
    // I had an exam today, so... no ._.
    loop {
        // get the leftmost path, which will be amongst the cohort of shortest paths
        let path = paths.pop_front();

        if let Some(((x, y), route_length)) = path {
            let height = height_map[y][x];

            if height == 27 {
                return route_length
            }

            // TODO: deduplication

            // left
            if x != 0 {
                // check whether the step can be traversed
                if height_map[y][x-1] <= height
                    || height_map[y][x-1] - height == 1
                {
                    let new_path = (x-1, y);

                    // add new path if it has never been added to paths
                    if !visited.contains(&new_path) {
                        paths.push_back((new_path, route_length+1));
                    }

                    // avoid duplicates in paths waiting to be processed
                    visited.insert(new_path);
                }
            }

            // right
            if x < height_map[y].len() - 1 {
                if height_map[y][x+1] <= height
                    || height_map[y][x+1] - height == 1
                {
                    let new_path = (x+1, y);

                    if !visited.contains(&new_path) {
                        paths.push_back((new_path, route_length+1));
                    }

                    visited.insert(new_path);
                }
            }

            // up
            if y != 0 {
                if height_map[y-1][x] <= height
                    || height_map[y-1][x] - height == 1
                {
                    let new_path = (x, y-1);

                    if !visited.contains(&new_path) {
                        paths.push_back((new_path, route_length+1));
                    }

                    visited.insert(new_path);
                }
            }

            // down
            if y < height_map.len() - 1 {
                if height_map[y+1][x] <= height
                    || height_map[y+1][x] - height == 1
                {
                    let new_path = (x, y+1);

                    if !visited.contains(&new_path) {
                        paths.push_back((new_path, route_length+1));
                    }

                    visited.insert(new_path);
                }
            }
        }
    }
}

fn main() {
    println!("part one: {}", shortest_path());
}
