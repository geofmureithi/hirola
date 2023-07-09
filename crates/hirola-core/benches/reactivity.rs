use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hirola_core::prelude::*;

pub fn bench(c: &mut Criterion) {
    c.bench_function("reactivity_signals", |b| {
        b.iter(|| {
            let state = Mutable::new(black_box(0));

            for _i in 0..1000 {
                let value = state.get();
                state.set(value + 1);
            }
        })
    });

    c.bench_function("reactivity_effects", |b| {
        b.iter(|| {
            let state = Mutable::new(black_box(0));
            create_effect(state.clone(), move |value| {
                let _double = value * 2;
            });

            for _i in 0..1000 {
                state.set(state.get() + 1);
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.05 /* noisy CI */);
    targets = bench
}
criterion_main!(benches);
