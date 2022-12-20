fn sum_coords() -> isize {
    // instead of actually moving values around a ring buffer,
    // just use a vec and calculate the positions
    let mut ring: Vec<(usize, isize)> = Vec::new();

    include_str!("../data/20.input")
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            if let Some(line) = line.parse::<isize>().ok() {
                ring.push((idx, line));
            }
        });


    // reposition around the ring buffer
    // lots of typecasting and off-by-one checking required
    for x in 0..ring.len() {
        for y in 0..ring.len() {
            if ring[y].0 == x {
                let mut new_position = (y as isize) + ring[y].1;

                while new_position < 0 {
                    new_position += ring.len() as isize - 1;
                }

                new_position = new_position.rem_euclid((ring.len() as isize) - 1);

                let value = ring.remove(y);

                if new_position == 0 {
                    ring.push(value);
                } else {
                    ring.insert(new_position as usize, value);
                }

                break
            }
        }
    }

    // there is one zero position, from which we can derive coordinates from
    let mut zero_position = 0;

    // find the position of 0
    for x in 0..ring.len() {
        if ring[x].1 == 0 {
            zero_position = x;

            break
        }
    }

    let mut coords = [0,0,0];

    // get the offset from the zero position around the ring buffer
    for (idx, offset) in [1000, 2000, 3000].iter().enumerate() {
        coords[idx] = ring[(zero_position + offset).rem_euclid(ring.len())].1;
    }

    return coords[0] + coords[1] + coords[2];
}


fn main() {
    println!("part one: {}", sum_coords());
}
