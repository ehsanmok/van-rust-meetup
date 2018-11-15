Benchmark
---

*Author*: Olivier de Gevigney

*Date*: November 14, 2018

### Benchmark on Nightly

- There is a [built in benchmark](https://doc.rust-lang.org/1.8.0/book/benchmark-tests.html) framework.
- Because the API for custom test frameworks isn't stabilized, this framework is available only on Nightly.
  This is the current [RFC](https://github.com/rust-lang/rfcs/blob/master/text/2318-custom-test-frameworks.md) for stabilizing.
- Most usage is covered with the [`Bencher`](https://doc.rust-lang.org/test/struct.Bencher.html) type and the [`black_box`](https://doc.rust-lang.org/test/fn.black_box.html) function.
- If you develop your code on Stable but want to rust benchmarks with Nightly, you can place your benchmark code in a `benches` folder that will be ignored by `cargo build`.

```Rust
#![feature(test)]

extern crate test;

pub fn fib_slow(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib_slow(n - 1) + fib_slow(n - 2),
    }
}

pub fn fib_fast(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut a = 1;
            let mut b = 1;
            for _ in 0..(n - 1) {
                let t = b;
                b += a;
                a = t;
            }
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn fibonacci_slow_bench(b: &mut Bencher) {
        // The black box to avoid dead-code elimination.
        let n = black_box(35);
        b.iter(|| fib_slow(n));
    }

    #[bench]
    fn fibonacci_fast_bench(b: &mut Bencher) {
        let n = black_box(35);
        b.iter(|| fib_fast(n));
    }
}
```

### Benchmark on Stable

A couple of libraries: 
- [Bencher](https://crates.io/crates/bencher): A port of the libtest (unstable) benchmark runner
- [Criterion](https://crates.io/crates/criterion): Statistics-driven Microbenchmarking in Rust

The [`example` folder](./example/) contains a couple of examples using Criterion:
- random vs sequential access to values in a vector `cargo bench --bench sequential_vs_random_access`
- single vs double indirection for representing a matrix `cargo bench --bench vec_vs_matrix`
- Criterion runs multiple samples of the closure under test and performs linear regression to assess the performance of a single function call.
- If you have [Gnuplot](http://www.gnuplot.info/) installed, Criterion is going to build some graphs giving interesting info on the statistical method used. (Look [here](./example/target/criterion/report/index.html)).

### Other ressources

- [Seena Burns blog](http://seenaburns.com/benchmarking-rust-with-cargo-bench/)
- [Crate test](https://doc.rust-lang.org/test/index.html)
- [Flame graph](https://github.com/TyOverby/flame) (Thanks Henrik for the link!)
