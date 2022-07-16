use criterion::{criterion_group, criterion_main, Criterion, black_box};
use ultra_nlp::{daachorse, cedarwood, hashmap, BehaviorForUnmatched};

criterion_group!(benches, bench_segment_backward_longest);
criterion_main!(benches);

fn bench_segment_backward_longest(c: &mut Criterion) {
    let mut group = c.benchmark_group(
      "segment_backward_longest"
    );

    let patterns: Vec<&str> = vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"];
    let text = " 南京市长江大桥, hello world ";

    group.bench_function("daachorse", |b| {
        let dict = daachorse::BackwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            daachorse::segment_backward_longest(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            )
        });
    });

    group.bench_function("cedarwood", |b| {
        let dict = cedarwood::BackwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            cedarwood::segment_backward_longest(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            );
        });
    });

    group.bench_function("hashmap", |b| {
        let dict = hashmap::BackwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            hashmap::segment_backward_longest(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            );
        });
    });

    group.finish();
}
