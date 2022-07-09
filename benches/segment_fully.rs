use criterion::{criterion_group, criterion_main, Criterion, black_box};
use ultra_nlp::{daachorse, cedarwood, BehaviorForUnmatched};

criterion_group!(benches, bench_segment_fully);
criterion_main!(benches);

fn bench_segment_fully(c: &mut Criterion) {
    let mut group = c.benchmark_group(
        "segment_fully"
    );

    let patterns: Vec<&str> = vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"];
    let text = " 南京市长江大桥, hello world ";

    group.bench_function("daachorse", |b| {
        let dict = daachorse::StandardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            daachorse::segment_fully(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            )
        });
    });

    group.bench_function("cedarwood", |b| {
        let dict = cedarwood::ForwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            cedarwood::segment_fully(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            );
        });
    });

    group.finish();
}
