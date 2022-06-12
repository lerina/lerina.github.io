# User input:

 To read user input from stdin:
 First bring into scope the io module from Rust standard library. 


Let's create a mutable variable name which is a string and we pass a reference to that to read_line. The reason read_line takes a mutable reference to a string buffer is because it will use this buffer to fill in the data that is entered by the user.

 We then create an instance of stdin using the stdin() function. This comes with a method read_line. Read_line takes a mutable reference to a string buffer.


After the user is done entering its data, 

we can output the data using println! We save the file and run the code. We'll notice that the compiler warns us about the fact that read_line returns something of type result, which can possibly be an error. It also tells us that the error should be handled.

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
