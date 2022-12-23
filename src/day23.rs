// TODO: it'd be nice to do a rewrite here
// there's likely a lot of potential for code deduplication

use std::collections::{VecDeque,HashSet};

fn get_map() -> VecDeque<VecDeque<bool>> {
    include_str!("../data/23.input")
        .lines()
        .map(|line| {
            line.chars().map(|char| {
                match char {
                    '#' => true,
                    _ => false
                }
            }).collect()
        }).collect()
}

fn is_clear(map: &VecDeque<VecDeque<bool>>, point: (usize, usize), round: usize) -> (usize, usize) {
    let mut lonely: u8 = 0;

    // bitmask
    if !map[point.1-1][point.0-1] { lonely +=   1 } // NW
    if !map[point.1-1][point.0]   { lonely +=   2 } // N
    if !map[point.1-1][point.0+1] { lonely +=   4 } // NE
    if !map[point.1][point.0+1]   { lonely +=   8 } // E
    if !map[point.1+1][point.0+1] { lonely +=  16 } // SE
    if !map[point.1+1][point.0]   { lonely +=  32 } // S
    if !map[point.1+1][point.0-1] { lonely +=  64 } // SW
    if !map[point.1][point.0-1]   { lonely += 128 } // W

    // lazy hardcoding

    match round % 4 {
        0 => match lonely {
            255 => (point.0, point.1),
            value if value &   7 ==   7 => (point.0, point.1-1), // north
            value if value & 112 == 112 => (point.0, point.1+1), // south
            value if value & 193 == 193 => (point.0-1, point.1), // west
            value if value &  28 ==  28 => (point.0+1, point.1), // east
            _ => (point.0, point.1)
        },
        1 => match lonely {
            255 => (point.0, point.1),
            value if value & 112 == 112 => (point.0, point.1+1), // south
            value if value & 193 == 193 => (point.0-1, point.1), // west
            value if value &  28 ==  28 => (point.0+1, point.1), // east
            value if value &   7 ==   7 => (point.0, point.1-1), // north
            _ => (point.0, point.1)
        },
        2 => match lonely {
            255 => (point.0, point.1),
            value if value & 193 == 193 => (point.0-1, point.1), // west
            value if value &  28 ==  28 => (point.0+1, point.1), // east
            value if value &   7 ==   7 => (point.0, point.1-1), // north
            value if value & 112 == 112 => (point.0, point.1+1), // south
            _ => (point.0, point.1)
        },
        3 => match lonely {
            255 => (point.0, point.1),
            value if value &  28 ==  28 => (point.0+1, point.1), // east
            value if value &   7 ==   7 => (point.0, point.1-1), // north
            value if value & 112 == 112 => (point.0, point.1+1), // south
            value if value & 193 == 193 => (point.0-1, point.1), // west
            _ => (point.0, point.1)
        },
        _ => unreachable!()
    }
}

fn trim_map(mut map: VecDeque<VecDeque<bool>>) -> VecDeque<VecDeque<bool>> {
    let (mut top, mut bottom, mut left, mut right) = (false, false, false, false);
    // top
    while !top {
        for item in map[0].iter() {
            if *item { top = true }
        }

        if !top { drop(map.pop_front()) }
    }

    // bottom
    while !bottom {
        for item in map[map.len() - 1].iter() {
            if *item { bottom = true }
        }

        if !bottom { drop(map.pop_back()) }
    }

    // left
    while !left {
        for row in map.iter() {
            if row[0] { left = true }
        }

        if !left {
            for row in map.iter_mut() { drop(row.pop_front()) }
        }
    }

    // left
    while !right {
        for row in map.iter() {
            if row[row.len() - 1] { right = true }
        }

        if !right {
            for row in map.iter_mut() { drop(row.pop_back()) }
        }
    }

    return map;
}

fn loose_elves() -> usize {
    let mut map = get_map();

    for round in 0..10 {
        // trim and then expand the map
        map = trim_map(map);

        map.push_front(VecDeque::new());
        map.push_back(VecDeque::new());

        for _ in 0..map[1].len() {
            map.front_mut().expect("shh").push_back(false);
            map.back_mut().expect("shh").push_back(false);
        }

        for row in map.iter_mut() {
            row.push_back(false);
            row.push_front(false);
        }

        let mut visited1: HashSet<(usize, usize)> = HashSet::new();
        let mut visited2: HashSet<(usize, usize)> = HashSet::new();

        // part one: monkey planning
        let mut plans: Vec<Vec<Option<(usize, usize)>>> = vec![];

        for y in 0..map.len() {
            plans.push(vec![]);

            for x in 0..map[y].len() {
                if map[y][x] {
                    let new_zone = is_clear(&map, (x, y), round);
                    plans.last_mut().expect("shh").push(Some(new_zone));

                    // check whether multiple elves are going to the same place
                    if visited1.contains(&new_zone) {
                        visited2.insert(new_zone);
                    }

                    visited1.insert(new_zone);
                } else {
                    plans.last_mut().expect("shh").push(None);
                }
            }
        }

        // part two: monkey moving
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if let Some(plan) = plans[y][x] {
                    // ensure no two plans go to the same place
                    if !visited2.contains(&plan) {
                        map[y][x] = false;
                        map[plan.1][plan.0] = true;
                    }
                }
            }
        }
    }

    // monkey counting
    trim_map(map)
        .iter()
        .map(|value| {
            value.iter().map(|v| {
                if !v { 1 } else { 0 }
            }).sum::<usize>()
        }).sum()
}

fn loosened_elves() -> usize {
    // basically just loose_elves() but exhaustive
    let mut map = get_map();

    for round in 0.. {
        let mut moved = false;

        map = trim_map(map);

        map.push_front(VecDeque::new());
        map.push_back(VecDeque::new());

        for _ in 0..map[1].len() {
            map.front_mut().expect("shh").push_back(false);
            map.back_mut().expect("shh").push_back(false);
        }

        for row in map.iter_mut() {
            row.push_back(false);
            row.push_front(false);
        }

        let mut visited1: HashSet<(usize, usize)> = HashSet::new();
        let mut visited2: HashSet<(usize, usize)> = HashSet::new();

        // part one: monkey planning
        let mut plans: Vec<Vec<Option<(usize, usize)>>> = vec![];

        for y in 0..map.len() {
            plans.push(vec![]);

            for x in 0..map[y].len() {
                if map[y][x] {
                    let new_zone = is_clear(&map, (x, y), round);
                    plans.last_mut().expect("shh").push(Some(new_zone));

                    // check whether multiple elves are going to the same place
                    if visited1.contains(&new_zone) {
                        visited2.insert(new_zone);
                    }

                    visited1.insert(new_zone);
                } else {
                    plans.last_mut().expect("shh").push(None);
                }
            }
        }

        // part two: monkey moving
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if let Some(plan) = plans[y][x] {
                    // ensure no two plans go to the same place
                    if !visited2.contains(&plan) {
                        map[y][x] = false;
                        map[plan.1][plan.0] = true;
                        if (plan.0, plan.1) != (x, y) {
                            moved = true;
                        }
                    }
                }
            }
        }

        if !moved { return round + 1; }
    }

    return 0;
}

fn main() {
    println!("part one: {}", loose_elves());
    println!("part two: {}", loosened_elves());
}
