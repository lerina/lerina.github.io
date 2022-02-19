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
[linux/mac](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos)  
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

[win](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-windows)  

[rustc is Rust's Compiler](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html).

[Cargo](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html) is Rust's build system and package manager that handles metadata and invokes rustc for you.

Modern projects depend on external packages and libraries. A package manager handles those packages and the dependencies they may rely upon.

## Hello World

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



