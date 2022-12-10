enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_line(line: &str) -> Instruction {
    if line == "noop" {
        return Instruction::Noop;
    }
    if let Some(("addx", arg)) = line.split_once(" ") {
        return Instruction::Addx(arg.parse().expect("valid argument"));
    }
    unreachable!("invalid instruction");
}

fn signal_strength(x: i32, cycle: usize) -> i32 {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => x * (cycle as i32),
        _ => 0,
    }
}

fn draw(cycle: usize, x: i32) {
    let position = (cycle - 1) % 40;
    if ((x - 1)..=(x + 1)).contains(&(position as i32)) {
        print!("#");
    } else {
        print!(".");
    }

    if cycle % 40 == 0 {
        println!();
    }
}

fn do_cycle(cycle: &mut usize, x: &mut i32, arg: i32) -> i32 {
    draw(*cycle, *x);
    *cycle += 1;
    *x += arg;
    signal_strength(*x, *cycle)
}

pub fn main() {
    let input = include_str!("./input.txt").lines().map(parse_line);

    let mut cycle = 1;
    let mut x = 1;
    let mut total_signal_strength = 0;

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                total_signal_strength += do_cycle(&mut cycle, &mut x, 0);
            },
            Instruction::Addx(arg) => {
                total_signal_strength += do_cycle(&mut cycle, &mut x, 0);
                total_signal_strength += do_cycle(&mut cycle, &mut x, arg);
            }
        }
    }

    println!("day10a: {total_signal_strength}");
}
