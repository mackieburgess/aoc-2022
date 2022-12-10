fn tube_values() -> isize {
    let instructions = include_str!("../data/10.input");
    let mut x_values: Vec<isize> = vec![];
    let mut x: isize = 1;

    for line in instructions.lines() {
        match line.split(' ').collect::<Vec<&str>>().get(1) {
            None => x_values.push(x),
            Some(new_value) => {
                if let Some(new_value) = new_value.parse::<isize>().ok() {
                    x_values.push(x);
                    x_values.push(x);
                    x += new_value;
                }
            }
        }
    }

    let mut answer = 0;

    for cycle in [20, 60, 100, 140, 180, 220] {
        answer += x_values.clone()[cycle-1] * cycle as isize;
    }

    answer
}

fn main() {
    println!("part one: {}", tube_values());
}
