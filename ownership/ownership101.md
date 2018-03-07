Ownership 101
---

* The biggest new feature of Rust
* Compile time check
* Implementation of *affine type system*: Each data/value has exactly **one owner.**
* *Create ownership* of a value with **variable binding** ` let var = value; ` (immutable binding) or ` let mut var = value; ` (mutable binding)

### Move: Copy vs. non-Copy
```rust
let x = 1;
println!("x: {}", x);
let y = x; // y has a copy of x value
println!("x: {}", x); // Ok
```
* Primitives are copied by default (*impl Copy trait*)

```rust
let v = vec![1,2,3]; // Vec doesn't impl Copy
println!("{:?}", v); // Ok
let v1 = v; // v1 is the owner. v is **moved**
println!("{:?}", v); // Compile Error: value used here after move
```
However,

```rust
let v = vec![1, 2, 3]; // Vec impl Clone instead. Heap thing!
println!("{:?}", v); // Ok
let v1 = v.clone(); // v1 owns a clone of v data
println!("{:?}", v); // Ok
```
### Scoping rule

* When binding goes out of scope the value is dropped/destroyed/deallocated (Welcome to Rust memory management)

```rust
let x = 1;
    {
        let y = 2;
        println!("x + y: {}", x + y); //Ok
    } // --> y goes out of scope and 2 is destroyed
println!("x + y: {}", x + y); //Error cannot find y
```

### Borrowing

* Shared (immutable) reference: **&**
* Mutable reference: **&mut**

```rust
let v = vec![1, 2, 3]; // Vec impl Clone instead. Heap thing!
println!("{:?}", v); // Ok
let v1 = &v; // v1 owns a reference to v's data
println!("{:?}", v); // Ok
```

Note: Scoping rule applied to references as well
(general lifetime)

```rust
let v = vec![1, 2, 3];
    {
        let v1 = &v;
        println!("v[0] + v1[0]: {}", v[0] + v1[0]); //Ok
    } // --> v1 goes out of scope so does the reference
println!("v[0] + v1[0]: {}", v[0] + v1[0]) // Error
```

#### Notorious move errors

* Ownership cannot be transferred while there's a reference to it (prevent dangling pointer)

```rust
let v = vec![1, 2, 3];
let v_ref = &v;
let v1 = v; // Compile Error: cannot move out of
            // `v` b/c it's borrowed
```

* Cannot move out of the borrowed content

```rust
let mut v = vec![1, 2, 3];
let v1 = &mut v;
let mut v2 = *v1; // Compile Error:
                  // cannot move out of the borrowed content.
                  // Consider using a reference
v2.push(4);
```

##### Summary of borrowing rules (memory safety)

* Cannot borrow something that doesn't exist! (obvious)

* Either one can happen not both:

    1) One mutable borrow (no data race)
    2) Multiple immutable borrows

```rust
let mut v = vec![1, 2, 3];
    for elt in &v {
        v.pop(); // pop needs a mutable borrow
        // ERROR: cannot borrow `v` as mutable because
        // it is also borrowed as immutable
    }
```

```rust
let mut v = vec![1, 2, 3];
    for elt in &mut v {
        v.pop(); // pop needs a mutable borrow
        // Error: two mutable borrows!
    }
```

### Exercises:

Try to see whether the code in each problem compiles or not and where the problem is. Is it possible to change the code slightly to make it compile?!

* [problem1](https://play.rust-lang.org/?gist=2fb835b895cabd22f8a6e5704e2ad4c5&version=stable)
* [problem2](https://play.rust-lang.org/?gist=0c1164b14fb09f795d7396fb8cc6a5d3&version=stable)
* [problem3](https://play.rust-lang.org/?gist=5010f9d84b0fe9eeff8d89026a217266&version=stable)
* [problem4](https://play.rust-lang.org/?gist=cef269adcc1e66ec4b274dca2c0fbf68&version=stable)
* [problem5](https://play.rust-lang.org/?gist=14364e86f432e48f09cbf00a62ea796e&version=stable): [solution1](https://play.rust-lang.org/?gist=ef4003c05137ee48a8177874c330c744&version=stable), [solution2](https://play.rust-lang.org/?gist=58a8136281d2ffe0292ecf2d436d478a&version=stable)
* [problem6](https://play.rust-lang.org/?gist=792afade0932bce8ee69ab51eb8b24b0&version=stable): [solution](https://play.rust-lang.org/?gist=2c5a71ff905d98a7bc5a5880e69de87e&version=stable)
* [problem7](https://play.rust-lang.org/?gist=f11de5d9a6ac4e1a4ba2a0712a1d717b&version=stable): [solution](https://play.rust-lang.org/?gist=4a1507233f62609614f3f22f6de6d203&version=stable)

#### References

* [Rust book v2](https://doc.rust-lang.org/book/second-edition/ch04-00-understanding-ownership.html)
* [CIS 198: Rust Programming](http://cis198-2016s.github.io/slides/01/#1)
* [TMLL](http://cglab.ca/~abeinges/blah/too-many-lists/book/)
* [users.rust-lang.org](https://users.rust-lang.org/)
* Stackoverflow
