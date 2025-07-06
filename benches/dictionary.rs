use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use ultra_nlp::{cedarwood, daachorse, hashmap};

criterion_group!(benches, bench_dictionary);
criterion_main!(benches);

fn bench_dictionary(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_fully");

    let patterns: Vec<&str> = vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"];

    group.bench_function("daachorse_standard_dictionary", |b| {
        b.iter(|| {
            daachorse::StandardDictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.bench_function("daachorse_forward_dictionary", |b| {
        b.iter(|| {
            daachorse::ForwardDictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.bench_function("daachorse_backward_dictionary", |b| {
        b.iter(|| {
            daachorse::BackwardDictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.bench_function("cedarwood_forward_dictionary", |b| {
        b.iter(|| {
            cedarwood::ForwardDictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.bench_function("cedarwood_backward_dictionary", |b| {
        b.iter(|| {
            cedarwood::BackwardDictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.bench_function("hashmap_dictionary", |b| {
        b.iter(|| {
            hashmap::Dictionary::new(
                black_box(patterns.clone())
            ).unwrap();
        });
    });

    group.finish();
}
