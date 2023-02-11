use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

const LEN: usize = 1000;
const SEED: u64 = 42;

use im_rc::OrdMap;
use rpds::HashTrieMap;
use rpds::RedBlackTreeMap;
use std::collections::BTreeMap;

fn insert(c: &mut Criterion) {
    let mut g = c.benchmark_group("Insertion");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    /* Ordered Map */

    g.bench_function("rpds::RBTM, Immut", |b| {
        b.iter_batched(
            || RedBlackTreeMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x = x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::RBTM, Mut", |b| {
        b.iter_batched(
            || RedBlackTreeMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert_mut(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap, Immut", |b| {
        b.iter_batched(
            || OrdMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    let y = x.clone();
                    x.insert(*key, value);
                    black_box(y);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap, Mut", |b| {
        b.iter_batched(
            || OrdMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::BTM", |b| {
        b.iter_batched(
            || BTreeMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    /* Unordered Map */

    g.bench_function("rpds::HTM, Immut", |b| {
        b.iter_batched(
            || HashTrieMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x = x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::HTM, Mut", |b| {
        b.iter_batched(
            || HashTrieMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert_mut(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap, Immut", |b| {
        b.iter_batched(
            || im_rc::HashMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    let y = x.clone();
                    x.insert(*key, value);
                    black_box(y);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap, Mut", |b| {
        b.iter_batched(
            || im_rc::HashMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::HashMap", |b| {
        b.iter_batched(
            || std::collections::HashMap::new(),
            |x| {
                let mut x = x;
                for (value, key) in keys.iter().enumerate() {
                    x.insert(*key, value);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });
}

fn remove(c: &mut Criterion) {
    let mut g = c.benchmark_group("Removal");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    let iter = keys.iter().enumerate().map(|(value, key)| (*key, value));
    let m1: RedBlackTreeMap<usize, usize> = RedBlackTreeMap::from_iter(iter.clone());
    let m2: OrdMap<usize, usize> = OrdMap::from_iter(iter.clone());
    let m3: BTreeMap<usize, usize> = BTreeMap::from_iter(iter.clone());

    let um1: HashTrieMap<usize, usize> = HashTrieMap::from_iter(iter.clone());
    let um2: im_rc::HashMap<usize, usize> = im_rc::HashMap::from_iter(iter.clone());
    let um3: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::from_iter(iter.clone());

    /* Ordered */

    g.bench_function("rpds::RBTM, Immut", |b| {
        b.iter_batched(
            || m1.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x = x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::RBTM, Mut", |b| {
        b.iter_batched(
            || m1.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove_mut(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap, Immut", |b| {
        b.iter_batched(
            || m2.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    let y = x.clone();
                    x.remove(key);
                    black_box(y);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap, Mut", |b| {
        b.iter_batched(
            || m2.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::BTM", |b| {
        b.iter_batched(
            || m3.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    /* Unordered Map */

    g.bench_function("rpds::HTM, Immut", |b| {
        b.iter_batched(
            || um1.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x = x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::HTM, Mut", |b| {
        b.iter_batched(
            || um1.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove_mut(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap, Immut", |b| {
        b.iter_batched(
            || um2.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    let y = x.clone();
                    x.remove(key);
                    black_box(y);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap, Mut", |b| {
        b.iter_batched(
            || um2.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::HashMap", |b| {
        b.iter_batched(
            || um3.clone(),
            |x| {
                let mut x = x;
                for key in keys.iter() {
                    x.remove(key);
                }
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });
}

fn get(c: &mut Criterion) {
    let mut g = c.benchmark_group("Get");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    let mut keys2 = keys.clone();
    keys2.shuffle(&mut rng);

    let iter = keys.iter().enumerate().map(|(value, key)| (*key, value));
    let m1: RedBlackTreeMap<usize, usize> = RedBlackTreeMap::from_iter(iter.clone());
    let m2: OrdMap<usize, usize> = OrdMap::from_iter(iter.clone());
    let m3: BTreeMap<usize, usize> = BTreeMap::from_iter(iter.clone());

    let um1: HashTrieMap<usize, usize> = HashTrieMap::from_iter(iter.clone());
    let um2: im_rc::HashMap<usize, usize> = im_rc::HashMap::from_iter(iter.clone());
    let um3: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::from_iter(iter.clone());

    g.bench_function("rpds::RBTM", |b| {
        b.iter_batched(
            || m1.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap", |b| {
        b.iter_batched(
            || m2.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::BTM", |b| {
        b.iter_batched(
            || m3.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });

    /* Unordered Map */

    g.bench_function("rpds::HTM", |b| {
        b.iter_batched(
            || um1.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap", |b| {
        b.iter_batched(
            || um2.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::HashMap", |b| {
        b.iter_batched(
            || um3.clone(),
            |x| {
                for key in keys2.iter() {
                    black_box(x.get(key));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn iter(c: &mut Criterion) {
    let mut g = c.benchmark_group("Iteration");
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut keys = (0usize..LEN).collect_vec();
    keys.shuffle(&mut rng);

    let mut keys2 = keys.clone();
    keys2.shuffle(&mut rng);

    let iter = keys.iter().enumerate().map(|(value, key)| (*key, value));
    let m1: RedBlackTreeMap<usize, usize> = RedBlackTreeMap::from_iter(iter.clone());
    let m2: OrdMap<usize, usize> = OrdMap::from_iter(iter.clone());
    let m3: BTreeMap<usize, usize> = BTreeMap::from_iter(iter.clone());

    let um1: HashTrieMap<usize, usize> = HashTrieMap::from_iter(iter.clone());
    let um2: im_rc::HashMap<usize, usize> = im_rc::HashMap::from_iter(iter.clone());
    let um3: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::from_iter(iter.clone());

    g.bench_function("rpds::RBTM", |b| {
        b.iter_batched(
            || m1.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::OrdMap", |b| {
        b.iter_batched(
            || m2.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::BTM", |b| {
        b.iter_batched(
            || m3.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rpds::HTM", |b| {
        b.iter_batched(
            || um1.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("im::HashMap", |b| {
        b.iter_batched(
            || um2.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("std::HashMap", |b| {
        b.iter_batched(
            || um3.clone(),
            |x| {
                for elem in x.iter() {
                    black_box(elem);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(map, insert, remove, get, iter);
criterion_main!(map);
