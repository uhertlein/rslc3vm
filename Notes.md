# Notes

## Rust for Dum^H^H^HC++ Programmers

- eco system
  - https://rust-lang.org/
  - The Rust Language Book https://doc.rust-lang.org/book/
  - `rustc` the Rust compiler
  - `rustfmt` code formatter
  - `cargo` build system, test and app runner, package management
    - `cargo build`
    - `cargo test <args>`
    - `cargo run <args>`
    - project meta-data and dependencies stored in `Cargo.toml` file
    - package management
      - so called `crates`
      - package repository at https://crates.io/
      - contains approx. 73,000 packages
      - approach with caution, similar issues as `Node.js`
  - `rustup` Rust eco system management
    - updates Rust and Cargo
    - can install multiple versions in parallel, e.g. `stable` and `nightly`
- basics
  - strict and explicit typing
    - no `int`, size must be explicit, e.g. `i32` or `u16`
    - same for `float`: `f64`
  - immutable by default
    ```rust
    let x = 5;
    x = 6; // error, x is immutable
    ```
  - moved by default
    - for complex types, i.e. not for int, literals, etc.
    ```rust
    let s1 = String::from("hello");
    let s2 = s1; // s1 becomes invalid

    fn foo(s: String) { ... } // string is moved to 'foo'
    foo(s2);
    ```
  - references
    ```rust
    fn foo(s: &String) { ... } // const-ref to string
    fn bar(s: &mut String) { ... } // non-const-ref to string
    ```
  - calls must be explicit
    ```rust
    let s = String::from("hello");
    foo(&s);
    bar(&mut s);
    ```
  - BUT! there can only be ONE MUTABLE REF AT A TIME!
    ```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // error, s is already 'borrowed' by r1
    ```
    - *can* have multiple immutable references simultaneously
    - *cannot* have immutable and mutable references simultaneously
- classes (or rather `struct`s)
  - no interitance
  - no standardized `ctor`
    - by convention a `new` static method is used
    - could be named anything you like, but expect people to hate you for it
  - member functions explicitly pass `self` aka `this`
    - `&self` is a `const` method
    - `&mut self` is a non-const method
    - shorthand for `self: &Self` and `self: &mut Self` respectively
- `Result<T,E>` and `Option<T>`
- `unwrap` takes the value of a `Result` or an `Option`
  - panics on error
  - use is discouraged, but nice to get started
- `enum` are complex objects
  - like a C enum, but each enum can also carry data
- `match` is quite nice
  - like `switch` but handles additional `enum` data as well
- `traits`
  - think `interface` class
  - can be implemented on any class
- development with VSCode is quite nice
  - code completion
  - error/warning messages
    - these are actually amazing in Rust!
  - debugging using `lldb`
