use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jikan_rs::prelude::*;
use tokio::runtime::Runtime;

fn benchmark_client_creation(c: &mut Criterion) {
    c.bench_function("client_creation", |b| {
        b.iter(|| {
            let _client = black_box(JikanClient::new());
        })
    });
}

fn benchmark_rate_limiter(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();
    let client = JikanClient::new();

    c.bench_function("rate_limiter_check", |b| {
        b.iter(|| {
            black_box(client.check_permit());
        })
    });
}

criterion_group!(benches, benchmark_client_creation, benchmark_rate_limiter);
criterion_main!(benches);
