SIMD in Rust :fire: :fire: :fire:
---------------------------------
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/) :bowtie:

*Date: May 1st, 2019*

## What is SIMD? :star:

* Historically, a parallel computing architecture
* Doing more at one instruction ([CPU cycle](https://en.wikipedia.org/wiki/Instruction_cycle)). Useful for compute intensive tasks.
* **Single Instruction, Multiple Data**: load multiple data at once and perform a *single* operation on all. A form of [auto-vectorization / auto-parallelism](https://en.wikipedia.org/wiki/Automatic_vectorization). Note that, compiler may *not* be able to that for us!
* Data level *parallelism* (not concurrency)
* SIMD instructions are hardware intrinsics specific, namely [Intel guide (SSE, AVX, etc)](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#), [ARM guide (NEON)](http://infocenter.arm.com/help/topic/com.arm.doc.ihi0073a/IHI0073A_arm_neon_intrinsics_ref.pdf), etc.
* As SIMD is about #Instructions <--> #Data then one can also consider other architecture scenarios such as 
    - Single Instuction, Single Data: [SISD (von Neumann architecture)](https://en.wikipedia.org/wiki/SISD)
    - Multiple Instructions, Single Data: [MISD (pipeline architecture)](https://en.wikipedia.org/wiki/MISD)
    - Multiple Instructions, Multiple Data: [MIMD/multi-processing (i.e. multiple async, independent processors on shared memory or distributed memory)](https://en.wikipedia.org/wiki/MIMD)

* Not to be confused with GPGPU, *Single Instruction, Multiple Thread* ([SIMT](https://en.wikipedia.org/wiki/Single_instruction,_multiple_threads)) which is basically *SIMD + multithreading* :boom:

## How to use SIMD in Rust? :muscle:

* Available in [core::arch](https://doc.rust-lang.org/core/arch/index.html) as of Rust 1.27 and under `#![feature(stdsimd)]`.
* There are number of [common architectures](https://doc.rust-lang.org/core/arch/index.html#other-architectures) available such as `x86, x86_64, arm, aarch64, nvptx, wasm32` etc. :hushed:
* `core::arch` is pure vendor specific intrinsics. Not easy to use and must check the reference all the time. Also very verbose in naming, e.g. [`_mm256_add_epi64`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_add_epi64&expand=100) :weary:
* Static CPU detection with `#[cfg(...)]` such as

```rust
// if "avx2" is available
#[cfg(target_arch = "x86")]
use std::arch::x86::_mm256_add_epi64;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm256_add_epi64;
```

and at compile time with `RUSTFLAGS='-C target-cpu=native' cargo build` or `RUSTFLAGS='-C target-feature=+avx2' cargo build`. 

*Note: This will enable [AVX2 (Advance Vector Extensions)](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions) for the entire program.*

* Dynamic CPU detection

```rust
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn foo_avx2() { // <-- requires unsafe because of `target_feature`
    #[cfg(target_arch = "x86")]
    use std::arch::x86::_mm256_add_epi64;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::_mm256_add_epi64;

    _mm256_add_epi64(...);
}

fn foo() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") { // <-- macro is platform-specific
            return unsafe { foo_avx2() };
        }
    }

    // fallback implementation without using AVX2
}
```

*Note: This will enable AVX2 only for `foo_avx2` function.*

* `core::arch` is not ergonomic :broken_heart: (see the [example](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2015&gist=48b89624724258cd7fefcd0654355845) :cry:), however, at the time of this presentation (Rust 1.34) there's [packed_simd](https://rust-lang-nursery.github.io/packed_simd/packed_simd/) that provides much better ergonomic and portable data structures and API which will hopefully be stabilized in `std::simd`.
* `packed_simd` :heart_eyes: provides **portable packed SIMD vectors** (packed means compile-time fixed size) in a single data structure `Simd<[T; N]>` which has many aliases according to underlying concrete type and the number of lanes. Basically, `{element_type}{lane_width}x{number_of_lanes}`. For example, 
    - `Simd<[i32; 4]>` is `i32x4`: 128-bit vector with 4, `i32` lanes.
    - `Simd<[f32; 16]>` is `f32x16`: 512-bit vector with 16, `f32` lanes.

* Operations on SIMD vectors are
    - **Vertical** (lane-wise) :open_mouth:
    ```rust
    let a = i32x4::new(1, 2, 3, 4);
    let b = i32x4::new(4, 3, 2, 1);
    // a + b happens all at once in parallel (vertically) and (typically) in one cycle
    assert_eq!(a + b, i32x4::new(5, 5, 5, 5));
    ```
    - **Horizontal** (along a vector): such as `a.sum(), a.max_element()` etc. They are usually translated into a *sequence* of multiple SIMD, that's why they're slower :grimacing:

* Layout-wise, `SIMD<[T; N]>` is the same as `[T; N]` :metal:

### Inner product example :stuck_out_tongue_closed_eyes:

* Plain:

```rust
fn inner_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut ret = 0.f32;
    for i in (0..a.len()) {
        ret += a[i] * b[i];
    }
    ret
}

fn functional_inner_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
```

* Packed SIMD:

```rust
use packed_simd::f32x4;

fn inner_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 4 == 0); // otherwise `from_slice_unaligned` will panic!

    let mut ret = f32x4::splat(0.); // creates [0.f32, 0., 0., 0.]
    for i in (0..a.len()).step_by(4) {
        let x = f32x4::from_slice_unaligned(&a[i..]);
        let y = f32x4::from_slice_unaligned(&b[i..]);
        ret += x * y;
    }
    ret.sum()
}

fn functional_inner_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 4 == 0);

    a.chunks_exact(4)
        .map(f32x4::from_slice_unaligned)
        .zip(b.chunks_exact(4).map(f32x4::from_slice_unaligned))
        .map(|(x, y)| x * y)
        .sum::<f32x4>()
        .sum()
}
```

### SIMD in Rust crates

* [ripgrep](https://github.com/BurntSushi/ripgrep): world's best grep!
* [regex](https://github.com/rust-lang/regex): to accelerate multiple substring search.
* [bytecount](https://github.com/llogiq/bytecount): to accelerate counting bytes.
* [simdnoise](https://crates.io/crates/simdnoise): super fast SIMD noise library
* [and more](https://crates.io/search?q=simd)


### Resources :relieved:
* [Flynn's taxanomy](https://en.wikipedia.org/wiki/Flynn%27s_taxonomy)
* [stable (vector intrinsic) SIMD RFC](https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md)
* [stdsimd](https://github.com/rust-lang-nursery/stdsimd)
* [std::simd RFC](https://github.com/gnzlbg/rfcs/blob/ppv/text/0000-ppv.md)
* [Ergonomic `packed_simd` crate](https://rust-lang-nursery.github.io/packed_simd/packed_simd/)
