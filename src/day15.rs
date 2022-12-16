use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize
}

impl Pos {
    fn from(x: isize, y: isize) -> Self { Pos { x, y } }

    fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
    }
}

fn beaconless(y_target: isize) -> usize {
    let data = include_str!("../data/15.input")
        .lines()
        .filter_map(|line| {
            // skim off the excess wording to make it easier to parse
            let line = line
                .replace("Sensor at x=", "")
                .replace(" closest beacon is at x=", "")
                .replace(" y=", "")
                .replace(":", ",")
                .split(",")
                .filter_map(|val| val.parse::<isize>().ok())
                .collect::<Vec<isize>>();

            if line.len() == 4 {
                Some((Pos::from(line[0], line[1]), Pos::from(line[2], line[3])))
            } else {
                None
            }

        }).collect::<Vec<(Pos, Pos)>>();


    // represent the line as a hashset
    let mut visited: HashSet<isize> = HashSet::new();

    for (sensor, beacon) in data.iter() {
        let manhattan = beacon.manhattan_distance(sensor);
        let from_line = sensor.y.abs_diff(y_target) as isize;

        if from_line <= manhattan {
            for v in (sensor.x - (manhattan - from_line))..=(sensor.x + (manhattan - from_line)) {
                if !visited.contains(&v) { visited.insert(v); }
            }

        }
    }

    for (_, beacon) in data.iter() {
        if beacon.y == y_target && visited.contains(&beacon.x) {
            visited.remove(&beacon.x);
        }
    }

    visited.len()
}

fn main() {
    println!("part one: {}", beaconless(2_000_000));
}
