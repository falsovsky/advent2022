use std::env;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let root = env::current_dir().unwrap().display().to_string();
    let path = format!("{}{}", root, "/../input/day01.txt");
    c.bench_function("day01 read_input", |b| b.iter(|| read_input(black_box(path.as_str()))));
    let input = read_input(path.as_str());
    c.bench_function("day01 solve_part1", |b| b.iter(|| solve_part1(black_box(&input))));
    c.bench_function("day01 solve_part2", |b| b.iter(|| solve_part2(black_box(&input))));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = criterion_benchmark
}
criterion_main!(benches);