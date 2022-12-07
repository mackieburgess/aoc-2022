fn is_lonely(
    input: &str,
    backwards: usize,
    forwards: usize,
    idx: usize
) -> bool {

    let mut found = false;

    // check backwards
    for i in 1..=backwards {
        // ensure idx is far enough from 0
        if (idx as isize) - (i as isize) >= 0 {
            if input.chars().nth(idx-i) == input.chars().nth(idx) {
                found = true;
            }
        } else {
            // the value is too close to the start to be true
            return false;
        }
    }

    // check forwards
    for i in 1..=forwards {
        if input.chars().nth(idx+i) == input.chars().nth(idx) {
            found = true;
        }
    }

    if !found {
        if backwards == 0 {
            return true;
        } else {
            return is_lonely(input, backwards-1, forwards+1, idx-1);
        }
    } else {
        return false;
    }
}

fn first_lonely_key() -> usize {
    let input = include_str!("../data/6.input").to_string();

    for idx in 0..input.len() {
        if is_lonely(&input, 3, 0, idx) {
            return idx+1;
        }
    }

    return 0;
}

fn main() {
    println!("part one: {}", first_lonely_key());
}
