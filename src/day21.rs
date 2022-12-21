use std::collections::HashMap;

#[derive(Clone)]
enum Value {
    Op((String, char, String)),
    Number(usize)
}

fn get_value(monkeys: &HashMap<&str, Value>, monkey: &str) -> usize {
    return match monkeys.get(monkey) {
        Some(Value::Op(x)) => match x {
            (left, '+', right) => get_value(monkeys, &left) + get_value(monkeys, &right),
            (left, '-', right) => get_value(monkeys, &left) - get_value(monkeys, &right),
            (left, '*', right) => get_value(monkeys, &left) * get_value(monkeys, &right),
            (left, '/', right) => get_value(monkeys, &left) / get_value(monkeys, &right),
            _ => 0
        },
        Some(Value::Number(num)) => *num,
        _ => 0
    };
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

fn main() {
    println!("part one: {}", monkey_calc());
}
