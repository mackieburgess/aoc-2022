use std::collections::HashMap;

struct Contents {
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
            let dir = [contents.current_dir[0..=idx].join("/")].join("/");

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


fn file_size() -> usize {
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

    // sum each dir less than 100000 in size together
    file_structure.dirs
        .into_values()
        .filter_map(|map| {
            let total = map.into_values().sum::<usize>();

            if total <= 100000 {
                Some(total)
            } else {
                None
            }
        }).sum()

}

fn main() {
    println!("part one: {}", file_size());
}
