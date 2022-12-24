use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction { North, East, South, West }

fn pass_time(map: &Vec<Vec<bool>>, old_hurricanes: Vec<((usize, usize), Direction)>, time: usize) -> Vec<((usize, usize), Direction)> {
    let mut hurricanes: Vec<((usize, usize), Direction)> = vec![];

    // move hurricanes, wrapping on the map edge
    // now with new and improved maths™
    for ((x, y), direction) in old_hurricanes.iter() {
        match direction {
            Direction::North => {
                let new_y = (y + (map.len()-2) - (time % (map.len()-2))) % (map.len()-2);

                hurricanes.push(((*x, new_y), *direction));
            },
            Direction::East => {
                let new_x = ((x + ((time-1) % (map[*y].len()-2))) % (map[*y].len()-2)) + 1;

                hurricanes.push(((new_x, *y), *direction));
            },
            Direction::South => {
                let new_y = ((y + ((time-1) % (map.len()-2))) % (map.len()-2)) + 1;

                hurricanes.push(((*x, new_y), *direction));
            },
            Direction::West => {
                let new_x = (x + (map[*y].len()-2) - (time % (map[*y].len()-2))) % (map[*y].len()-2);

                hurricanes.push(((new_x, *y), *direction));
            }
        }
    }

    return hurricanes;
}

fn manhattan(cur: (usize, usize), dest: (usize, usize)) -> usize {
    // compares the second last element location to the last element location
    return cur.0.abs_diff(dest.0) + cur.1.abs_diff(dest.1);
}

fn h_contains(hurricanes: &Vec<((usize, usize), Direction)>, cur: (usize, usize)) -> bool {
    // checks a square for hurricanes
    for (hur, _) in hurricanes.iter() {
        if *hur == cur { return true; }
    }

    return false;
}

fn fewest_steps_required(start_time: usize, forwards: bool) -> usize {
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

    let start_point = match forwards {
        true => (1, 0),
        false => (map[map.len()-1].len()-2, map.len()-1)
    };

    let destination = match forwards {
        true => (map[map.len()-1].len() - 2, map.len() - 1),
        false => (1, 0)
    };

    let mut agenda: Vec<((usize, usize), (usize, usize))> = Vec::from([(start_point, (start_time, manhattan((1, 0), destination)))]);
    let mut visited: HashSet<((usize, usize), (usize, usize))> = HashSet::new();


    while agenda.len() != 0 {
        // sort by distance + heuristic, descending
        agenda.sort_by(|a, b| (b.1.0 + b.1.1).cmp(&(&a.1.0 + &a.1.1)));

        // take the item with the lowest distance
        if let Some(((x, y), (time, h))) = agenda.pop() {
            // if the end is found
            if y == destination.1 {
                return time;
            }

            visited.insert(((x, y), (time, h)));

            let mut round_hurricanes = hurricanes.clone();

            // hurricane simulation
            round_hurricanes = pass_time(&map, round_hurricanes, time+1);

            // up
            if y != 0 && map[y-1][x] && !h_contains(&round_hurricanes, (x, y-1)) {
                let to_add = ((x, y-1), (time+1, manhattan((x, y-1), destination)));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // right
            if map[y][x+1] && !h_contains(&round_hurricanes, (x+1, y)) {
                let to_add = ((x+1, y), (time+1, manhattan((x+1, y), destination)));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // down
            if y != map.len()-1 && map[y+1][x] && !h_contains(&round_hurricanes, (x, y+1)) {
                let to_add = ((x, y+1), (time+1, manhattan((x, y+1), destination)));

                if !visited.contains(&to_add) && !agenda.contains(&to_add) {
                    agenda.push(to_add);
                }
            }

            // left
            if map[y][x-1] && !h_contains(&round_hurricanes, (x-1, y)) {
                let to_add = ((x-1, y), (time+1, manhattan((x-1, y), destination)));

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

fn one_way_trip() -> usize {
    return fewest_steps_required(0, true);
}

fn round_trip() -> usize {
    return fewest_steps_required(
        fewest_steps_required(
            fewest_steps_required(
                0,
                true
            ), false
        ), true
    );
}

fn main() {
    // today's solution takes absolutely _forever_ :/
    // A* on a big graph with a questionable heuristic is just really slow
    println!("part one: {}", one_way_trip());
    println!("part two: {}", round_trip())
}
