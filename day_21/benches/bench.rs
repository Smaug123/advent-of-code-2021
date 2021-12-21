use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_21::day_21::{input, part_1, part_2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = input();
    c.bench_function("day 21 part 1", |b| {
        b.iter(|| {
            black_box(part_1(&input));
        })
    });
    c.bench_function("day 21 part 2", |b| {
        b.iter(|| {
            black_box(part_2(&input));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
