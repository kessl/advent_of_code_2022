#[derive(Debug, Clone)]
enum Operand {
    OldValue,
    Num(i64),
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Operation {
    left_op: Operand,
    operator: Operator,
    right_op: Operand,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    truthy_throw: usize,
    falsy_throw: usize,
    num_throws: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();
    let mut monkeys = vec![];

    while let Some(_) = lines.next() {
        let (_, starting_items) = lines
            .next()
            .expect("monkey block")
            .split_once(":")
            .expect("items");
        let mut items = starting_items.to_string();
        items.retain(|c| !c.is_whitespace());
        let items: Vec<_> = items
            .split(",")
            .map(|item| item.parse::<i64>().expect("numeric item"))
            .collect();

        let operation = lines.next().expect("operation");
        let (_, op) = operation.split_at(19);
        let mut tokens = op.split_whitespace();
        let left = tokens.next().expect("first operand");
        let left = match left {
            "old" => Operand::OldValue,
            _ => Operand::Num(left.parse::<i64>().expect("numeric operand")),
        };
        let op = match tokens.next().expect("operator") {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => unreachable!("invalid operator"),
        };
        let right = tokens.next().expect("second operand");
        let right = match right {
            "old" => Operand::OldValue,
            _ => Operand::Num(right.parse::<i64>().expect("numeric operand")),
        };
        let operation = Operation {
            left_op: left,
            operator: op,
            right_op: right,
        };

        let test = lines.next().expect("condition");
        let divisor = test.split_at(21).1.parse::<i64>().expect("numeric divisor");

        let truthy = lines.next().expect("truthy throw");
        let truthy_throw = truthy
            .split_at(29)
            .1
            .parse::<usize>()
            .expect("monkey number");

        let falsy = lines.next().expect("falsy throw");
        let falsy_throw = falsy
            .split_at(30)
            .1
            .parse::<usize>()
            .expect("monkey number");

        lines.next(); // line break

        let monkey = Monkey {
            items,
            operation,
            divisor,
            truthy_throw,
            falsy_throw,
            num_throws: 0,
        };
        monkeys.push(monkey);
    }

    monkeys
}

fn worry_level(old: i64, op: &Operation) -> i64 {
    let left = match op.left_op {
        Operand::OldValue => old,
        Operand::Num(val) => val,
    };
    let right = match op.right_op {
        Operand::OldValue => old,
        Operand::Num(val) => val,
    };
    match op.operator {
        Operator::Add => left + right,
        Operator::Multiply => left * right,
    }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, divide_by: i64) -> usize {
    let common_divisor_multiple = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);
    let mut thrown_items = vec![];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for item in monkey.items.iter() {
                let mut worry_level = worry_level(*item, &monkey.operation) / divide_by;
                worry_level = worry_level % common_divisor_multiple;
                if worry_level % monkey.divisor == 0 {
                    thrown_items.push((worry_level, monkey.truthy_throw));
                } else {
                    thrown_items.push((worry_level, monkey.falsy_throw));
                }
                monkey.num_throws += 1;
            }
            monkey.items.clear();
            for &(item, target) in &thrown_items {
                monkeys[target].items.push(item);
            }
            thrown_items.clear();
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.num_throws);
    let mut iter = monkeys.iter().rev();
    let monkey_business = iter.next().unwrap().num_throws * iter.next().unwrap().num_throws;
    monkey_business
}

pub fn main() {
    let input = include_str!("./input.txt");
    let monkeys = parse_input(input);

    println!("day11a: {}", simulate(monkeys.clone(), 20, 3));
    println!("day11b: {}", simulate(monkeys, 10_000, 1));
}
