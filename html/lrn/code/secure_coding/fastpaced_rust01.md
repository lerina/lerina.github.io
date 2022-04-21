⇦ [razafy.com](../../../../index.html)  - [lerina](../../index.html) - [code](./index.html) 

> Installing Rust on Linux, macOS, and Windows
>
> See [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) from The Rust Programming Language book at doc.rust-lang.org


# Level 01

## Data: take one 

When it comes to Software, everything is Data and the access to its storage.

Programming is about Data and its manupulation
Besides it value, all Data has a **type** whether it is explicit or not.

> Rust is a statically typed language, which means that it must know the types of all variables at compile time.  
> The compiler can usually infer what type we want to use based on the value and how we use it.  
_ rustbook

see also: [Rust built-in types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)

### Scalar Types
#### Integers

|Length	  | Signed| Unsigned |
|---------|-------|----------|
| 8-bit   | [i8](https://doc.rust-lang.org/std/primitive.i8.html)    | [u8](https://doc.rust-lang.org/std/primitive.u8.html)       |
| 16-bit  | [i16](https://doc.rust-lang.org/std/primitive.i16.html)   | [u16](https://doc.rust-lang.org/std/primitive.u16.html)      |
| 32-bit  | [i32](https://doc.rust-lang.org/std/primitive.i32.html)   | [u32](https://doc.rust-lang.org/std/primitive.u32.html)      |
| 64-bit  | [i64](https://doc.rust-lang.org/std/primitive.i64.html)   | [u64](https://doc.rust-lang.org/std/primitive.u64.html)      |
| 128-bit | [i128](https://doc.rust-lang.org/std/primitive.i128.html)  | [u128](https://doc.rust-lang.org/std/primitive.u128.html)     |
| arch	  | [isize](https://doc.rust-lang.org/std/primitive.isize.html) | [usize](https://doc.rust-lang.org/std/primitive.usize.html)    |

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

Rust’s floating-point types are [f32 (32 bits)](https://doc.rust-lang.org/std/primitive.f32.html) and [f64 (64 bits)](https://doc.rust-lang.org/std/primitive.f64.html){target="_blank"} .  

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

Also have a look at the Basic mathematical constants at [std::f32::consts](https://doc.rust-lang.org/std/f32/consts/index.html) and [std::f64::consts](https://doc.rust-lang.org/std/f64/consts/index.html)


#### bool

Booleans are one byte in size.
Two possible values: `true` and `false`

```rust
fn main() {
    let water_is_wet = true;
    let fire_is_wet = false;

    let f: bool = false; // with explicit type annotation
}
```

#### Character
`char` type represents a single character, a ‘Unicode scalar value’.

```rust
let infinity: char = '\u{267E}';
let the_one = '♾';

```

also see:
[char](https://doc.rust-lang.org/std/primitive.char.html)  
[unicode table](https://unicode-table.com/en/)  

### Mutability  
Variables are immutable by default, mutable explicitly

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
var01 = 12;     // integers 
var02 = 41;     // defaults to i32
```

However var01 is immutable whereas var02 can be reassigned a value ... **of the same type**.

```rust
// var01 = 42;  //ERROR var01 immutable

var02 = var01 + 1; // Ok
println!(var02); // 42
```

We can declare and define our variable at the same time

```rust
let var01 = 12;
let mut var02 = 41;
```

We can also explicitly annotate the type

```rust
let ascii: u8 = 255;
let mut var02: i64 = 41; // default would be i32 so we have to be explicit if we don't  want i32

```

### Collection Types

#### Array 
> The Array Type 
An array is a collection of elements of the same type allocated in a contiguous memory block.

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
A mutable array means the elements are mutable. But the array size cannot.


```rust
fn main() {
    let mut array: [u8; 3] = [0; 3];

    println!(";T -1: {}, {}, {}";, array[0], array[1], array[2]); // T -1: 0, 0, 0

    array[0] = 3;
    array[1] = 2;
    array[2] = 1;

    println!(";{}, {}, {} Lift Off!";, array[0], array[1], array[2]); // 3, 2, 1 Lift Off!
}
```
See vectors for a dynamic size array like type.

#### The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
Tuples have a fixed length: once declared, they cannot grow or shrink in size.  
_ rustbook

>  fixed-length collections of values of different types.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1); // mixed datatype

let middle = tup.1 // starts at index 0
println!(middle); // 6.4

let (x, y, z) = tup; // unpack
println!("The value of y is: {}", y);
```

#### Vector

A vector is a contiguous re-sizable array type.


### Slice, &str and Strings

#### Slice 
Slices act like temporary views into an array. 
It works by storing a reference to the first element and a length.
Hence all the elements must be of the same type.

```rust
let my_arr = [10, 20, 30, 40, 50];
let my_slice = &my_arr[2..];

println!("{}", my_slice[0]); // output: 30
println!("{}", my_arr[0]); // output: 10

```

#### string literal: `&str`

```rust
let name = "Rust"; // bind the string literal to variable `name`
let msg: &str = "Trust";

println!("In {} we {}", name, msg); // output: In Rust we Trust
```

A string literal &str is a slice `&[u8]`.
It is allocated on the stack as UTF-8 sequence.
The stack is parts of memory available to your code to use at runtime.
The size of &str is fixed, meaning it cannot be resized.

#### String: Allocation on the heap

A String is stored on the Heap as a vector of bytes `Vec<u8>` and is  encoded in UTF 8 sequence.
The heap is another parts of memory available to your code to use at runtime.
Heap allocation akkows for growable strings.

The data present in the string can be viewed using &str.

Creates a new empty String
```rust 
let s = String::new();

```

String from a literal string with String::from
```rust
let hello = String::from("Hello, world!");
```

also see:
[Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

## Input - Output: Terminal

### Writing to the Terminal

#### println!()

```rust
fn main() {
    println!("Hello world");
    
    let message = "Use `println!` to display text to the console";
    println!("The message is {}", message);


    let new_message = "Additional simpler formatting strings starting with Rust 1.58" 
    println!("The message is {new_message}");
}
```
For more on string interpolation :     // [Announcing Rust 1.58.0
](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html#captured-identifiers-in-format-strings)

see also:  
[print!()](https://doc.rust-lang.org/std/macro.print.html). It's equivalent to println!() except that a newline is not printed at the end of the message.  
[eprint!()](https://doc.rust-lang.org/std/macro.eprint.html) to print error and progress messages.

### Reading User Input from the Terminal

To receive keyboard input, we bring `std::io::stdin` into scope with the `use` keyword.
This gives us access to [read_line](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line) method.

`read_line` takes whatever the user types into standard input
and append that into a mutable string, so it takes a mutable reference to that string as an argument.
A mutable reference is noted `&mut`. This will be explained in the topic **Ownership and Borrowing** later.

#### user input is read into a String
```rust
use std::io::stdin;

fn main() {
    println!("Please enter some text: ");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("{}", user_input);
}
```
#### Converting Strings into other types

##### Convert to numbers

```rust
use std::io::stdin;

fn main() {
    println!("Please enter a number: ");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let my_int: u32 = user_input
                        .trim()
                        .parse()
                        .expect("Please type a number!");

    println!("{0} + {0} = {1}", my_int, my_int+my_int);
}
```

##### Convert to characters

```rust
    println!("Please enter a word: ");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let ch: Vec<_> = user_input.chars().collect();
    println!("the first char is {}", ch[0]);

    for ch in user_input.chars().collect::<Vec<char>>() {
        println!("{}", ch);
    }
    for ch in user_input.chars() {
        println!("{}", ch);
    }
    for (i,ch) in user_input.chars().enumerate() {
        println!("{} {}", i, ch);
    }
```
.chars() converts the string to a char iterator

.collect() converts the iterator to a collection

    


## Processing

### Basic mathematical operations
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

### user defined [Functions in Rust](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)

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

### Code block and scope
A scope is the range within a program for which an item is valid.

Like a lot of programming languages, a pair of brackets declares a block of code with its own scope.
Variables remains valid from the point at which it is declared until it goes out of scope

```rust
// This prints "in", then "out"
fn main() {
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}
```
Unlike most other languages brackets delimited blocks are also expressions. An expression evaluate to a value.

# Level 02

## Data: Take two  

### Ownership and Borrowing
Ownership is how Rust make memory safety guarantees without needing a garbage collector.
It is a set of rules that governs how a Rust program manages memory and verified at compile time.

Rust is a systems programming language. 
Understanding what [The Stack and the Heap](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap){target="\_blank"} are is important.

#### The stack

The stack and the heap are parts of memory available to your code to use at runtime.
The stack stores values in a last in, first out. But more importantly,  
data stored on the stack must have a known, fixed size. 

#### The heap 

Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
Putting data on the heap entails a request for a certain amount of space in memory. 
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, 
and returns a pointer, which is the address of that location.
The pointer to the heap is known, has fixed size and thus is stored on the stack.
The actual data will be on the head as its sizer may not be know at compile time and or it may grow or shrink during run time. Thus access to the data is via the pointer.



### Ownership 

Ownership Rules::
    
    
    Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.


## Input-Output: Working with files and directories  

### Reading a file  

### Writing to a file  

### Navigating the directory path  

## Processing

### Conditionals

### Loops

### Matching

# Level 03

## Data: Take three 

### Traits and behaviour

## Input-output: 

### Getting data from the CLI arguments

### Undestanding the Display trait

## Processing

### Implementing an Iterator

### hangman using Chars()

# Level 04

### Let's play with

#### user input from the console without the newline

Note: When using `print!()` keep in mind that stdout is frequently line-buffered by default so it may be necessary 
to use `io::stdout().flush()` to ensure the output is emitted immediately.

```rust
fn main() {
    use std::io::{stdin,stdout,Write};

    print!("Please enter some text: ");
    stdout().flush().unwrap();

    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Failed to read the line");

    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    // windows has \r too
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    println!("You typed: {}",s);
}
```

see also:  
[std::print](https://doc.rust-lang.org/std/macro.print.html)  
[std::io::Write::flush](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush)  
