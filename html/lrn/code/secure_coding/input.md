
## Reading User Input

To receive keyboard input, Rust provides terminal input functions in std::io::stdin . 
You can find read_line as std::io::stdin::read_line
`read_line` takes whatever the user types into standard input
and append that into a mutable string, so it takes that string as an argument.

```rust
use std::io::stdin;

let mut user_input = String::new();
stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");

println!("{}", user_input);
```
