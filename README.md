# rslc3vm - Implements a simple LC3 virtual machine in Rust.

Based on the blog entry by Andrei Ciobanu:
https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c

Also, my first introduction to Rust :-)

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

## Samples

These two sample programs are from Andrei's code.

### `sum.lc3obj`

- reads two `u16` from the console
- adds them together
- writes the result to the console
- terminates

```
0xF026    //  1111 0000 0010 0110  TRAP tinu16      ;read an uint16_t in R0
0x1220    //  0001 0010 0010 0000  ADD R1,R0,x0     ;add contents of R0 to R1
0xF026    //  1111 0000 0010 0110  TRAP tinu16      ;read an uint16_t in R0
0x1240    //  0001 0010 0010 0000  ADD R1,R1,R0     ;add contents of R0 to R1
0x1060    //  0001 0000 0110 0000  ADD R0,R1,x0     ;add contents of R1 to R0
0xF027    //  1111 0000 0010 0111  TRAP toutu16     ;show the contents of R0 to stdout
0xF025    //  1111 0000 0010 0101  HALT             ;halt
```

### `simple_program.lc3obj`

- calculates the sum of the ten values in R1
- terminates

```
0x5260    //  0101 0010 0110 0000             AND R1,R1,x0    ;clear R1, to be used for the running sum               
0x5920    //  0101 1001 0010 0000             AND R4,R4,x0    ;clear R4, to be used as a counter                      
0x192A    //  0001 1001 0010 1010             ADD R4,R4,xB    ;load R4 with #10, the number of times to add           
0xE406    //  1110 0100 0000 0110             LEA R2,x7       ;load the starting address of the data          
0x6680    //  0110 0110 1000 0000     LOOP    LDR R3,R2,x0    ;load the next number to be added                       
0x14A1    //  0001 0100 1010 0001             ADD R2,R2,x1    ;increment the pointer
0x1243    //  0001 0010 0100 0011             ADD R1,R1,R3    ;add the next number to the running sum
0x193F    //  0001 1001 0011 1111             ADD R4,R4,x-1   ;decrement the counter
0x03FB    //  0000 0011 1111 1011             BRp LOOP        ;do it again if the counter is not yet zero
0xF025    //  1111 0000 0010 0101             HALT            ;halt

Data at 0x300A
0x0001 /* 1 */
0x0002 /* +2 = 3 */
0x0001 /* +1 = 4 */
0x0002 /* +2 = 6 */
0x0003 /* +3 = 9 */
0x0001 /* +1 = 10 */
0x0002 /* +2 = 12 */
0x0001 /* +1 = 13 */
0x0002 /* +2 = 15 */
0x0001 /* +1 = 16 */
```

