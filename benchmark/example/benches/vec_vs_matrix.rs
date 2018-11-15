#[macro_use]
extern crate criterion;
extern crate criterion_demo;

use criterion::{black_box, Criterion, Fun};
use criterion_demo::Matrix;

mod data;

const N: usize = 100;

fn make_vec_of_vec(a: &[u32], size: usize) -> Vec<Vec<u32>> {
    let mut v = Vec::with_capacity(size);
    for i in 0..size {
        v.push(a[(size * i)..(size * (i + 1))].to_vec());
    }
    v
}

fn accessing_value(c: &mut Criterion) {
    let v = make_vec_of_vec(&data::DATA, black_box(N));
    let vec_of_vec = Fun::new("Vec of Vec", move |b, &(i, j): &(usize, usize)| {
        b.iter(|| v[i][j])
    });

    let m = Matrix::from_slice(&data::DATA, black_box(N));
    let matrix = Fun::new("Matrix", move |b, &(i, j): &(usize, usize)| {
        b.iter(|| m[(i, j)])
    });

    let functions = vec![vec_of_vec, matrix];
    c.bench_functions("Accessing", functions, black_box((15, 17)));
}

criterion_group!(benches, accessing_value);
criterion_main!(benches);
