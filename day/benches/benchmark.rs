use criterion::{criterion_group, criterion_main, Criterion};
use {{crate_name}}::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve {{crate_name}} part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });

    c.bench_function("solve {{crate_name}} part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
