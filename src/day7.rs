use std::collections::HashMap;

const TOTAL_AVAILABLE_SPACE: isize = 70_000_000;
const SPACE_REQUIRED: isize = 30_000_000;

struct Contents {
    current_dir: Vec<&'static str>,
    dirs: HashMap<String, HashMap<String, usize>>
}


fn interpret_command(mut contents: Contents, command: &'static str) -> Contents {
    // disregard ls commands
    if command == "ls" {
        return contents;
    }

    // .. moves out one layer
    // /  moves back to the outermost layer (/)
    // anything else moves into that layer
    match command.split_once(' ') {
        Some((_, "..")) => drop(contents.current_dir.pop()),
        Some((_, "/")) => contents.current_dir = vec!["/"],
        Some((_, folder)) => contents.current_dir.push(folder),
        _ => ()
    }

    if contents.current_dir.len() == 0 {
        contents.current_dir= vec!["/"];
    }

    return contents;
}


fn increment_dir_size(
    mut contents: Contents,
    size: &str,
    file: &'static str
) -> Contents {
    if size == "dir" {
        return contents;
    }

    // convert size to a number
    // if unconvertable, contents will be returned as is
    if let Some(size) = size.parse::<usize>().ok() {
        // build the full file path
        // this held me up for... 3 days
        let file_path = [contents.current_dir[..].join("/"), file.to_string()].join("/");

        for idx in 0..contents.current_dir.len() {

            // build up directory
            let dir = contents.current_dir[0..=idx].join("/");

            // add hashmap if it doesn't exist, else append if file isn't accounted for
            // drop() lets an operation happen and discards the output, leaving no return value
            contents.dirs
                .entry(dir)
                .and_modify(|map| drop(map.entry(file_path.clone()).or_insert(size)))
                .or_insert(
                    HashMap::from([(file_path.clone(), size)])
                );
        }
    };

    return contents;
}

fn process_file_structure() -> Contents {
    let input = include_str!("../data/7.input");

    let mut file_structure: Contents = Contents {
        current_dir: vec!["/"],
        dirs: HashMap::new()
    };

    for line in input.lines() {
        // commands change the current directory
        // files increment the directory size
        file_structure = match line.split_once(' ') {
            Some(("$", command)) => interpret_command(file_structure, command),
            Some((size, file)) => increment_dir_size(file_structure, size, file),
            None => file_structure,
        }
    }

    return file_structure;
}


fn small_folders_size() -> usize {
    let file_structure: Contents = process_file_structure();

    // sum each dir less than 100000 in size together
    file_structure.dirs
        .into_values()
        .filter_map(|dir| {
            let total = dir.into_values().sum::<usize>();

            if total <= 100000 {
                Some(total)
            } else {
                None
            }
        }).sum()

}

fn smallest_viable_folder() -> usize {
    let file_structure: Contents = process_file_structure();

    let mut totals: Vec<usize> = file_structure.dirs
        .into_values()
        .map(|dir| dir.values().sum::<usize>())
        .collect();

    totals.sort();

    let space_available = TOTAL_AVAILABLE_SPACE - totals[totals.len()-1] as isize;

    let cleanup_required = SPACE_REQUIRED - space_available;

    for total in totals {
        if total as isize >= cleanup_required {
            return total;
        }
    }

    unreachable!();
}

fn main() {
    println!("part one: {}", small_folders_size());
    println!("part two: {}", smallest_viable_folder());
}
