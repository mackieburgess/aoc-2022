use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction { North, East, South, West }

fn pass_time(map: &Vec<Vec<bool>>, old_hurricanes: Vec<((usize, usize), Direction)>) -> Vec<((usize, usize), Direction)> {
    let mut hurricanes: Vec<((usize, usize), Direction)> = vec![];

    // move hurricanes, wrapping on the map edge
    for ((x, y), direction) in old_hurricanes.iter() {
        match direction {
            Direction::North => {
                match y {
                    1 => hurricanes.push(((*x, map.len() - 2), *direction)),
                    _ => hurricanes.push(((*x, y - 1), *direction))
                }
            },
            Direction::East => {
                match x {
                    v if *v == map[*y].len() - 2 => hurricanes.push(((1, *y), *direction)),
                    _ => hurricanes.push(((x + 1, *y), *direction))
                }
            },
            Direction::South => {
                match y {
                    v if *v == map.len() - 2 => hurricanes.push(((*x, 1), *direction)),
                    _ => hurricanes.push(((*x, y + 1), *direction))
                }
            },
            Direction::West => {
                match x {
                    1 => hurricanes.push(((map[*y].len() - 2, *y), *direction)),
                    _ => hurricanes.push(((x - 1, *y), *direction))
                }
            }
        }
    }

    return hurricanes;
}

fn manhattan(map: &Vec<Vec<bool>>, cur: (usize, usize)) -> usize {
    // compares the second last element location to the last element location
    return cur.0.abs_diff(map.len() - 1) + cur.1.abs_diff(map[map.len()-1].len() - 2)
}

fn h_contains(hurricanes: &Vec<((usize, usize), Direction)>, cur: (usize, usize)) -> bool {
    // checks a square for hurricanes
    for (hur, _) in hurricanes.iter() {
        if *hur == cur { return true; }
    }

    return false;
}

fn fewest_steps_required() -> usize {
    let map_data = include_str!("../data/24.input");

    // false is wall, true is floor
    let map = map_data.clone()
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => false,
                    _ => true,
                }
            }).collect()
        }).collect::<Vec<Vec<bool>>>();

    let mut hurricanes: Vec<((usize, usize), Direction)> = vec![];

    for (y, row) in map_data.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            match val {
                '^' => hurricanes.push(((x, y), Direction::North)),
                '>' => hurricanes.push(((x, y), Direction::East)),
                'v' => hurricanes.push(((x, y), Direction::South)),
                '<' => hurricanes.push(((x, y), Direction::West)),
                _ => ()
            }

        }
    }

    let mut agenda: Vec<((usize, usize), (usize, usize))> = Vec::from([((1, 0), (0, manhattan(&map, (1, 0))))]);
    let mut visited: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut counter = 0;

    while agenda.len() != 0 {
        counter += 1;
        if counter % 10_000 == 0 { println!("{counter}") }
        if counter % 100_000 == 0 { println!("so far: {} {}", agenda.len(), visited.len()) }

        // sort by distance + heuristic, descending
        agenda.sort_by(|a, b| (b.1.0 + b.1.1).cmp(&(&a.1.0 + &a.1.1)));

        // take the item with the lowest distance
        if let Some(((x, y), (time, h))) = agenda.pop() {
            // if the end is found
            if y == map.len()-1 {
                println!("after {counter} rounds");
                return time;
            }

            visited.insert(((x, y), (time, h)));

            let mut round_hurricanes = hurricanes.clone();

            // churn through the hurricane simulation
            for _ in 0..time+1 {
                round_hurricanes = pass_time(&map, round_hurricanes);
            }

            // up
            if y != 0 && map[y-1][x] && !h_contains(&round_hurricanes, (x, y-1)) {
                let to_add = ((x, y-1), (time+1, manhattan(&map, (x, y-1))));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // right
            if map[y][x+1] && !h_contains(&round_hurricanes, (x+1, y)) {
                let to_add = ((x+1, y), (time+1, manhattan(&map, (x+1, y))));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // down
            if map[y+1][x] && !h_contains(&round_hurricanes, (x, y+1)) {
                let to_add = ((x, y+1), (time+1, manhattan(&map, (x, y+1))));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // left
            if map[y][x-1] && !h_contains(&round_hurricanes, (x-1, y)) {
                let to_add = ((x-1, y), (time+1, manhattan(&map, (x-1, y))));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // wait
            if !h_contains(&round_hurricanes, (x, y)) {
                let to_add = ((x, y), (time+1, h));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }
        }
    }

    panic!("valley is unsurpassable");
}

fn main() {
    println!("part one: {}", fewest_steps_required());
}
