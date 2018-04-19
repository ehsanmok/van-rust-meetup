Functions and FFI (part 1)
---
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/)

*Date: April 18th, 2018*

### Plain function

* Primitive type `fn` for creating plain functions
* Plain function is [zero-sized](https://play.rust-lang.org/?gist=7fd5d3b2ce74a9f9d3cf0f09558e743d&version=stable)
* All functions impl: `Fn`, `FnMut`, `FnOnce`, `Copy`, `Clone`, `Send` and `Sync` traits
* Safe ⊂ `unsafe` worlds:
    - Unsafe: No compiler (static) guarantees about memory satefy
    - Dereference a raw pointer
    - *Call an unsafe (foreign) function or method* (related to this presentation). [Example](https://play.rust-lang.org/?gist=6a94bda08d364f2e6dd7ccac8374627c&version=stable)
    - Access or modify a mutable static variable
    - Impl an unsafe trait

### Closure

* Impl one of `Fn`, `FnMut` or `FnOnce` traits. In fact;
```rust
pub trait FnMut<Args>: FnOnce<Args> { ... }
pub trait Fn<Args>: FnMut<Args> { ... }
```
    - `FnOnce`: consumes the variables it captures from its enclosing scope (environment)
    - `FnMut`: can mutate the environment it was mutably borrowd
    - `Fn`: borrows values from its environment immutably


* Rust *infers* which trait to use based on how the closure uses the values from the environment
* Use `move` to force the closure to take ownership of the values from the environment
* Create costum closure-like type ([unstable fn_traits](https://doc.rust-lang.org/1.24.1/unstable-book/library-features/fn-traits.html)) [ example](https://play.rust-lang.org/?gist=249e4c7bd2da91fb4e88f36624fb45f6&version=nightly) using [`extern "rust-call" fn`](https://doc.rust-lang.org/1.17.0/src/core/ops.rs.html#2599). More on signature later!


### Function pointer

* Create **Function pointer type** `fn(i32) -> i32` by
    - Casting plain functions:
    ```Rust
    fn inc(x: i32) -> i32 { x + 1 }
    let ptr: fn(i32) -> i32 = inc;
    ```
    - Casting closures (that *don't* capture an environment)
    ```Rust
    fn clos_ptr: fn(i32) -> i32 = |x| x + 1;
    ```
    - [Function pointer is 8 bytes!](https://play.rust-lang.org/?gist=73dcfe38467ae201c223d277bc51203f&version=stable)

#### Function pointer type modifiers: `unsafe` and `extern`
* Safe and unsafe function pointers:
    - *Safe function pointers* can only point to *safe* functions
    - *Unsafe function pointers* can point to either *safe* or *unsafe* functions
    ```Rust
    fn inc(x: i32) -> i32 { x + 1 }
    unsafe unsafe_inc(x: i32) -> i32 { x + 1 }

    let safe_ptr: fn(i32) -> i32 = inc;
    let unsafe_ptr: unsafe fn(i32) -> i32 = unsafe_inc;
    let really_safe_ptr: unsafe fn(i32) -> i32 = inc;
    ```

* Function pointers depend on what [ABI](https://en.wikipedia.org/wiki/Application_binary_interface) they use
    - Rust's ABI `fn`, default type `extern "Rust" fn`
    - C ABI: `extern "C" fn` type. Shorter `extern fn` (default ABI is "C")
    - `extern "ABI name" fn"` type: for various supported ABIs `stdcall`(win32), `Rust`, `rust-intrinsic` (compiler intrinsics), `rust-call` (saw in `FnOnce`, `fn_traits`  above), `system`, etc.
    - **"Rust"**, **"C"** and **"system"** ABIs are cross-platforms and *all compilers* are guaranteed to support


* `extern "ABI name" fn()` *allows foreign code to call Rust's code*.
 Opposite to `extern block` where *Rust calls foreign codes*.
    ```rust
    // Foreign code (different ABI) can call Rust
    #[no_mangle] // turns off Rust's name mangling -> easier to link to
    extern "C" fn inc(x: i32) -> i32 { x + 1 }
    extern fn inc(x: i32) -> i32 { x + 1 } // default ABI is "C"

    // Rust calls foreign code (basis for FFI). Compiler translates to Rust ABI
    #[link(...)] // instruct linker
    extern { // default ABI is "C" as well
        fn foo(x: i32) -> i32; // function foreign interface
    }
    ```
    - [Example](https://github.com/ehsanmok/van-rust-meetup/tree/master/ffi/basic)
    - Enforce interoperability with the same representation/layout through `#[repr(C)]` (though it is pretty close to `#[repr(Rust)]`)
        ```rust
        #[repr(C)]
        struct S { x: i32 }
        ```

### References

* [Rust books](https://doc.rust-lang.org/book/)
* [Rust doc](https://doc.rust-lang.org/std/)
* [Nomicon](https://doc.rust-lang.org/nomicon/README.html)
* [Rust reference](https://doc.rust-lang.org/reference/introduction.html)
* [Unstable book](https://doc.rust-lang.org/unstable-book/the-unstable-book.html)
