use coordination_2021::double_buffering;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::runtime;

pub fn bench_frameworks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("double_buffering_frameworks");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    group.bench_function("rumpsteak", |bencher| {
        use double_buffering::rumpsteak::{kernel, sink, source, Roles};
        use tokio::try_join;

        bencher.iter(|| {
            let Roles(mut s, mut k, mut t) = Roles::default();
            rt.block_on(async {
                try_join!(
                    source::<false>(&mut s),
                    kernel::<false>(&mut k),
                    sink::<false>(&mut t),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("mpstthree", |bencher| {
        use double_buffering::mpstthree::{kernel, sink, source};
        use mpstthree::fork::fork_mpst;

        bencher.iter(|| {
            let (s, k, t) = fork_mpst(source, kernel, sink);
            assert!(s.join().is_ok() && k.join().is_ok() && t.join().is_ok());
        });
    });
}

pub fn bench_channels(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("double_buffering_channels");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    for repeats in 1..=5 {
        group.bench_with_input(
            BenchmarkId::new("rumpsteak", repeats),
            &repeats,
            |bencher, &repeats| {
                use double_buffering::rumpsteak::{kernel, sink, source, Roles};
                use tokio::try_join;

                bencher.iter(|| {
                    let Roles(mut s, mut k, mut t) = Roles::default();
                    for _ in 0..repeats {
                        rt.block_on(async {
                            try_join!(
                                source::<false>(&mut s),
                                kernel::<false>(&mut k),
                                sink::<false>(&mut t),
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
                use double_buffering::rumpsteak_oneshot::{kernel, sink, source};
                use rumpsteak_oneshot::session3;

                bencher.iter(|| {
                    for _ in 0..repeats {
                        rt.block_on(session3(source, kernel, sink));
                    }
                });
            },
        );
    }
}

pub fn bench_optimizations(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("double_buffering_optimizations");
    let mut builder = runtime::Builder::new_current_thread();
    let rt = builder.enable_time().build().unwrap();

    group.bench_function("unoptimized", |bencher| {
        use double_buffering::rumpsteak::{kernel, sink, source, Roles};
        use tokio::try_join;

        let Roles(mut s, mut k, mut t) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    source::<true>(&mut s),
                    kernel::<true>(&mut k),
                    sink::<true>(&mut t),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("optimized_weak", |bencher| {
        use double_buffering::rumpsteak::{kernel_optimized_weak, sink, source, Roles};
        use tokio::try_join;

        let Roles(mut s, mut k, mut t) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    source::<true>(&mut s),
                    kernel_optimized_weak::<true>(&mut k),
                    sink::<true>(&mut t),
                )
                .unwrap();
            });
        });
    });

    group.bench_function("optimized", |bencher| {
        use double_buffering::rumpsteak::{kernel_optimized, sink, source, Roles};
        use tokio::try_join;

        let Roles(mut s, mut k, mut t) = Roles::default();
        bencher.iter(|| {
            rt.block_on(async {
                try_join!(
                    source::<true>(&mut s),
                    kernel_optimized::<true>(&mut k),
                    sink::<true>(&mut t),
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
