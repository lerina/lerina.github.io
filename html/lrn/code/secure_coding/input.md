# User input:

## Get a String from the console

To read user input from stdin:
- First bring into scope the io module from Rust standard library.  

```rust
use std::io;
```

This will give access to the console via `stdin()` and its method `read_line`

- Next create a mutable String variable which will serve as string buffer to capture the user input.

```rust 
let mut my_buffer = String::new();
```

- Next the mutable string buffer is mutably borrowed by read_line in order to capture the user input.

 ```rust
io::stdin().read_line(&mut my_buffer) // ... not done yet
```

We are not done. read_line returns a Result type (Ok, Err) so we need to hadle potential errors. We could just make the program crash using `unwrap()` but it is recommended to use `expect()` which lets use define an appropriate error message.

```rust
io::stdin()
        .read_line(&mut my_buffer)
        .expect("Failed to read line input");
```

- Finaly we can use the string. For instance we can output the data, using `println!`


```rust
use std::io;

fn main(){

    println!("Enter any number");
    let mut my_buffer = String::new();
    io::stdin()
        .read_line(&mut my_buffer)
        .expect("Failed to read input");

    println!("Your number is {}", my_buffer);
}
```

Recap::  
    
    use std::io; library provides ability to accept user input.
    fn main() is declaring new function
    println! macro prints string to the screen
    let mut is declaring a mutable variable
    String::new() is creating a empty string
    io::stdin().readline() is handling the standard input to get input from user
    .expect is handling any potential failure by crashing the program with an additional programmer defined error message.


## Get a Number from the console

Previously, we supposedly printed a number when in reality it was still a Sring. 

To get a numerical type we need to parse the string and have rust convert it into a desired number type (i32, f64, ...)

- First we trim the string buffer

```rust
let my_number = my_buffer.trim() //... not done yet
```

- Next we parse the input into the desired number type
explicitly with the turbofish syntax `parse::<T>()`

```rust
let my_number = my_buffer
                        .trim() 
                        .parse::<i32>() //... not done yet
```

or implicitly with type annotation

```rust
let my_number: i32 = my_buffer.trim() //... not done yet
```

- Finally we handle potential errors 

```rust
let my_number: i32 = my_buffer
                        .trim() 
                        .parse()
                        .expect("Please type a number");
```


- Get a i32 full example

```rust
use std::io;

fn main(){

    println!("Enter any number");
    let mut my_buffer = String::new();
    io::stdin()
        .read_line(&mut my_buffer)
        .expect("Failed to read input");
    
    let my_number: i32 = my_buffer
                        .trim() 
                        .parse()
                        .expect("Please type a number");

    println!("Your number plus one is {}", my_number + 1);
}
```


