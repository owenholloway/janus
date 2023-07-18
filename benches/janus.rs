mod modbus;

use criterion::{criterion_group, criterion_main, Criterion};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_coil_processing 10", |b| {
        b.iter(|| modbus::bench_coil_processing(10))
    });
    c.bench_function("bench_discrete_input_processing 10", |b| {
        b.iter(|| modbus::bench_discrete_input_processing(10))
    });

    c.bench_function("bench_coil_processing 100", |b| {
        b.iter(|| modbus::bench_coil_processing(100))
    });
    c.bench_function("bench_discrete_input_processing 100", |b| {
        b.iter(|| modbus::bench_discrete_input_processing(100))
    });

    c.bench_function("bench_coil_processing 1000", |b| {
        b.iter(|| modbus::bench_coil_processing(1000))
    });
    c.bench_function("bench_discrete_input_processing 1000", |b| {
        b.iter(|| modbus::bench_discrete_input_processing(1000))
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
