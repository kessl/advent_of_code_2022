#![feature(test)]
extern crate test;
mod jobs;

fn main() {
    run("day02");
}

fn run(name: &str) {
    println!("Running {name}\n-------------");
    (jobs::all()
        .iter()
        .find(|job| job.name == name)
        .expect(&format!("unknown job '{name}'"))
        .proc)()
}

#[allow(dead_code)]
fn run_seq() {
    jobs::all().iter().for_each(|job| {
        println!("Running {}\n-------------", job.name);
        (job.proc)();
        println!();
    })
}

#[bench]
fn bench_all_seq(b: &mut test::Bencher) {
    b.iter(|| run_seq());
}
