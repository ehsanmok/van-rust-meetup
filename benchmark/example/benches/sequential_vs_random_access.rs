#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rayon;

use criterion::Criterion;
use rayon::prelude::*;

mod data;
mod random;

const N: usize = 10000;

fn random_access(c: &mut Criterion) {
    let indices = self::random::make_indices(N, data::LENGTH);
    let vector = data::DATA.to_vec();
    c.bench_function("random access", move |b| {
        b.iter(|| {
            let _: u32 = indices.iter().map(|i| vector[*i]).sum();
        })
    });
}

fn sequential_access(c: &mut Criterion) {
    let indices = (0..N).collect::<Vec<usize>>();
    let vector = data::DATA.to_vec();
    c.bench_function("sequential access", move |b| {
        b.iter(|| {
            let _: u32 = indices.iter().map(|i| vector[*i]).sum();
        })
    });
}

fn sequential_access_par(c: &mut Criterion) {
    let indices = (0..N).collect::<Vec<usize>>();
    let vector = data::DATA.to_vec();
    c.bench_function("sequential access rayon", move |b| {
        b.iter(|| {
            let _: u32 = indices.par_iter().map(|i| vector[*i]).sum();
        })
    });
}

criterion_group!(
    benches,
    sequential_access,
    random_access,
    sequential_access_par
);
criterion_main!(benches);
