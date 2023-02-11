use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

const LEN: usize = 10000;
const SEED: u64 = 42;

fn push(c: &mut Criterion) {
    let mut g = c.benchmark_group("Vector Push");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    g.bench_function("rpds::Vector, Immut", |b| {
        b.iter_batched(
            || rpds::Vector::new(),
            |x| {
                let mut x = x;
                for v in keys.iter() {
                    x = x.push_back(*v);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::Vector, Mut", |b| {
        b.iter_batched(
            || rpds::Vector::new(),
            |x| {
                let mut x = x;
                for v in keys.iter() {
                    x.push_back_mut(*v);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::Vector, Immut", |b| {
        b.iter_batched(
            || im_rc::Vector::new(),
            |x| {
                let mut x = x;
                for v in keys.iter() {
                    let y = x.clone();
                    x.push_back(*v);
                    black_box(y);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::Vector, Mut", |b| {
        b.iter_batched(
            || im_rc::Vector::new(),
            |x| {
                let mut x = x;
                for v in keys.iter() {
                    x.push_back(*v);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::Vec", |b| {
        b.iter_batched(
            || Vec::new(),
            |x| {
                let mut x = x;
                for v in keys.iter() {
                    x.push(*v);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });
}

fn get(c: &mut Criterion) {
    let mut g = c.benchmark_group("Vector Get");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    let mut keys2 = keys.clone();
    keys2.shuffle(&mut rng);

    let iter = keys.iter().cloned();

    let v1: rpds::Vector<usize> = iter.clone().collect();
    let v2: im_rc::Vector<usize> = iter.clone().collect();
    let v3: Vec<usize> = iter.clone().collect();

    g.bench_function("rpds::Vector", |b| {
        b.iter_batched(
            || v1.clone(),
            |x| {
                for &index in keys2.iter() {
                    black_box(x[index]);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::Vector", |b| {
        b.iter_batched(
            || v2.clone(),
            |x| {
                for &index in keys2.iter() {
                    black_box(x[index]);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::Vec", |b| {
        b.iter_batched(
            || v3.clone(),
            |x| {
                for &index in keys2.iter() {
                    black_box(x[index]);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn iter(c: &mut Criterion) {
    let mut g = c.benchmark_group("Vector Iter");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    let iter = keys.iter().cloned();

    let v1: rpds::Vector<usize> = iter.clone().collect();
    let v2: im_rc::Vector<usize> = iter.clone().collect();
    let v3: Vec<usize> = iter.clone().collect();

    g.bench_function("rpds::Vector", |b| {
        b.iter_batched(
            || v1.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::Vector", |b| {
        b.iter_batched(
            || v2.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::Vec", |b| {
        b.iter_batched(
            || v3.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(vector, push, get, iter);
criterion_main!(vector);
