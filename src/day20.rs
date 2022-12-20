fn sum_coords(decryption_key: isize, mixings: usize) -> isize {
    // instead of actually moving values around a ring buffer,
    // just use a vec and calculate the positions
    let mut ring: Vec<(usize, isize)> = Vec::new();

    include_str!("../data/20.input")
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            if let Some(value) = line.parse::<isize>().ok() {
                // multiply each value by the encryption key
                ring.push((idx, value * decryption_key));
            }
        });

    // number of times that the values should be shuffled around
    for _ in 0..mixings {
        // reposition around the ring buffer
        // lots of typecasting and off-by-one checking required
        for x in 0..ring.len() {
            for y in 0..ring.len() {
                if ring[y].0 == x {
                    let fold_point = ring.len() as isize - 1;

                    let mut new_position = (y as isize) + ring[y].1;

                    if new_position < 0 {
                        // figure out how many times you need to fold up by the ring buffer size
                        let folds_to_do = (new_position * -1).rem_euclid(fold_point);

                        new_position += fold_point * folds_to_do;
                    }

                    new_position = new_position.rem_euclid(fold_point);

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
    println!("part one: {}", sum_coords(1, 1));
    println!("part two: {}", sum_coords(811589153, 10));
}
