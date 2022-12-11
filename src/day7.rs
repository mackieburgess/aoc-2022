use std::collections::HashMap;

const DISK_SPACE: isize = 70_000_000;
const SPACE_NEEDED: isize = 30_000_000;


struct Contents {
    current_dir: Vec<&'static str>,
    dirs: HashMap<String, HashMap<String, usize>>
}

impl Contents {
    fn build() -> Self {
        let input = include_str!("../data/7.input");

        let mut file_system = Contents {
            current_dir: vec!["/"],
            dirs: HashMap::new()
        };

        for line in input.lines() {
            // commands change the current directory
            // files increment the directory size
            match line.split_once(' ') {
                Some(("$", dir)) => file_system.cd(dir),
                Some((size, file)) => file_system.count_file(file, size),
                None => (),
            }
        }

        return file_system;
    }

    fn cd(&mut self, command: &'static str) {
        // `cd /` moves to the outermost layer (/)
        // `cd ..` moves out one layer
        // `cd dir` moves into dir
        match command.split_once(' ') {
            Some((_, "/")) => self.current_dir = vec!["/"],
            Some((_, "..")) => drop(self.current_dir.pop()),
            Some((_, dir)) => self.current_dir.push(dir),
            _ => ()
        }
    }

    fn count_file(&mut self, file: &str, size: &str) {
        if let Some(size) = size.parse::<usize>().ok() {
            // build the full file path
            // this held me up for... 3 days
            let file_path = [self.current_dir[..].join("/"), file.to_string()].join("/");

            for idx in 0..self.current_dir.len() {
                // build up directory
                let dir = self.current_dir[0..=idx].join("/");

                // add hashmap if it doesn't exist, else append if file isn't accounted for
                // drop() lets an operation happen and discards the output, leaving no return value
                self.dirs
                    .entry(dir)
                    .and_modify(|map| drop(map.entry(file_path.clone()).or_insert(size)))
                    .or_insert(
                        HashMap::from([(file_path.clone(), size)])
                    );
            }
        }
    }
}


fn small_folders_size() -> usize {
    let file_system = Contents::build();

    // sum each dir less than 100000 in size together
    file_system.dirs
        .into_values()
        .map(|dir| dir.into_values().sum::<usize>())
        .filter(|total| *total <= 100000)
        .sum()
}

fn smallest_viable_folder() -> usize {
    // collect and sort all dirs by total size, ascending
    let mut totals: Vec<usize> = Contents::build().dirs
        .into_values()
        .map(|dir| dir.values().sum::<usize>())
        .collect();

    totals.sort();

    // space required - space taken up
    let cleanup_needed = SPACE_NEEDED - (DISK_SPACE - totals[totals.len()-1] as isize);

    for total in totals {
        if total as isize >= cleanup_needed {
            return total
        }
    }

    unreachable!();
}

fn main() {
    println!("part one: {}", small_folders_size());
    println!("part two: {}", smallest_viable_folder());
}
