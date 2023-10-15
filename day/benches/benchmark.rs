use criterion::{black_box, criterion_group, criterion_main, Criterion};
use {{crate_name}}::{solve_part1, solve_part2, EXAMPLE, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve part1 example", |b| {
        b.iter(|| solve_part1(black_box(EXAMPLE)));
    });
    c.bench_function("solve part1 puzzle", |b| {
        b.iter(|| solve_part1(black_box(INPUT)));
    });
    c.bench_function("solve part2 example", |b| {
        b.iter(|| solve_part2(black_box(EXAMPLE)));
    });
    c.bench_function("solve part2 puzzle", |b| {
        b.iter(|| solve_part2(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
