# User input:

use std::io; library provides ability to accept user input.
fn main() is declaring new function
println! macro prints string to the screen
let mut is declaring a mutable variable
String::new() is creating a empty string
io::stdin().readline() is handling the standard input to get input from user
.expect is handling any potential failure 


```rust
use std::io;



fn main(){

    println!("Enter any number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    println!("Entered number is {}", number);

}
```
