Subtyping and Variance in Rust :fire:
-------------------------------------
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/) :blush:

*Date: February 6th, 2019*

## What is subtyping?

* Intuition: substitutability, compatibility
* *Type theory*  :sunglasses:: Type S is a *subtype* of type T (denoted **S <: T**) if any term/instance of type S can be "safely" used in a context where type T is expected
* Form of type polymorphism

### Does Rust have subtyping? :smirk:

Yes! But not in a familiar (OOP inheritence) way (Rust doesn't have inheritence). [It has subtyping wrt **lifetime parameters**](http://featherweightmusings.blogspot.com/2014/03/subtyping-and-coercion-in-rust.html). :sob:

* What kind of rules/relations involved between types? 

* Can we derive them from experiments?

#### Observations :neutral_face:

What's the type relationship between `&[T]` and `&mut [T]`?

* Is [`&[i32] <: &mut [i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=b364a672d7e6cb470c49fdf2a138290c)?
* Is [`&mut [i32] <: &[i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=1a077048abc0ef534eeeb7d9e3cf9ba7)?

What's the type relationship between `Vec<T>` and the slice `[T]`?

* Is [`Vec<i32> <: &[i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=f13fe69b4b815a38159ba367efa6ed7a) or [`Vec<i32> <: &mut [i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=f05b06e248f39f1b162ba4c9fdb018db)?
* Is [`&Vec<i32> <: &[i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=a7642f06e41f5190633b0f0530c5c316) or [`&mut Vec<i32> <: &mut [i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=8cf6bbc6b6ca23ee5404ed83ae1d4857)?
* Is [`&mut Vec<i32> <: &[i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=82a3ca9710d868b5fcce7a67a8b4d14e) or [`&Vec<i32> <: &mut [i32]`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ad68f8fdfc636ed48b24ea45de29157f)?
* Is [`&[i32] <: &Vec<i32>`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=cb0221f34f3c7210703c2b24c64573b4) or [`&mut [i32] <: &mut Vec<i32>`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=3392fb12e94b9993b02c9797700ea845)?

More details about the inner working later in this presetation.

## Variance :hushed:

A set of *rules/relations* indicating *how subtyping should behave in composition*. These rules are related to *generic parameter* of a type constructor. :unamused:

* [Why does it even matter?](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=95814e996026c754f946a87aaa3c4d22)

* [Is std::cell::Cell also broken?](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=93be4c952552d35fae6d7eab73899fc6)

* [Rust's Cell](https://doc.rust-lang.org/src/core/cell.rs.html#240-242) is defined as 

```rust
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

#[lang = "unsafe_cell"] // --> known to the compiler
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```
What just happened? what is the difference? :confused:

First some background.

* Unsafe is dark!
* Lifetime is a *region of code*. Lifetime parameter `'a` *behaves like a type*.
* For a type `T`, we know that `&static T` outlives (a generic) `&'a T`, therefore, `&static T <: &'a T`
* In general, if `'b: 'a` (i.e. `'b` outlives `'a` and pedagogically, we write code from left to right so larger lifetime would encompass smaller ones), then `&'b T <: &'a T`, i.e. *co-variant*. More general, for `'b: 'a` and `S <: T` then `&'b S <: &'a T`.
* [What about `&mut`?](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=d18b09887c5a8748397c617a9e7878e6)
* Therefore, `&'a mut &'static T` is NOT a subtype of `&'a mut &'a T`. In fact, `&mut` is *in-variant* (no variant relation) wrt general subtyping `S <: T`. But it is ONLY valid when **`S` exactly `T`** i.e. `&'b mut T <: &'a mut T` for `'b:'a`. :sleeping:

> The problem with making `&mut T` covariant over `T` is that it gives us the power to modify the original value when *we don't remember all of its constraints*. [[Rustonomicon](https://doc.rust-lang.org/nomicon/subtyping.html#variance)]

* Summary: 
    * `&'a T` is covariant wrt `'a` and `T`.
    * `&mut 'a T` is covariant wrt `'a` and invariant wrt `T`.
* Checkout the complete variance table [here](https://doc.rust-lang.org/nomicon/subtyping.html#variance).

Back to the question; `UnsafeCall<T>` is **invariant** wrt `T` ([deep root in compiler with #[lang = "unsafe_cell"]](https://users.rust-lang.org/t/why-unsafecell-t-is-invariant-wrt-t/24926)), so is `Cell`. But our own `MyCell<T>` is covariant wrt `T` meaning `&mut` becomes dangerous :scream: :skull:

## How to derive variance of a type?

Let `F` be a type constructor and `S <: T` then `F` is
* **Co-variant** iff `F<S> <: F<T>` (example: function return type, `&T`, `*const T`, `Vec<T>`).
* **Contra-variant** iff `F<T> <: F<S>` (example: function argument type)
* **In-variant** iff there's no subtyping relation (example:`&mut T`, `*mut T`, `UnsafeCell<T>`, `Cell<T>`).
* **Bi-variant** iff it's *both* covariant and contravariant i.e. generic parameter is not used (example: built-in type contructors `i32`, `bool`, `str`, `extern type` etc).

### Variance algebra :ghost:

* Let `0, +, -, ∞` correspond to invariance, covariance, contravariance and bivariace, respectively. Then

* [**Transform**](https://doc.rust-lang.org/nightly/nightly-rustc/rustc/ty/enum.Variance.html#method.xformhttps://doc.rust-lang.org/nightly/nightly-rustc/rustc_typeck/variance/xform/fn.glb.html) (denoted by `x` below): for type composition (example: `fn(Vec<T>)`)

| x | 0 | + | - | ∞ |
|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 0 |
| + | 0 | + | - | ∞ |
| - | 0 | - | + | ∞ |
| ∞ | ∞ | ∞ | ∞ | ∞ |

* [**Greatest lower bound**](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_typeck/variance/xform/fn.glb.html) (denote by `^` bellow): for type aggregates (example: *struct, tuple, enum and union*)

| ^ | 0 | + | - | ∞ |
|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 0 |
| + | 0 | + | 0 | + |
| - | 0 | 0 | - | - |
| ∞ | 0 | + | - | ∞ |

More details, see [rustc::ty::Variance](https://doc.rust-lang.org/nightly/nightly-rustc/rustc/ty/enum.Variance.html).

#### Exercise :wink:

Use the Variance algebra and derive the following

1. Given `UnsafeCell<T>` is invariant wrt `T`, prove that `Cell<T>` is invariant wrt `T`.
2. Show that `*mut Vec<T>` is invariant wrt `T` and the same holds for `*mut Vec<i32>`.
3. Show that `fn(Box<&'a T>)` is contravariant wrt `'a` and `T`.
4. Prove that `Vec<T>` is covariant wrt `T` ([hint](https://github.com/rust-lang/rust/blob/master/src/liballoc/raw_vec.rs)).
5. Show that `extern fn(&'a [T]) -> Result<U>` is contravariant wrt `'a` and `T` and covariant wrt `U`.
6. Show that `PhantomData<fn(T) -> T>` is invariant wrt `T`.

### Inner working of Variance :relieved:

Earlier we observed that `&Vec<i32> <: &[i32]` and `&mut Vec<i32> <: &mut [i32]`. What is actually happening is a combination of **coercion** and **auto-(re)borrowing**.

1. Deref coercion: [Vec<T>: Deref<Target = [T]>](https://doc.rust-lang.org/src/alloc/vec.rs.html#1673-1683)

2. Auto reborrow: 
    
> passing a *mutable **value** reference* into an expression context that is itself inferred to be expecting a reference, the compiler will automatically inset `&mut *value`. (Felix Klock talk, see the resource below and [Stackoverflow thread](https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082))

### Resources :smile:

* [Rustonomicon](https://doc.rust-lang.org/stable/nomicon/subtyping.html)
* [(Video presentation) Felix Klock - Subtyping in Rust and Clarke's Third Law](https://www.youtube.com/watch?v=fI4RG_uq-WU), [slides](http://pnkfx.org/presentations/rustfest-berlin-2016/slides.html)
* [Variance in Rust](https://medium.com/@kennytm/variance-in-rust-964134dd5b3e)
* [Undestanding lifetime](https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa)
* [Subtyping and coercion in Rust](http://featherweightmusings.blogspot.com/2014/03/subtyping-and-coercion-in-rust.html)
