
fn beset_sections() -> usize {
    include_str!("../data/4.input")
        .split('\n')
        .map(|pair| {
            let routines: Vec<(usize, usize)> = pair.split(',').filter_map(|routine| {
                // parse the routine into the starting and ending segments
                let segments = routine
                    .split('-')
                    .filter_map(|segment| segment.parse().ok()).collect::<Vec<usize>>();

                // if there are enough segments, include the routine
                if segments.len() == 2 {
                    Some((segments[0],segments[1]))
                } else {
                    None
                }
            }).collect();

            if routines.len() == 2 {
                // ensure that one routine completely enthralls the other
                if routines[0].0 <= routines[1].0 && routines[0].1 >= routines[1].1
                    || routines[0].0 >= routines[1].0 && routines[0].1 <= routines[1].1
                {
                    return 1;
                }
            }

            0
        }).sum()
}

fn connected_sections() -> usize {
    include_str!("../data/4.input")
        .split('\n')
        .map(|pair| {
            let routines: Vec<(usize, usize)> = pair.split(',').filter_map(|routine| {
                // parse the routine into the starting and ending segments
                let segments = routine
                    .split('-')
                    .filter_map(|segment| segment.parse().ok()).collect::<Vec<usize>>();

                // if there are enough segments, include the routine
                if segments.len() == 2 {
                    Some((segments[0],segments[1]))
                } else {
                    None
                }
            }).collect();

            if routines.len() == 2 {
                // ensure that one routine connects to the other
                if routines[0].0 >= routines[1].0 && routines[0].0 <= routines[1].1
                    || routines[0].1 >= routines[1].0 && routines[0].1 <= routines[1].1
                    || routines[1].0 >= routines[0].0 && routines[1].0 <= routines[0].1
                    || routines[1].1 >= routines[0].0 && routines[1].1 <= routines[0].1
                {
                    return 1;
                }
            }

            0
        }).sum()
}

fn main() {
    println!("part one: {}", beset_sections());
    println!("part two: {}", connected_sections());
}
