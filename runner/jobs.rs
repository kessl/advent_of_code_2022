pub struct Job {
    pub proc: fn(),
    pub name: &'static str,
}

pub fn all() -> &'static [Job] {
    &[
        Job { proc: day01::main, name: "day01" },
        Job { proc: day02::main, name: "day02" },
        Job { proc: day03::main, name: "day03" },
        Job { proc: day04::main, name: "day04" },
        Job { proc: day05::main, name: "day05" },
        Job { proc: day06::main, name: "day06" },
        Job { proc: day07::main, name: "day07" },
        Job { proc: day08::main, name: "day08" },
        Job { proc: day09::main, name: "day09" },
        Job { proc: day10::main, name: "day10" },
        // Job { proc: day11::main, name: "day11" },
        // Job { proc: day12::main, name: "day12" },
        // Job { proc: day13::main, name: "day13" },
        // Job { proc: day14::main, name: "day14" },
        // Job { proc: day15::main, name: "day15" },
        // Job { proc: day16::main, name: "day16" },
        // Job { proc: day17::main, name: "day17" },
        // Job { proc: day18::main, name: "day18" },
        // Job { proc: day19::main, name: "day19" },
        // Job { proc: day20::main, name: "day20" },
        // Job { proc: day21::main, name: "day21" },
        // Job { proc: day22::main, name: "day22" },
        // Job { proc: day23::main, name: "day23" },
        // Job { proc: day24::main, name: "day24" },
        // Job { proc: day25::main, name: "day25" },
    ]
}