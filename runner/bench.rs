extern crate test;

#[allow(dead_code)]
fn run_seq() {
    crate::jobs::all().iter().for_each(|job| {
        println!("Running {}\n-------------", job.name);
        (job.proc)();
        println!();
    })
}

#[bench]
fn bench_all_seq(b: &mut test::Bencher) {
    b.iter(|| run_seq());
}

macro_rules! bench_individual {
    ($fn_name:ident, $job_name:tt) => (
        #[bench]
        fn $fn_name(b: &mut test::Bencher) {
            b.iter(|| crate::run($job_name));
        }
    );
    ($fn_name_x:ident, $job_name_x:tt, $($fn_name_y:ident, $job_name_y:tt),+) => (
        bench_individual!($fn_name_x, $job_name_x);
        bench_individual!($($fn_name_y, $job_name_y),+);
    )
}

bench_individual!(
    day01, "day01",
    day02, "day02"
);