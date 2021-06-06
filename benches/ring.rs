use coordination_2021::ring;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::runtime;

pub fn bench_frameworks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("ring_frameworks");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    group.bench_function("rumpsteak", |bencher| {
        use ring::rumpsteak::{ring_a, ring_b, ring_c, Roles};
        use tokio::try_join;

        bencher.iter(|| {
            let Roles(mut a, mut b, mut c) = Roles::default();
            rt.block_on(async {
                try_join!(
                    ring_a::<false>(&mut a),
                    ring_b::<false>(&mut b),
                    ring_c::<false>(&mut c),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("mpstthree", |bencher| {
        use mpstthree::fork::fork_mpst;
        use ring::mpstthree::{ring_a, ring_b, ring_c};

        bencher.iter(|| {
            let (a, b, c) = fork_mpst(ring_a, ring_b, ring_c);
            assert!(a.join().is_ok() && b.join().is_ok() && c.join().is_ok());
        });
    });
}

pub fn bench_channels(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("ring_channels");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    for repeats in 1..=5 {
        group.bench_with_input(
            BenchmarkId::new("rumpsteak", repeats),
            &repeats,
            |bencher, &repeats| {
                use ring::rumpsteak::{ring_a, ring_b, ring_c, Roles};
                use tokio::try_join;

                bencher.iter(|| {
                    let Roles(mut a, mut b, mut c) = Roles::default();
                    for _ in 0..repeats {
                        rt.block_on(async {
                            try_join!(
                                ring_a::<false>(&mut a),
                                ring_b::<false>(&mut b),
                                ring_c::<false>(&mut c),
                            )
                            .unwrap();
                        });
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("rumpsteak_oneshot", repeats),
            &repeats,
            |bencher, &repeats| {
                use ring::rumpsteak_oneshot::{ring_a, ring_b, ring_c};
                use rumpsteak_oneshot::session3;

                bencher.iter(|| {
                    for _ in 0..repeats {
                        rt.block_on(session3(ring_a, ring_b, ring_c));
                    }
                });
            },
        );
    }
}

pub fn bench_optimizations(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("ring_optimizations");
    let mut builder = runtime::Builder::new_current_thread();
    let rt = builder.enable_time().build().unwrap();

    group.bench_function("unoptimized", |bencher| {
        use ring::rumpsteak::{ring_a, ring_b, ring_c, Roles};
        use tokio::try_join;

        let Roles(mut a, mut b, mut c) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    ring_a::<true>(&mut a),
                    ring_b::<true>(&mut b),
                    ring_c::<true>(&mut c),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("optimized_b", |bencher| {
        use ring::rumpsteak::{ring_a, ring_b_optimized, ring_c, Roles};
        use tokio::try_join;

        let Roles(mut a, mut b, mut c) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    ring_a::<true>(&mut a),
                    ring_b_optimized::<true>(&mut b),
                    ring_c::<true>(&mut c),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("optimized_c", |bencher| {
        use ring::rumpsteak::{ring_a, ring_b, ring_c_optimized, Roles};
        use tokio::try_join;

        let Roles(mut a, mut b, mut c) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    ring_a::<true>(&mut a),
                    ring_b::<true>(&mut b),
                    ring_c_optimized::<true>(&mut c),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("optimized", |bencher| {
        use ring::rumpsteak::{ring_a, ring_b_optimized, ring_c_optimized, Roles};
        use tokio::try_join;

        let Roles(mut a, mut b, mut c) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    ring_a::<true>(&mut a),
                    ring_b_optimized::<true>(&mut b),
                    ring_c_optimized::<true>(&mut c),
                )
                .unwrap();
            });
        });
    });
}

criterion_group!(
    benches,
    bench_frameworks,
    bench_channels,
    bench_optimizations,
);

criterion_main!(benches);
