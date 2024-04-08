use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use host::execute;

fn config() -> Criterion {
    Criterion::default().measurement_time(Duration::new(10, 0))
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sensor read", |b| b.iter(|| execute()));
}

criterion_group!{name = benches; config = config(); targets = criterion_benchmark}
criterion_main!(benches);
