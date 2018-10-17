Declarative macros
------------------
*Author*: [Ehsan M. Kermani](https://ehsanmkermani.com/)

*Date: Oct. 17th, 2018*

### Basics

* **Metaprogramming** in Rust. Provides *syntax extension* mechanism
* Two types of macros:
    *  [**This presentation**] *Declarative macros* (aka Macros by example or Macro 2.0): `println!`, `vec!` 
    *  *Procedural macros*: `#[derive(...)]`

### Very quick recap: how macros are built (compiler side)

* *rustc* 
    * Rust source code --> Tokenization --> Token Tree --> Abstract Syntax Tree (AST) --> *Macro processing/expansion* --> ...
* **Macros are parsed into the AST**, they can **only** appear in positions where they are explicitly supported:
    * Patterns
    * Statements
    * Expressions
    * Items
    * `impl` Items

### macro_rules!

```rust
    macro_rules! $name {
            $rule0 ; // should be at least one rule
            $rule1 ;
            // …
            $ruleN ; // can omit the last semi-colon
    }
```

Each `rule` is of the form `($pattern) => { $expansion }` (like match arms). See [empty macro example](https://play.rust-lang.org/?gist=8ef6f5ca1b930a592f97744bd78d679a&version=stable&mode=debug&edition=2015)

#### Captures

* Patterns in `($pattern) => { $expansion }` can contain captures. Captures are turned into *variables* which can be use to substitute into the output
* `$capture_name:kind`
* The kind of capture can be one of the followings:
    * `item`: an item, a function, struct, module, etc.
    * `block`: a block of statements/expression surrounded by braces
    * `stmt`: a statement
    * `pat`: a pattern
    * `expr`: an expression
    * `ty`: a type
    * `ident`: an identifier
    * `path`: a path like `::std::mem::replace`
    * `meta`: a meta item, like `#[...]` or `!#[...]` attributes
    * `tt`: a single token tree

* [Basic capture examples](https://play.rust-lang.org/?gist=279a97bd4934a5bd50a135b9bcdb38c0&version=stable&mode=debug&edition=2015)

#### Repetition with [Kleene Star](https://en.wikipedia.org/wiki/Kleene_star)

* So far Rust's only *variadic* support is through repetitions operator in macros. For example, `vec![...]` can take any number of inputs up to *macro recursion limit* (controlable through `#![recursion_limit=" number "]` at the root) 
* General form: `$ ( ... ) optional_seperator required_repeat_control` where `optional_seperator` examples are `,`, `;` and `repeat_control` are either `*` (indicating zero of more repeats) or `+` (one or more repeats) up to now in stable Rust. In Editin 2018 `?` (at most once reptition) is available through [#![feature(macro_at_most_once_rep)]](https://doc.rust-lang.org/beta/unstable-book/language-features/macro-at-most-once-rep.html#macro_at_most_once_rep)
* Famous [vec! example](https://play.rust-lang.org/?gist=9d1ee9449512623ac7cf967eaa11e3e1&version=stable&mode=debug&edition=2015)
* [Exercise](https://play.rust-lang.org/?gist=a076d89181ea17accb23d829a4ddaabf&version=stable&mode=debug&edition=2015): One common use of macros is to *prevent repeating yourself*. For example, given an empty trait `Empty`, use declarative macros to implement `Empty` for all primitive integer types. ([Solution](https://play.rust-lang.org/?gist=3b199cfda84573e47ced067a2a12c125&version=stable&mode=debug&edition=2015))
* Exercise: What is the default recursion limit? ([Solution](https://play.rust-lang.org/?gist=4fc414600e5ade3812f1461ca04c713f&version=nightly&mode=debug&edition=2015))

#### Captures vs. Expansions

* [Run this first and see the difference](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=8752c308b0d22721b32b7df309e11f2f)
* `macro_rules!` stringifies the **AST expression node** while `stringify!` concatenates all the **tokens** into a string of tokens.

#### Import/Export macros

1) `#[macro_use]` attribute for *either* modules or external crates. Exmaple; `#[macro_use] extern crate lazy_static;` or control what to bring into scope `#[macro_use(name_of_macro)] crate_name`

NOTE: you can only `#[macro_use]` an external crate from the root module.

2) Export a macro from the current crate use `#[macro_export]` attribute along `macro_rules!` definition

#### Debugging

* `rustc` exposes [#![feature(trace_macros)]](https://doc.rust-lang.org/beta/unstable-book/language-features/trace-macros.html) and [#![feature(log_syntax)]](https://doc.rust-lang.org/nightly/nightly-rustc/syntax_ext/log_syntax/) to trace and output tokens passed to a macro. Same as compile flag `-Z trace-macros --pretty`
* [log_syntax example](https://play.rust-lang.org/?gist=1dd3a91850277bb8556ec32411467641&version=nightly&mode=debug&edition=2015)

#### Internal rules `@name`

* Any public macro must import all of it dependencies. To avoid pollution of global macro namespace, we hide macros inside the macro being exported.
* To specify the internal (private) macros inside of our *uber* (exported) macros, we can use internal rules. That's by convention use the token `@name` to name the internal macros in their definition.
* Example: instead of exporting `foo!` which would export it's dependency `as_expr!` as in 

```rust
#[macro_export]
macro_rules! as_expr {
    ($e:expr) => { $e }
    }
}

#[macro_export]
macro_rules! foo {
    ( $($tts:tt)* ) => {
        foo!(as_expr!($($tts)*)) // For the sake of example! does NOT compile though
    }
}
```

We can make `as_expr` an internal rule (won't be a separate macro though!) like

```rust
#[macro_export]
macro_rules! foo {
    (@as_expr $e:expr) => { $e }; // conventionally use @name
    ($($tts:tt)*) => {
        foo!(@as_expr $($tts)*)
    };
}
```
This way we won't export the unnecessary `as_expr!` macro to the global namespace.

#### Some examples as case study

* [quickcheck!](https://github.com/BurntSushi/quickcheck/blob/master/src/lib.rs#L47)
* Harder to follow (downside of macros!): 
    * [lazy_static!](https://github.com/rust-lang-nursery/lazy-static.rs/blob/master/src/lib.rs)
    * From [tarpc](https://google.github.io/tarpc/tarpc/index.html) crate: [service!](https://github.com/google/tarpc/blob/master/tarpc/src/macros.rs#L53)

### Resources

* [**Little book of macros**](https://danielkeep.github.io/tlborm/book/README.html): The best and advanced guide
* [Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html)
* [An overview of macros in Rust](https://words.steveklabnik.com/an-overview-of-macros-in-rust)
* [Rust book](https://doc.rust-lang.org/book/2018-edition/appendix-04-macros.html)
