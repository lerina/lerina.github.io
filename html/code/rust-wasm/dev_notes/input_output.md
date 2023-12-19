<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# Rust Input - Output

[source](https://fitech101.aalto.fi/programming-languages/rust/8-interaction-input-and-os/#:~:text=To%20read%20user%20input%20in,written%20on%20the%20command%20line)

## Input from a terminal/console

### Read a line

- Production code

```rust
use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {error}"),
    }
}
```

- Spike or example code

```rust
use std::io;

fn main() {
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input).unwrap();
    println!("{} bytes read", n);
    println!("{}", input);
}
```


You can cause an error on purpose with `read_line` by passing invalid UTF-8 as input 
from e.g. an image file. 
File contents can be redirected to the standard input of a program process with the syntax `command < file`.

```sh
cargo run < my-image.png
```

Note:  
The `read_line` method that we used to read a line from `stdin` is a *blocking* function. 
It will read from the underlying input stream until it encounters a newline `\n` 
(pressing Enter when inserting input in an interactive command line environment) 
or an `EOF` i.e. end of file marker.  
In other words, calling `read_line` will wait until a new line appears, 
which is the case when we press Enter in an interactive command line program, or the input stream ends.


### multiple-line input

We can get an iterator over the lines in the standard input stream by using the lines method of `Stdin`.

```rust
use std::io;
fn main() {
    println!("Input 3 numbers");
    let mut numbers = vec![];
    for line in io::stdin().lines().take(3) {
        numbers.push(line);
    }
    println!("The numbers were {numbers:?}");
}
```

Here we used the iterator's `take` method to stop the iterator after going through three lines. 
Otherwise the program would run forever waiting for more lines when executed in a command line 

As an alternative to `read_line`. We may take one line from the iterator given by calling lines.

```rust
use std::io;
fn main() {
    println!("Input one line");
    let line = io::stdin().lines().next().unwrap().unwrap();
    println!("The line was {line:?}");
}
```

Here we see something that can be a bit unpleasant to the eye: two unwraps in a row. 

The first unwrap is on the Option returned by next (the next line might not exist) that needs to be handled for any iterator.  
The second unwrap is on the Result for handling invalid input.

With the `read_line` method, we needed only one unwrap.  
With this approach, we don't need to trim the string and we don't need to define a mutable string to store the input. 

Choose your poison.

### Parsing input into numbers

Next, we have a slightly more complex example than just reading input.  
We will read two numbers from the standard input and print out their sum.

To parse a string into a number, we can use the parse method on the str type.

```rust
use std::io;
fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();

    line.parse().unwrap()
}

fn main() {
    println!("Input a number");
    let number1 = read_i32();
    println!("Input another number");
    let number2 = read_i32();
    println!("{number1} + {number2} = {}", number1 + number2);
}
```

Parsing the string into `i32` returns a `Result`, so we need to handle that too in addition to all the possible errors from reading input. 
The resulting code is a bit verbose, but it is necessary to keep the compiler happy.

The error type of the `Result` returned by the parse method is `ParseIntError`, which represents multiple different error kinds that are defined in the enum `IntErrorKind`. For example, parsing a string that contains invalid characters will result in an `IntErrorKind::InvalidDigit`.

We can handle the different error kinds by first getting the kind enum from the error with `kind()`, and then using the match expression to handle the enum variants.

```rust
use std::io;
use std::num::IntErrorKind;
fn main() {
    let mut line = String::new();
    loop {
        println!("Please enter a number");
        io::stdin().read_line(&mut line).unwrap();
        if let Err(e) = line.parse::<i32>() {
            match e.kind() {
                IntErrorKind::Empty => {
                    println!("Exiting...");
                    break;
                }
                IntErrorKind::InvalidDigit => {
                    println!("Invalid digit, try again")
                }
                error => {
                    panic!("Unexpected error {error:?}")
                }
            }
        }
        line.clear()
    }
}
```

## Command line arguments and environment variables

When running a program from a command line, we can provide arguments to the program after giving the program name, 
like run for cargo in cargo run.  
Passing arguments to a program is not that different from passing arguments (i.e. values) to a parameterized function.

The following example works like the "echo" program, it prints out the arguments it is given.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

We first bring into scope the `std::env` module, which contains various functions for getting information about the environment of the process (the instance of a computer program being executed).  
We then get an iterator of the program arguments with the `std::env::args` function and collect the iterator values before printing them out.

Running the above example with `cargo run -- Hello World` in a terminal prints out `["target/debug/echo", "Hello", "World"]` (assuming the project name is echo). 

The first argument (at index 0) is the path to the program.  
The rest of the arguments are the arguments passed to the program.  
We need to include the `--` argument to tell cargo that the arguments after the `--` are not for cargo run, 
but for the program that is run.


Below, we have a program that reads two arguments and multiplies them together. 
It gets the arguments by using the indices 1 and 2, and then parses them into f64s. 

It doesn't handle the case where the user doesn't provide two arguments very nicely  but provides an obscure message instead. 
With `get`, we can provide a better error explanation or a default value to use when the index is out of bounds

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number1 = args[1].parse::<f64>().unwrap();
    let number2 = args[2].parse::<f64>().unwrap();
    let product = number1 * number2;
    println!("{product}");
}
```

With this sort of an application that simply multiplies its values, 
we could easily do much better than multiplying only two values. 
We can handle an arbitrary number of arguments by using the `product` method directly on the args iterator.

```rust
use std::env;

fn main() {
    let product = env::args()
        .map(|arg| arg.parse::<f64>().expect("Could not parse argument to number"))
        .product::<f64>();
    println!("{product}");
}
```

We'll want to ignore the filename argument at the beginning of the iterator though for our multiplication. For this, the `skip` method of the iterator comes in handy. The product method returns 1.0 if the iterator is empty, which makes this approach safe to use also when providing no arguments.

### Environment variables

Environment variables are variables defined in a shell's environment that programs inherit when they are run in the shell. Environment variables are often used to configure a program. For example, cargo uses the `RUST_BACKTRACE` environment variable for enabling backtrace for Rust runtime errors.

In Rust, we can access environment variables with the `env::vars` function. It returns an iterator of environment variables names and values as tuples, which we can collect into a hash map for further use.

Let's see what environment variables are available to us in our program.

```rust
use std::env;
use std::collections::HashMap;

fn main() {
    let vars: HashMap<String, String> = env::vars().collect();
    println!("{vars:#?}");
}
```

## Managing files and directories

An operating system (OS) manages the resources our computer can use: memory, disks, networking, filesystem and drawing to the screen. 

Next, we will look at how to interact with the operating system by reading and writing files within Rust code

### Reading files

> A quick look.

#### Checking if a file exists

```rust
fn main() {
    let fp = "/etc/hosts";
    let b = std::path::Path::new(fp).exists();
    println!("{}: {}", fp, b);

    let fp = "/etc/kittens";
    let b = std::path::Path::new(fp).exists();
    println!("{}: {}", fp, b);
}
```

output:

```sh
/etc/hosts: true
/etc/kittens: false
```

#### Reading the file as a string

```rust
use std::fs;
fn main() {
    let file_contents = fs::read_to_string("info.txt")
        .expect("LogRocket: Should have been able to read the file");
    println!("info.txt context =\n{file_contents}");
}
```

#### Reading a file as a vector

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("info.txt")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("File contents: {:?}", contents);

    Ok(())
}
```

#### Reading a file with a buffer

Reading a Rust file with a buffer can be more efficient than reading the entire file at once 
because it allows the program to process the data in chunks. 
This can be particularly useful for large files that may not fit in memory in their entirety.

To read a file using buffer, you can use the `BufReader` struct and the `BufRead` trait:

```rust
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("info.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
```

The `BufReader` reads the file in chunks (or “buffers”) rather than reading it all at once, which can be more efficient for large files.

---

> A longer view

Reading a file requires knowing the path to it. In Unix-like operating systems, like Linux, the directories and files of the directory structure are separated by slashes `/` in the path. 
In Windows, the directories and files are separated by backslashes `\.` 

A path can start with `./` to indicate that it is relative to the directory the program is being run at. 
Let's say we are running the following program from the path `/home/user/project/`. 
We can use the `std::fs::read` function to read the contents of a file into a vector of bytes (`Vec<u8>`). We can then convert the bytes into a string with the `String::from_utf8` function.

```rust
use std::fs;

fn main() {
    let bytes = fs::read("./src/main.rs").expect("Unable to read file");
    println!("file contents as bytes:");
    println!("{bytes:?}");
    let string = String::from_utf8(bytes).expect("Invalid UTF-8");
    println!("file contents as string:");
    println!("{string}");
}
```

Calling `fs::read("./src/main.rs")`` will try to read the ``/home/user/project/src/main.rs` file. 
If the file exists, and the user's permissions are sufficient, the contents of that file will be saved in the bytes variable. 

Try to modify the path in the above example to a file that does not exist, e.g. `/src/main.rs`, to see a runtime error.

In the usual case, we want to read a file and convert its contents to a string, like we did with `fs::read` and `String::from_utf8`. 

Being such a common operation `fs` has a function for just that `fs::read_to_string`.

```rust
use std::fs;

fn main() {
    let data = fs::read_to_string("./src/main.rs").expect("Unable to read file to UTF-8 String");
    println!("{data}");
}
```

### Writing to a file

We can use the standard library function `fs::write` to write a string to a file in the specified path. 
The `fs::write` first creates a file (if it doesn't already exist) and then writes to the file 
by combining the `fs::File::create` and `io::Write::write_all` functions into one convenient function.

```rust
use std::fs;

fn main() {
    let data = "This will be deleted anyway";
    fs::write("/tmp/testing", data).expect("Unable to write file");

    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}
```

We can also pass a *byte vector* to `fs::write` to write any binary data to a file, like the contents of an image or a video.

```rust
use std::fs;

fn main() {
    let random_bytes = vec![255, 23, 42, 0, 255, 235, 222, 10];
    fs::write("/tmp/testing", random_bytes).expect("Unable to write file");

    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}
```

Note that the `fs::write` function will **overwrite** the file if it already exists. 

To avoid overwriting an existing file, we can check its existence before writing to it with the `path::Path` struct and its exists method.

```rust
use std::fs;
use std::path::Path;

fn main() {
    let data = "This will be deleted anyway";
    if !Path::new("/tmp/testing").exists() {
        fs::write("/tmp/testing", data).expect("Unable to write file");
    }

    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}
```

### Appending to a file

Rust does not provide a convenience function for appending to a file, but we can use the `fs::OpenOptions` struct to open the file in append mode.  
We can then append text to the file using the `writeln!` macro, which is a convenience macro for writing a string 
and a newline to a buffer (there is also `write!` when we don't want a new line at the end). 
Using the macro requires an additional method for `OpenOptions` though, 
which can be added by importing the trait `std::io::Write` (the compiler kindly hints us to do so in case we forget).

```rust
use std::fs;

fn main() {
    fs::write("/tmp/testing", "").expect("Unable to write file");

    let data = "This will be deleted anyway";
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/tmp/testing")
        .expect("Unable to open file");
    writeln!(file, "{}", data).expect("Unable to write data");
    writeln!(file, "{}", data).expect("Unable to write data");
    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}
```

### Removing a file

Removing a file in Rust code is as straightforward as creating or overwriting them with `fs::write` with the `fs::remove_file` function. This function will return an error if the given path doesn't exist, the path is a directory, or the user doesn't have permission to remove the file.

```rust
use std::fs;

fn main() {
    let data = "This will be deleted anyway";
    fs::write("/tmp/testing", data).expect("Unable to write file");
    fs::remove_file("/tmp/testing").expect("Unable to remove file");
}
```

#### Reading a file at Compile time

We can also read files at compile time with the `include_str!` macro. 
The `include_str!` macro will read the file at compile time and include the contents of the file as a string. 
The path of the read file is located relative to the file where the macro is called.

An invalid path will cause a compile time error. On the other hand, the file will not be read at runtime so the file does not need to exist when the program is run.

```rust
use std::fs;

fn main() {
    let cargo_toml = include_str!("./Cargo.toml");
    fs::remove_file("./Cargo.toml").expect("Unable to remove Cargo.toml");
    println!("Project toml:
{cargo_toml}");
}
```

---

### `?` try operator

Rust can often be verbose, but it doesn't have to be always. 
Let's have a look at a simple backup function that leverages the `fs::read_to_string` function 
along with the `fs::write` to create a backup copy of a file.

```rust
use std::{io, fs};

fn backup(filename: &str) -> Result<(), io::Error> {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => return Err(e),
    };
    let backup = format!("// this is a backup\n{contents}");

    let backup_path = format!("{filename}.backup");
    match fs::write(&backup_path, backup) {
        Ok(_) => println!("Contents of {filename} successfully backed up to {backup_path}!"),
        Err(e) => return Err(e),
    };
    Ok(())
}

fn main() {
    if let Err(e) = backup("main.rs") {
        println!("Error: {e}");
    }
}
```
Even though the function does not do that much, it contains quite a lot of code. 

We could of course use the more concise `expect` or `unwrap` functions to handle the error by causing a runtime panic, but often we want to propagate the error back to the caller instead.  
This way the caller can choose how to handle the error, and that is also the way most programming languages work implicitly.

To make error handling simpler, Rust provides a way to propagate errors by using the `?` (pronounced try) operator. With it, our backup function can look rather nice and concise.

```rust
use std::{io, fs};

fn backup(filename: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(filename)?;               // <--------- try!
    let backup = format!("// this is a backup\n{contents}");

    let backup_path = format!("{filename}.backup");
    fs::write(&backup_path, backup)?;                           // <--------- try!
    println!("Contents of {filename} successfully backed up to {backup_path}!");
    Ok(())
}

fn main() {
    if let Err(e) = backup("main.rs") {
        println!("Error: {e}");
    }
}
```

The `?` operator works for both Options and Results by checking 
if the value in front of it is `None` or `Err` and returning the error prematurely. 
If the value is `Some` or `Ok`, `?` unwraps the value.

Note that using `?` requires the function to return either an `Option` or a `Result`, 
and the propagated value needs to match the return type.


#### Handling a file I/O error in Rust

Wondering how to handle an error when a Rust file cannot be opened or read? 
You can use the `std::io::Result` type and the `?` operator.

Here’s an example of handling a specific I/O error:


```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = match File::open("info.txt") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok(());
                }
                _ => return Err(error),
            }
        }
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("File contents: {:?}", contents);

    Ok(())
}
```
---

### Listing directories

Listing directories can be a bit more complicated than reading and writing files because we have more possible errors to deal with.   
Although with the help of the `?` (try) operator, we can streamline through most of them by propagating the errors back to the caller.  

Using `std::fs::read_dir` we can get an iterator over all the files and directories at the path provided as argument.  

The `read_dir` function returns an `io::Result<ReadDir>`, which we can iterate over, but iterating over a `Result` only gives the wrapped value if it is `Ok`.  

<aside>
Note:  
<br/>
We need to give the function a return type of `Result` 
or `Option` to be able to use `?`.
</aside>

We want to iterate over the `ReadDir` instead to get individual `DirEntrys`, 
which contain information about the entry, like whether it is a directory or a file.

Here is a good place to try the `?` operator to get the value inside the result and propagate the error to the caller if it is an `Err`. 


```rust
use std::fs::read_dir;

fn main() {
    for dir_entry in read_dir(".") {
        println!("{dir_entry:?}"); // ReadDir(".") ???
    }
}
```

The `ReadDir` iterator gives us `io::Result<DirEntry>`s, which is interesting because we have just handled the errors from `read_dir`. The reason is that the `ReadDir` iterator doesn't contain the contents of the directory in any way. When the `for loop` calls `next()` during each iteration, the program gets the next `DirEntry` from the operating system. As with anything that interacts with the operating system, this may also fail.

But now we finally have access to the `DirEntry`s which have many useful methods, like `file_name`, `path` and `metadata`. We can use the `metadata` method on a `DirEntry` to get more information about the *file* or *directory*. `metadata` also interacts with the operating system, thus requiring us to handle potential errors.

We can see for example, which entries are directories and how big each file is.

```rust
use std::fs::read_dir;
fn main() -> std::io::Result<()> {
    for entry in read_dir(".")? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let filename = entry.file_name();
        println!(
            "{filename:?}: is_dir = {}, len = {}, created = {:?}",
            metadata.is_dir(),
            metadata.len(),
            metadata.created()
        );
    }
    Ok(())
}
```

Here we also use the `Result` type from the `io` module as the return type, which works with the `?` operator because it is just a regular `Result` with the error type already set to `io::Error`.

The `metadata` for a file or directory can be accessed also by using the `fs::metadata` function, which takes a `path` as argument. It too returns a `Result` in case the `path` doesn't exist or the program doesn't have permission to access it.

```rust
use std::fs;
fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("./src")?;
    println!("is_dir = {}, len = {}, created = {:?}", 
                metadata.is_dir(), metadata.len(), metadata.created()
    );

    Ok(())
}
```

When we need to create a new directory, Rust standard library provides the functions `fs::create_dir` and `fs::create_dir_all`.  
The `create_dir` function will return an error if a directory with the same name already exists or if one of it's parent directories doesn't exist.   
The `create_dir_all` function will create all the parent directories if they don't exist and will return `Ok` even when all directories in a given path exists.

For removing directories, Rust standard library provides the functions `fs::remove_dir` and `fs::remove_dir_all`.  
The `remove_dir` function only works for empty directories,   
while `remove_dir_all` recursively removes all the files and directories inside the directory before removing the directory itself.

```rust
use std::fs;
fn main() -> std::io::Result<()> {
    println!("{:?}", fs::create_dir("new_dir/subdir"));
    println!("{:?}", fs::create_dir_all("new_dir/subdir"));
    println!("{:?}", fs::remove_dir("new_dir"));
    println!("{:?}", fs::remove_dir_all("new_dir"));
    Ok(())
}
```

Like the file modification and removal functions, these all return an error on failure due to e.g. insufficient permissions.


### OsSrting

The `file_name` method of `DirEntry` doesn't return a `String` or a `&str` which are already familiar to us, but rather an `std::ffi::OsString`.   

`OsString` is a compatibility feature in Rust which can store data in the different encodings different operating systems use — an `OsString` may contain `non-valid UTF-8` unlike a `String`.

Let's say we want to format our file metadata listing from previous example with padding (`:>20`) for more pleasant reading. An OsString can't be displayed without debug format (`:?`) and padding doesn't work on debug format, so we need to get a `String` or `&str` from the `OsString`.

```rust
fn main() {
  let os_string = std::ffi::OsString::from("chicken");
  println!("{os_string:>20?}!"); // chicken!
}
```

The simplest way to convert an `OsString` to a `&str` is to use the `to_string_lossy` method, which returns a `&str` where invalid unicode characters are replaced with `�`. 
This method technically returns a *Clone-on-write* smart pointer `Cow<str>`, but we don't need to worry about that yet. 
For our current purposes, we can use it like a normal `&str` — we'll cover smart pointers later when looking closer into memory and lifetimes.

With this information, we should now know for instance how to format the prints in our previous metadata listing example for prettier output. The following code won't compile however because in it a temporary value is dropped before it is being used. 
To fix this, we need to follow the compiler's advice and store the result of calling `entry.file_name()` in a separate variable.

```rust
use std::fs::read_dir;
fn main() -> std::io::Result<()> {
    for entry in read_dir(".")? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let filename = entry.file_name().to_string_lossy();
        println!(
            "{:>20}: is_dir = {:<5}, len = {:<8}, created = {:?}",
            filename,
            metadata.is_dir(),
            metadata.len(),
            metadata.created()
        );
    }
    Ok(())
}
```

This mistake is very common, and can be quite surprising to new Rustaceans. 
The problem here is that `entry.file_name()` returns a new `OsString`, which is not a reference to entry. 
Then calling `to_string_lossy` on the OsString returns a value that references the `OsString`. 
But the referenced `OsString` gets dropped because no variable is going to be its owner in the current scope. 
To fix this, we can add a variable for the temporary owned value `OsString`. 

</main>


<script src="https://lerina.github.io/js/toc.js"></script>
<script>
let anchor= document.createElement('a');
anchor.href="javascript:closeNav()"; //void(0)"; //anchor[0].onclick = closeNav();
anchor.className = "closebtn";  
anchor.innerHTML="&times;";
document.getElementById("TOC").prepend(anchor);

let navCrumbs= document.createElement('div');
navCrumbs.className = "hover-nav";
navCrumbs.innerHTML = `
<div class="hover-nav">
<ul>
<li><a href="../../../../../index.html">⇦ home</a></li>
<li><a href="../../index.html">code</a></li>
<li><a href="../index.html">Rust / Wasm</a></li>
<li><a href="./index.html">dev notes</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>

