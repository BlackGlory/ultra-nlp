use criterion::{criterion_group, criterion_main, Criterion, black_box};
use ultra_nlp::{daachorse, cedarwood, hashmap, BehaviorForUnmatched};

criterion_group!(benches, bench_segment_bidirectional_longest);
criterion_main!(benches);

fn bench_segment_bidirectional_longest(c: &mut Criterion) {
    let mut group = c.benchmark_group(
      "segment_bidirectional_longest"
    );

    let patterns: Vec<&str> = vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"];
    let text = " 南京市长江大桥, hello world ";

    group.bench_function("daachorse", |b| {
        let forward_dict = daachorse::ForwardDictionary::new(
            patterns.clone()
        ).unwrap();
        let backward_dict = daachorse::BackwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            daachorse::segment_bidirectional_longest(
                black_box(text),
                black_box(&forward_dict),
                black_box(&backward_dict),
                black_box(BehaviorForUnmatched::Ignore),
            )
        });
    });

    group.bench_function("cedarwood", |b| {
        let forward_dict = cedarwood::ForwardDictionary::new(
            patterns.clone()
        ).unwrap();
        let backward_dict = cedarwood::BackwardDictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            cedarwood::segment_bidirectional_longest(
                black_box(text),
                black_box(&forward_dict),
                black_box(&backward_dict),
                black_box(BehaviorForUnmatched::Ignore),
            );
        });
    });

    group.bench_function("hashmap", |b| {
        let dict = hashmap::Dictionary::new(
            patterns.clone()
        ).unwrap();

        b.iter(|| {
            hashmap::segment_bidirectional_longest(
                black_box(text),
                black_box(&dict),
                black_box(BehaviorForUnmatched::Ignore),
            );
        });
    });

    group.finish();
}
