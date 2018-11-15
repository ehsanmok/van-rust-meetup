Closure
-------
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/)

*Date: July 11th, 2018*

### Basics

* Closure is an (anonymous) function that captures (variables from) its *environment* (in its lexical scope). Each closure expression has a *unique anonymous type*
    ```rust
    let closure = |x: i32| -> i32 { x + 1 };
    let closure = |x: i32| { x + 1 };
    let closure = |x| { x + 1 }; // x's type will be inferred by the compiler
    let closure = |x| x + 1 ;
    ```

* [Function-like usage and type inference](https://play.rust-lang.org/?gist=54c9ce457e940ad4a40ebd901afbe939&version=stable&mode=debug&edition=2015)
* [Closing over its environment](https://play.rust-lang.org/?gist=a23e14d1a2cf55b1faee43895606de0f&version=stable&mode=debug&edition=2015)

* Use closures with generics and `Fn*` traits
    
    ```rust
    // As function argument
    fn use_in_fn<T>(x: i32, f: T) -> i32
        where T: Fn(i32) -> i32
    {
        f(x)
    }
    ```
    
    [Function example](https://play.rust-lang.org/?gist=1d0b05ddb5f599c32857d3bb5b13cf55&version=stable&mode=debug&edition=2015)
    
    ```rust
    // As in struct
    struct UseInStruct<T>
    where T: Fn(i32) -> i32
    {
        func: T,
    }
    ```
    [Struct exmaple](https://play.rust-lang.org/?gist=cc24315cb1fc17d6d4415de048ab1a76&version=stable&mode=debug&edition=2015)
    
    ```rust
    enum UseInEnum<T>
    where T: Fn(i32) -> i32
    {
        Func(T)
    }
    ```
    [Enum example](https://play.rust-lang.org/?gist=b806312e02c4ca0ddc1b0e8fee51f44f&version=stable&mode=debug&edition=2015)

* *Note*: [`fn` is a primitive type](https://doc.rust-lang.org/std/primitive.fn.html). In fact, `fn` is a [*function pointer type*](https://doc.rust-lang.org/reference/types.html#function-pointer-types). Safe `fn` implements `Fn`, `FnMut`, `FnOnce`, `Pointer`, `Clone`, `Send`, `Sync` etc. so it can be used as [non-capturing closure](https://play.rust-lang.org/?gist=e57a0150017f1f604a43da13916ca86c&version=stable&mode=debug&edition=2015)

* As of Rust 1.26, we can use the new sytax `impl Fn(i32) -> i32` which is a **concrete type** instead of generic

    ```rust
    fn use_in_fn(x: i32, f: impl Fn(i32) -> i32) -> i32 {
        f(x)
    }
    
    ```
    [Revisit function example](https://play.rust-lang.org/?gist=8cc1d29d5a66d4fd335814b846421fb2&version=stable&mode=debug&edition=2015)
    
* [As return type before 1.26 and after 1.26](https://play.rust-lang.org/?gist=5b7e4876eb75a3d02560b9b6e92d3b08&version=stable&mode=debug&edition=2015)

### Under the hood of "Capturing the Environment"

* A closure uses variables defined in its environment (enclosing lexical scope) in three way (Rust's way!):
    1. **Move** by `FnOnce` trait
    2. **Mutable borrow** by `FnMut` trait
    3. **Immutable borrow (shared reference)** by `Fn` trait

* Recall traits `Fn*` hierarchy
```rust
    pub trait FnMut<Args>: FnOnce<Args> { ... }
    pub trait Fn<Args>: FnMut<Args> { ... }
```

* When using closures, the compiler selects the "best" option. To enforce the move use `move` in a closure
```rust
    let x = vec![1];
    let f = move || x;
    println!("{:?}", x);  --> use of moved value
```

#### How does compiler do the capture given some rules?

[Exercise](https://play.rust-lang.org/?gist=0d9739dc4dcb2b22e86edb0819c741c9): Try to emulate `map` on `Option<T> -> Option<U>` like
```rust
let a = 10;
let option = Some(42);
assert_eq!(option.map(move |x| x + a), Some(52));
```
but **without** using any closures or `Fn*` traits.
([Solution](https://play.rust-lang.org/?gist=4eb54a1a8391b91a83f1c39d9fe1c918&version=stable&mode=debug&edition=2015))

* *Side note*: the equivalent of our `transform` method (in above solution) in Rust is `call_once` (in nightly as of now) and can be used (with much care) through `#![feature(unboxed_closures)` and `rust-call` ABI. We can change our solution to [this](https://play.rust-lang.org/?gist=248782effb90cfe43aa8986ab75177fc&version=nightly&mode=debug&edition=2015) with obscure tuple sytax as argument (needs stabilization of course!).


*Answer*: 
**struct as environment** + [fn_traits/unboxed_closures](https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html)

That means, for the above example, `rustc` creates an anonymous struct (equivalent to `Adder` struct) and clones the required values from the environment for its fields (closure inevitable overhead), implements `FnOnce` (we used `self`) for it when using closure `move |x| x + a` in `map`.

### Advanced: Higher-Rank Trait Bound (HRTB)

* Ability to have trait bounds that are *polymorphic over lifetimes*

```rust
fn with<T>(callback: T)
    where T : Fn(&Data) // &Data needs a lifetime, but how can we determine that?
{
    let data = Data { ... };
    callback(&data)
}

// Doesn't work! why?
fn with<'a, T>(callback: T)
    where T : Fn(&'a Data)
{                              // <--------------+
    let data = Data { ... };   //       'a scope |
    callback(&data)            //                |
}                              // <--------------+

```

[See the compiler error first.](https://play.rust-lang.org/?gist=1b4213038f4843ba85fa844183e52140&version=stable&mode=debug&edition=2015)
There's *no single lifetime* that can be placed there and solve the lifetime issue! what to do?

*Answer*: generate an *infinite* list of trait bounds that `T` must satisfiy with `T: for<'a> Fn<&'a Data>`. Read `for<'a>` as "for all choices of 'a". The compiler chooses the smallest lifetime that satisfies the bound. [See the fix](https://play.rust-lang.org/?gist=7d401ba450da4017185287f8018f2468&version=stable&mode=debug&edition=2015)

* [Exercise](https://play.rust-lang.org/?gist=9126b3c9a8f9474e06afb2b89b6f9170&version=stable&mode=debug&edition=2015): Continue from the previous exercise, implement `filter` on `MyOption`. ([Solution](https://play.rust-lang.org/?gist=e641d9690e80a997543077543f538a3e&version=stable&mode=debug&edition=2015))

### References

* Rust's book closure chapter, [V1](https://doc.rust-lang.org/book/first-edition/closures.html), [V2](https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html)

* Rust 2018 edition: [impl trait](https://rust-lang-nursery.github.io/edition-guide/2018/transitioning/traits/impl-trait.html)

* Unstable book [fn_traits](https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html)

* [Finding closures in Rust](http://huonw.github.io/blog/2015/05/finding-closure-in-rust/)

* HRTB: [RFC](https://github.com/nox/rust-rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md), [Nomicon](https://doc.rust-lang.org/nomicon/hrtb.html), [Niko's blog](http://smallcultfollowing.com/babysteps/blog/2016/11/04/associated-type-constructors-part-3-what-higher-kinded-types-might-look-like/)