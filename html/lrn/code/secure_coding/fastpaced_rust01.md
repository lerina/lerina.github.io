⇦ [razafy.com](../../../../index.html)  - [lerina](../../index.html) - [code](./index.html)  

# The Rust programming language Level 01: Building a solid foundation

- Rust is a multi-paradigm system programming language. You can write it in a functional way. You can write in in an object-oriented way.
It is neither functional nor OOP. its a bit of both worlds.

- Systems programming is used for system software. System software provides a platform for other software. 
Rust is a language used for systems programming. It provides high performance and easy access to the underlying hardware.

- Rust is a modern language that focuses on safety. Regular usage of Rust is in Safe rust mode by default. In this mode you will *never* have to worry about type-safety or memory safety.
You will never endure a dangling pointer, a use-after-free, or any other kind of *Undefined* Behavior. [source Rustonomicon](./)

- Rust makes concurrency safe.

## Install Rust
On [linux/mac](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos)  
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

NOTE:  
Installing Rust on Windows is more involved. See: 
[Installing rustup on Windows](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-windows)  

Most people will go for the easiest way to acquire the build tools by installing [VS code](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
and the pluggins for Rust. 

[Rust install page at rust-lang.org](https://www.rust-lang.org/tools/install)

### rustc and cargo

[rustc is Rust's Compiler](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html).

[Cargo](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html) is Rust's build system and package manager that handles metadata and invokes rustc for you.

Modern projects depend on external packages and libraries. A package manager handles those packages and the dependencies they may rely upon.

## First program: Hello World

### Create an executable (bin) with `rustc`
Create directory

> mkdir hello_world

Create program entry file

> vim main.rs

Edit file
```rust
// main.rs

// program entry function
fn main() {
    // by convention use 4 space indentations
}
```

`fn` is the keyword to declare a function in Rust

Print hello world
```rust
fn main() {
    println!("Hello, world!);
}
```
Compile the program 

> rustc main.rs

Run the program  

> ./main

### Create an executable (bin) with `cargo`

1. Create a new project

> cargo new hello_world

This generates a main.rs file with a main function as starting point.

Tt is used to create stand alone projects.

2. edit code (hello world is generated for you :-) ) 

> cd hello_world 
> vim src/main.rs

3. Compile

> cargo build

4. Run

> cargo run

5. [more about cargo](https://doc.rust-lang.org/cargo/guide/index.html)

- [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
- [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [cargo clean](https://doc.rust-lang.org/cargo/commands/cargo-clean.html)



### create a Rust library

> cargo new my_lib --lib

This generates a lib.rs file 
It is used to create a library to be consumed by some other project 

We ll see later why and how to use libraries

## Programming is about Data and its manupulation
Besides it value, all Data has a **type** whether it is explicit or not.

### [Rust built-in types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)

> Rust is a statically typed language, which means that it must know the types of all variables at compile time.  
> The compiler can usually infer what type we want to use based on the value and how we use it.  
_ rustbook

#### Integer Types in Rust

|Length	  | Signed| Unsigned |
|---------|-------|----------|
| 8-bit   | i8    | u8       |
| 16-bit  | i16   | u16      |
| 32-bit  | i32   | u32      |
| 64-bit  | i64   | u64      |
| 128-bit | i128  | u128     |
| arch	  | isize | usize    |

integers default to [i32](https://doc.rust-lang.org/std/primitive.i32.html#implementations)

> isize and usize types depend on the architecture of the computer your program is running on,  
> which is denoted in the table as “arch”:  
> 64 bits if you’re on a 64-bit architecture and 
> 32 bits if you’re on a 32-bit architecture.    
_ rustbook


##### Integer Literals in Rust
> You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
_ rustbook


| Number literals   | Example       |
|-------------------|---------------|
| Decimal           | 98_222        |
| Hex               | 0xff          |
| Octal             | 0o77          |
| Binary            | 0b1111_0000   |
| Byte (u8 only)    | b'A'          |

#### Floats
Rust’s floating-point types are f32 (32 bits) and f64 (64 bits).  
All floating-point types are signed.

Floats defaults to [f64](https://doc.rust-lang.org/std/primitive.f64.html#implementations)

>  The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.  
_ rustbook



```rust
fn main() {
    let x = 42.0; // f64
    let y: f32 = 42.0; // f32
}
```

#### Basic mathematical operations
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

NOTE:  
The common power `^` or `**` is not used in rust.  
Instead the numerical type has a method for it.  
such as [powi](https://doc.rust-lang.org/std/primitive.f32.html#method.powi) and 
[powf](https://doc.rust-lang.org/std/primitive.f32.html#method.powf) for the `f32` type.

```rust
let x: f32 = 2.0; 
println!( x.powi(2) );
println!( x.powf(2.0) );
```

#### bool
Booleans are one byte in size.

#### Tuple and Array Types
> The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.  
_ rustbook


```rust
let tup: (i32, f64, u8) = (500, 6.4, 1); // mixed datatype

let middle = tup.1 // starts at index 0
println!(middle); // 6.4

let (x, y, z) = tup; // unpack
println!("The value of y is: {}", y);
```
#### Array 
> The Array Type 

Every element of an array must have the same type. 
```rust
let my_array = [0.07, 1.2, 2.0, 3.1415, 4.2];
```

Arrays in Rust have a fixed length.
```rust
let a: [i32; 5]; // type and size of the array
a = [1, 2, 3, 4, 5];
```

We can initialize an array to contain the same value for each element
```rust
let five_ones = [1; 5]; // [1, 1, 1, 1, 1]
```

You can access elements of an array using indexing
```rust
let a = [1, 2, 3, 4, 5];
let first = a[0]; // 1
let second = a[1]; // 2
```
### Variables are immutable by default, mutable explicitly

> When a variable is immutable, once a value is bound to a name, you can’t change that value.  
_ rustbook

#### `mut` keyword

You can make them mutable by adding mut in front of the variable name. 
Adding mut also conveys intent to future readers of the code by indicating

```rust
let var01;
let mut var02;
```

Once we bind a value to var01 or var02 they will acquire a type and a value. 

```rust
var01 = 12;
var02 = 41;
```

However var01 is immutable whereas var02 can be reassigned a value ... **of the same type**.

```rust
// var01 = 42;  ERROR var01 immutable
var02 = var01 + 1; // Ok
println!(var02); // 42
```

We can declare and define our viriable at the same time

```rust
let var01 = 12;
let mut var02 = 41;
```

We can also explicitly annotate the type

```rust
let ascii: u8 = 255;
let mut var02: i64 = 41; // default would be i32 so we have to be explicit if we don't  want i32
```

### [Functions in Rust](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)
 The `fn` keyword is used to declare new functions.

> Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.  
_ rustbook

```rust
// program entry function is main
fn main() {
    println!("Hello, world!");

    another_function(); //user defined function called (used) here
}

// user defined function declared and defined here
fn another_function() {
    println!("Another function.");
}

```

#### Function parameters and call arguments
> We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.  
_ rustbook  

```rust
fn main() {
    let x = 20;
    let y = 1;
    let answer = double_me(x) + double_me(y);
    
    println!("{} is the answer", answer);
}

fn double_me(a: i32) -> i32 {
    let x = a;

    x*2  // last line without `;` same as `return x*2;`
}
```

let's make a better usage of this function and also remove unnecessary bindings 
inside the fuction

```rust
fn main() {
    let x = 20;
    let y = 1;
    let answer = double_me(x + y);
    
    println!("{} is the answer", answer);
}

fn double_me(a: i32) -> i32 {

    a*2
}
```

Lets include an add function

```rust
fn main() {
    let x = 20;
    let y = 1;
    let answer = add_me(double_me(x),  double_me(y));
    
    println!("{} is the answer", answer);
}

fn  add_me(x: i32, y: i32) -> i32 {

    x+y
}

fn double_me(a: i32) -> i32 {

    a*2
}
```




## A little about Computer memory, stack and heap

[source](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html)


### Taking a peek at the binary
Lets see what is under the hood

> vim main.rs

```rust
// filename: main.rs

fn main() {
    let x = 20;
    let y = 1;
    let answer = add_me(double_me(x),  double_me(y));
    
    println!("{} is the answer", answer);
}

fn  add_me(x: i32, y: i32) -> i32 {

    x+y
}

fn double_me(a: i32) -> i32 {

    a*2
}
```

> rustc main.rs
> xxd -g1 main

```

```
