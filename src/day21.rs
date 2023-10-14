use std::collections::HashMap;

#[derive(Clone)]
enum Value {
    Op((String, char, String)),
    Number(usize)
}

fn calc(left: isize, op: char, right: isize) -> isize {
    return match op {
        '+' => left + right,
        '*' => left * right,
        '-' => left - right,
        '/' if right != 0 && left.rem_euclid(right) == 0 => left / right,
        _ => { println!("shite"); 0 }
    }
}

fn get_value(monkeys: &HashMap<&str, Value>, monkey: &str) -> usize {
    return match monkeys.get(monkey) {
        Some(Value::Op(x)) => match x {
            (left, op, right) => calc(get_value(monkeys, &left) as isize, *op, get_value(monkeys, &right) as isize) as usize
        },
        Some(Value::Number(num)) => *num,
        _ => 0
    };
}

fn contains_humn(monkeys: &HashMap<&str, Value>, monkey: &str) -> bool {
    if monkey == "humn" {
        return true;
    } else {
        return match monkeys.get(monkey) {
            Some(Value::Op((left, _, right))) => contains_humn(monkeys, left) || contains_humn(monkeys, right),
            _ => false

        }
    }

}

fn get_calls() -> HashMap<&'static str, Value> {
    include_str!("../data/21.input")
        .lines()
        .filter_map(|line| {
            if let Some((name, etc)) = line.split_once(": ") {
                let etc = etc.split(" ").collect::<Vec<&str>>();

                if etc.len() == 1 {
                    // monkey has a value
                    if let Some(number) = etc[0].parse::<usize>().ok() {
                        return Some((
                            name,
                            Value::Number(number)
                        ));
                    }
                } else if etc.len() == 3 {
                    // monkey has an operation
                    if let Some(op) = etc[1].chars().nth(0) {
                        return Some((
                            name,
                            Value::Op((
                                etc[0].to_string(),
                                op,
                                etc[2].to_string()
                            ))
                        ))
                    }
                }
            }

            return None;
        }).collect::<HashMap<&str, Value>>()
}

fn monkey_calc() -> usize {
    let monkey_calls = get_calls();

    return get_value(&monkey_calls, "root");
}

fn humn_calc(
    monkeys: &HashMap<&str, Value>,
    node: &str,
    mut stack: Vec<(isize, char, bool)>,
    initial_value: isize
) -> isize {
    // Returns the value `humn` needs to be set to, such that lhs and rhs of root are equal.
    //
    // The way to solve this is to find `humn`, then walk upwards, inverting each nodes onto the
    // result of the rhs.
    //
    // a / ((humn + b) * c) == d / e
    // => (humn + b) * c    == a / (d / e)
    // => humn + b          == (a / (d / e)) / c
    // => humn              == ((a / (d / e)) / c) - b

    if node == "humn" {
        let mut output = initial_value;

        stack.iter().for_each(|(val, op, val_rhs)| {
            if *val_rhs {
                output = calc(output, *op, *val)
            } else {
                output = calc(*val, *op, output)
            }
        });

        return output;
    }

    if let Some(Value::Op((left, op, right))) = monkeys.get(node) {
        if contains_humn(&monkeys, left) {
            let new_op = match op {
                '+' => '-',
                '-' => '+',
                '*' => '/',
                '/' => '*',
                _ => panic!("Invalid operator")
            };

            // LHS contains humn
            stack.push((get_value(monkeys, right) as isize, new_op, true));
            return humn_calc(monkeys, left, stack, initial_value);
        } else {
            let (new_op, accum_on_left) = match op {
                '+' => ('-', true),
                '-' => ('-', false),
                '*' => ('/', true),
                '/' => ('/', false),
                _ => panic!("Invalid operator")
            };

            // RHS contains humn
            stack.push((get_value(monkeys, left) as isize, new_op, accum_on_left));
            return humn_calc(monkeys, right, stack, initial_value);
        }
    } else {
        panic!("Impossible monkey.");
    }
}

fn get_humn_value(monkeys: &HashMap<&str, Value>) -> isize {
    // Initial op doesn't matter, we know it mean equality.
    if let Some(Value::Op((left, _, right))) = monkeys.get("root") {
        if contains_humn(&monkeys, left) {
            let initial_value = get_value(monkeys, right);
            return humn_calc(monkeys, left, vec![], initial_value as isize)
        } else {
            let initial_value = get_value(monkeys, left);
            return humn_calc(monkeys, right, vec![], initial_value as isize)
        }
    } else {
        panic!("No root monkey.")
    }
}

fn main() {
    let monkeys = get_calls();

    println!("part one: {}", monkey_calc());
    println!("part two: {}", get_humn_value(&monkeys));
}
