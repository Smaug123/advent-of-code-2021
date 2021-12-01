use criterion::{criterion_group, criterion_main, Criterion};
use day_1::day_1::{input, part_1, part_2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = input();
    c.bench_function("day 1 part 1", |b| {
        b.iter(|| {
            part_1(&input);
        })
    });
    c.bench_function("day 1 part 2", |b| {
        b.iter(|| {
            part_2(&input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
