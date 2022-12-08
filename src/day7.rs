use std::collections::{HashMap, HashSet};

const TOTAL_AVAILABLE_SPACE: isize = 70_000_000;
const SPACE_REQUIRED: isize = 30_000_000;

struct Contents {
    size: usize,
    current_dir: Vec<&'static str>,
    dirs: HashMap<String, HashMap<&'static str, usize>>
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
        Some((_, "..")) => {
            // bad pattern, fix
            let _x = contents.current_dir.pop();

            ()
        },
        Some((_, "/")) => contents.current_dir = vec!["/"],
        Some((_, folder)) => contents.current_dir.push(folder),
        _ => ()
    }

    contents
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
        for (idx, _) in contents.current_dir.iter().enumerate() {
            // build up directory
            let dir = [contents.current_dir[1..=idx].join("/")].join("/");

            // only insert if the directory isn't tracked, or the file isn't tracked in the
            // directory
            contents.dirs
                .entry(dir)
                .and_modify(|map| { map.entry(file).or_insert(size); })
                .or_insert(
                    HashMap::from([(file, size)])
                );
        }
    };

    contents
}

fn process_file_structure() -> Contents {
    let input = include_str!("../data/7.input");

    let mut input_set: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        input_set.insert(line);
    }

    // bad code
    let size = input_set.iter().filter_map(|line| {
        if let Some((value, _)) = line.split_once(' ') {
            value.parse::<usize>().ok()
        } else {
            None
        }
    }).sum();

    let mut file_structure: Contents = Contents {
        size,
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

    file_structure
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
        .map(|dir| dir.into_values().sum::<usize>())
        .collect();

    totals.sort();

    let space_used = TOTAL_AVAILABLE_SPACE - (TOTAL_AVAILABLE_SPACE - file_structure.size as isize);

    let cleanup_required = space_used - SPACE_REQUIRED;

    dbg!(&totals);
    dbg!(file_structure.size);

    for total in totals {
        if total as isize >= cleanup_required {
            return total;
        }
    }

    0
}

fn main() {
    println!("part one: {}", small_folders_size());
    println!("part two - incomplete: {}", smallest_viable_folder());
}
