use std::collections::VecDeque;


#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    true_result: usize,
    false_result: usize,
    inspected: usize,
}

#[derive(Clone, Copy)]
struct Op {
    lhs: usize,
    op: char,
    rhs: usize,
}

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Fancy
}

impl Monkey {
    fn build(monkey: &str) -> Self {

        let monkey: Vec<&str> = monkey.lines().collect();

        let items = match monkey[1].split_once(": ") {
            Some((_, list)) => list.split(", ").filter_map(|item| item.parse::<usize>().ok()).collect(),
            None => VecDeque::new()
        };

        let operation: Vec<&str> = monkey[2].split_whitespace().collect();

        // 0 will be replaced with `old`
        let operation = Op {
            lhs: match operation[3].parse::<usize>().ok() {
                Some(integer) => integer,
                None => 0
            },
            op: match operation[4] {
                "+" => '+',
                "*" => '*',
                "-" => '-',
                "/" => '/',
                _ => '?'
            },
            rhs: match operation[5].parse::<usize>().ok() {
                Some(integer) => integer,
                None => 0
            }
        };

        let test: usize = match monkey[3].split_whitespace()
            .collect::<Vec<&str>>()[3]
            .parse::<usize>()
            .ok()
        {
            Some(int) => int,
            None => 1
        };

        let true_result: usize = match monkey[4].split_whitespace()
            .collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .ok()
        {
            Some(int) => int,
            None => 0
        };

        let false_result: usize = match monkey[5].split_whitespace()
            .collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .ok()
        {
            Some(int) => int,
            None => 0
        };

        Monkey {
            items,
            operation,
            test,
            true_result,
            false_result,
            inspected: 0
        }
    }

    // fn print(&self) {
    //     println!("items: {:?}", self.items);
    //     println!("operation: {} {} {}", self.operation.lhs, self.operation.op, self.operation.rhs);
    //     println!("test: {}", self.test);
    //     println!("if true, pass to monkey {}", self.true_result);
    //     println!("if false, pass to monkey {}", self.false_result);
    //     println!();
    // }
}

fn monkey_round(mut monkeys: Vec<Monkey>, mode: Mode) -> Vec<Monkey> {
    let common_multiple: usize = monkeys.clone().iter().map(|monkey| monkey.test).product();

    for idx in 0..monkeys.len() {
        // perform the worry operation on the monkey
        monkeys[idx].items = monkeys[idx].items.iter().map(|item| {
            let lhs: usize;
            let rhs: usize;

            if monkeys[idx].operation.lhs != 0 {
                lhs = monkeys[idx].operation.lhs;
            } else {
                lhs = *item;
            }

            if monkeys[idx].operation.rhs != 0 {
                rhs = monkeys[idx].operation.rhs;
            } else {
                rhs = *item;
            }

            if let Mode::Normal = mode {
                match monkeys[idx].operation.op {
                    '+' => (lhs + rhs).div_euclid(3),
                    '*' => (lhs * rhs).div_euclid(3),
                    _ => *item
                }
            } else {
                match monkeys[idx].operation.op {
                    '+' => (lhs + rhs).rem_euclid(common_multiple),
                    '*' => (lhs * rhs).rem_euclid(common_multiple),
                    _ => *item
                }
            }
        }).collect();

        monkeys[idx].inspected += monkeys[idx].items.len();

        // for len
        for _ in 0..monkeys[idx].items.len() {
            let move_location;

            if monkeys[idx].items[0] % monkeys[idx].test == 0 {
                move_location = monkeys[idx].true_result;
            } else {
                move_location = monkeys[idx].false_result;
            }

            if let Some(moving_item) = monkeys[idx].items.pop_front() {
                // have to assign to avoid immutable + mutable borrow
                monkeys[move_location].items.push_back(moving_item);
            }
        }

    }

    monkeys
}

fn monkey_business_level(rounds: usize, mode: Mode) -> usize {
    let mut monkeys = include_str!("../data/11.input")
        .split("\n\n")
        .map(|monkey| Monkey::build(monkey))
        .collect::<Vec<Monkey>>();

    for _ in 0..rounds {
        monkeys = monkey_round(monkeys, mode);
    }

    monkeys
        .iter()
        .enumerate()
        .for_each(|(idx, monkey)|
            drop(println!("monkey {} inspected {} items", idx, monkey.inspected))
        );

    let mut inspected: Vec<usize> = monkeys.iter()
        .map(|monkey| monkey.inspected)
        .collect();

    // I could go O(n) instead of O(n log n) but I'm being lazy
    inspected.sort_by(|a, b| b.cmp(a));

    if let Some(a) = inspected.get(0) {
        if let Some(b) = inspected.get(1) {
            return a * b;
        }
    }

    0
}

fn main() {
    println!("part one: {}", monkey_business_level(20, Mode::Normal));
    println!("part two: {}", monkey_business_level(10000, Mode::Fancy));
}
