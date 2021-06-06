use coordination_2021::three_adder;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::runtime;

pub fn bench_frameworks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("three_adder_frameworks");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    group.bench_function("rumpsteak", |bencher| {
        use three_adder::rumpsteak::{adder_a, adder_b, adder_c, Roles};
        use tokio::try_join;

        bencher.iter(|| {
            let Roles(mut a, mut b, mut c) = Roles::default();
            rt.block_on(async {
                try_join!(adder_a(&mut a), adder_b(&mut b), adder_c(&mut c)).unwrap();
            });
        });
    });

    group.bench_function("mpstthree", |bencher| {
        use mpstthree::fork::fork_mpst;
        use three_adder::mpstthree::{adder_a, adder_b, adder_c};

        bencher.iter(|| {
            let (a, b, c) = fork_mpst(adder_a, adder_b, adder_c);
            assert!(a.join().is_ok() && b.join().is_ok() && c.join().is_ok());
        });
    });
}

pub fn bench_channels(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("three_adder_channels");
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    for repeats in 1..=5 {
        group.bench_with_input(
            BenchmarkId::new("rumpsteak", repeats),
            &repeats,
            |bencher, &repeats| {
                use three_adder::rumpsteak::{adder_a, adder_b, adder_c, Roles};
                use tokio::try_join;

                bencher.iter(|| {
                    let Roles(mut a, mut b, mut c) = Roles::default();
                    for _ in 0..repeats {
                        rt.block_on(async {
                            try_join!(adder_a(&mut a), adder_b(&mut b), adder_c(&mut c)).unwrap();
                        });
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("rumpsteak_oneshot", repeats),
            &repeats,
            |bencher, &repeats| {
                use rumpsteak_oneshot::session3;
                use three_adder::rumpsteak_oneshot::{adder_a, adder_b, adder_c};

                bencher.iter(|| {
                    for _ in 0..repeats {
                        rt.block_on(session3(adder_a, adder_b, adder_c));
                    }
                });
            },
        );
    }
}

criterion_group!(benches, bench_frameworks, bench_channels);

criterion_main!(benches);
