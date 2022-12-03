fn most_plentiful_elves(amount: usize) -> usize {
    let mut elves: Vec<usize> = include_str!("../data/1.input")
        .split("\n\n")
        .map(|bundle| {
            bundle
                .split("\n")
                // filter by values that can be usize
                .filter_map(|val| val.parse::<usize>().ok())
                .sum()
        }).collect();

    elves.sort_by(|a,b| b.cmp(a));

    elves.iter().take(amount).sum()
}

fn main() {
    println!("part one: {}", most_plentiful_elves(1));
    println!("part two: {}", most_plentiful_elves(3));
}
