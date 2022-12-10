fn print_out(x: isize, pos: isize) {
    if pos == 0 {
        println!();
    }

    if x == pos || x-1 == pos || x+1 == pos {
        print!("#");
    } else {
        print!(" ")
    }
}

fn tube_values() -> isize {
    let instructions = include_str!("../data/10.input");
    let mut x_values: Vec<isize> = vec![];
    let mut x: isize = 1;

    for line in instructions.lines() {
        match line.split(' ').collect::<Vec<&str>>().get(1) {
            None => {
                print_out(x, (x_values.len() % 40) as isize);
                x_values.push(x);
            },
            Some(new_value) => {
                if let Some(new_value) = new_value.parse::<isize>().ok() {
                    // during cycle 1
                    print_out(x, (x_values.len() % 40) as isize);
                    x_values.push(x);
                    // during cycle 2
                    print_out(x, (x_values.len() % 40) as isize);
                    x_values.push(x);
                    // after cycle 2
                    x += new_value;
                }
            }
        }
    }

    let mut answer = 0;

    for cycle in [20, 60, 100, 140, 180, 220] {
        answer += x_values.clone()[cycle-1] * cycle as isize;
    }

    println!();

    answer
}

fn main() {
    println!("part one: {}", tube_values());
}
