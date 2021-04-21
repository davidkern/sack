use criterion::{BatchSize, black_box, criterion_group, criterion_main, Criterion};
use sack::Sack;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("allocate", move |b| {
        b.iter_batched(|| Sack::new(), |sack| drop(black_box(sack.alloc(0))), BatchSize::SmallInput)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
