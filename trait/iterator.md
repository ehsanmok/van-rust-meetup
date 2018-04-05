Iterator
---
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/)

*Date: April 4th, 2018*

### Quick recap of traits

* Defining *shared behaviour* (interface).
 * Examples: `Clone, Copy, Debug, Display, Drop` and `Iterator`
* For trivial constructs compiler can impl them for us with `#[derive(...)]`. Iterating is *not* trivial!

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Int(i32);
```
Equivalent to
```Rust
impl Clone for Int {
    fn clone(&self) -> Self { *self }
}
// Copy is allowed since i32 is copy
impl Copy for Int {}
impl Debug for Int {...}
...
```

* Can have default impls. Examples: `Clone` above,

* Type bounds over generics tells compiler to check for defined behaviours (impled traits)
    - `struct Int<T: Copy>(T)`

* *Associate types*: placeholder for trait definition.
    - Example, `type Item` in `Iterator` or `type Output` in `Add`
    ```Rust
    trait Add<RHS=Self> {
        type Output; // associated type
        fn add(self, rhs: RHS) -> Self::Output; // associated function
    }
    ```

* *Default generic type parameters and operator overloading*: `<PlaceholderType=ConcreteType>`.
    - Example, `Add<RHS=Self>`

[Exercise](https://play.rust-lang.org/?gist=2fe72ad5dcce27ae13a4c61aa14c7097&version=stable):
Let's impl `Add` for `Int` i.e. `Int + Int`, `Int + String` and `Int + &str`
([Solution](https://play.rust-lang.org/?gist=5b97709bf20cd63876ddbe8f3414ce99&version=stable))

* Sub-behaviour (customized): `traits: super_trait`. Example
    - `pub trait Copy: Clone {}`
    - `pub trait Eq: PartialEq<Self> { ... }`
    - `pub trait Ord: Eq + PartialOrd<Self> { ... }`
    - `pub trait FnMut<Args>: FnOnce<Args> { ... }`
    - `pub trait Fn<Args>: FnMut<Args> { ... }`

* [*Orphan rule*](http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/#the-covered-rule): allowed to implement a trait on a type as long as **either** the trait **or** the type are local to our crate.
 - Above exercise; needed `Int` wrapper around `i32` to make it local to our crate (*newtype pattern*)

### Polymorphism

* Trait static dispatch: (impled a trait for multiple types)
    - Callee is known at compile time
    - Monomorphization
    - [Example from the book](https://play.rust-lang.org/?gist=fa9a2dbd70cb6c0a0be98a0bb6377c59&version=stable)

* Dynamic dispatch:
    - Runtime
    - Mechanism is through `Trait object` i.e. a *trait behind a pointer* (type erasure). Stores any value that impl the trait.
    - [Example cont.](https://play.rust-lang.org/?gist=2b0ad49fe55654dda3ef7e54ec6ce658&version=stable)

Needs a *separate* presentation!

### Iterator

Main iterator trait

```Rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

```

* [iter() vs. iter_mut() vs. into_iter()](https://play.rust-lang.org/?gist=9c96353d5047d499cb6c1bbcef725efe&version=stable)

* Create iterator in two steps:
    1. A `struct State` holding iterator's state
    2. `impl Iterator for State`

[Exercise](https://play.rust-lang.org/?gist=84c7b15a8db25dfb020a8ee9762ede75&version=stable): create empty iterator. An iterator returning `None`. Can you make it generic? ([Solution](https://play.rust-lang.org/?gist=adf9b7360ee3ee008ee6840401172598&version=stable))

[Exercise](https://play.rust-lang.org/?gist=6888e2c75ef6a4ec08e61c6ab3d752a4&version=stable): create `repeat` iterator ([Solution](https://play.rust-lang.org/?gist=a1679f3bb0a1bb3fb7440a0369676d6d&version=stable))

* `for elt in iterator`
```Rust
let v = vec![1, 2, 3];
for elt in v { // consumes v: 1, 2, 3}
for elt in &v { // &1, &2, &3 }
for elt in &mut v { // &mut 1, &mut 2, &mut 3 }
```
* We haven't called anything on `v` (`&v`, `&mut v`). How did `for` make `v` into an iterator?
    - *Answer*: `IntoIterator` trait with `into_iter()` method. `for` loop invokes `IntoIterator::into_iter(v)` for us (syntatic sugar). More later!

* Converting from an iterator to a collection with `FromIterator` trait with one method `from_iter`:

    ```Rust
    use std::iter;
    use std::iter::FromIterator;

    let r = iter::repeat(1).take(5);
    let v = Vec::from_iter(r);

    let r = iter::repeat(1).take(5);
    assert_eq!(v, r.collect::<Vec<i32>>());
    ```

### Closer look into `IntoIterator`
```Rust
pub trait IntoIterator
where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

* Associated `type IntoIter` is responsible for holding the state.
    - Example [`vec::IntoIter` struct](https://doc.rust-lang.org/std/vec/struct.IntoIter.html)
* std lib: [`impl<I: Iterator> IntoIterator for I`](https://doc.rust-lang.org/src/core/iter/traits.rs.html#252) means all `Iterator`s are `IntoIterator`s!
    1. Exactly for this reason impling `Iterator` trait for a type, can use it with `for` loop
    2. For a collection with impled `IntoIterator` allows us to use it with `for` loop

[Challenging exercise](https://play.rust-lang.org/?gist=f31bbdc777f6a4168e6804e2a4678fbe&version=stable): impl various iterators for Stack i.e. impl methods `iter`, `iter_mut` and `IntoIterator` to pass all the tests ([Solution](https://play.rust-lang.org/?gist=a399db40d4edb5c1eacbef9483fcbe65&version=stable))

### Resources

* [Rust book(s)](https://doc.rust-lang.org/book/)
* [TMLL](http://cglab.ca/~abeinges/blah/too-many-lists/book/)
* [Iteration patterns for Result & Option](http://xion.io/post/code/rust-iter-patterns.html)
