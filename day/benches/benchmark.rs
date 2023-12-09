use bencher::setup_benches;
use criterion::{criterion_group, criterion_main, Criterion};
use {{crate_name}}::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    setup_benches("{{crate_name}}", &part1::solve, &part2::solve, INPUT, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
