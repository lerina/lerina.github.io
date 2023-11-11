

⇦ [lerina.github.io](../../../../index.html)  - [lerina](../../index.html) - [code](./index.html)  

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

```
hello_world/
├── Cargo.toml
└── src
    └── main.rs
```

This generates a main.rs file with a main function as starting point.

It is used to create stand alone projects aka binaries (bin)

2. Edit code (hello world is generated for you :-) ) 

> cd hello_world 
> vim src/main.rs

Change the message to be pronted out. For example

```rust
    println!("Hello,  Rust is fun!");
```

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







## A little about Computer memory, stack and heap

### [The Stack](https://computersciencewiki.org/index.php/Stack)

### [Stack and Heap Memory](https://courses.grainger.illinois.edu/cs225/fa2021/resources/stack-heap/)

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
