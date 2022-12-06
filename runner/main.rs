#![feature(test)]

mod jobs;
mod bench;

fn main() {
    run("day06");
}

fn run(name: &str) {
    println!("Running {name}\n-------------");
    (jobs::all()
        .iter()
        .find(|job| job.name == name)
        .expect(&format!("unknown job '{name}'"))
        .proc)()
}

