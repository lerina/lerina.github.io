<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
⇦ [lerina.github.io](../../../index.html) - [code](../index.html)  

# Input / Output 

## Console/Terminal User input


### from prompt

#### io::stdin().read_line

**Struct std::io::Stdin**  
*A handle to the standard input stream of a process.*

*Each handle is a shared reference to a global buffer of input data to this process.*

**Function std::io::stdin**  
*Constructs a new handle to the standard input of the current process.*

*Each handle returned is a reference to a shared global buffer whose access is synchronized via a mutex.*

**io::stdin().read_line**  
*Locks this handle and reads a line of input, appending it to the specified buffer.*


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

Or

```rust
use std::io;

fn main(){
    let input: String = get_input("Please type something...");
    println!("{}",input);
}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);

    let mut input = String::new();

    // io::stdin::().read_line(&mut input)
    //    .expect("error: unable to read user input");

    match io::stdin().read_line(&mut input) {
        Ok(_data_is_in_input_above) => {},
        Err(_initial_empty_string_is_fine) => {},
    }

    input.trim().to_string()
}
```

[more here](./dev_studies/input_output.html)

### from env

**std::env**  
*This module contains functions to inspect various aspects such as environment variables, process arguments, the current directory, and various other important directories.*

*There are several functions and structs in this module that have a counterpart ending in os. Those ending in os will return an OsString and those without will return a String.*

**env::args()**  
*Returns the arguments that this program was started with (normally passed via the command line).*
*An iterator over the arguments of a process, yielding a String value for each argument.*

*The first element is traditionally the path of the executable*,  
but it can be set to arbitrary text, and might not even exist. This means this property should not be relied upon for security purposes.

```rust
use std::env;

fn main() {
    echo();
}

fn echo() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

**env::var**  
Fetches the environment variable key from the current process.

We can access shell variables such as *$HOME* with `var`

**env!**  
Inspects an environment variable at compile time.

```rust
use std::env;

fn main() {
    
    for argument in env::args() {
        println!("{argument}");
    }


    let name = "USER";
    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e)
    }


    let shell = env!("SHELL", "$SHELL is not set");
    println!("Shell is set to {}", shell);


    let key = "RUST_BACKTRACE";
        env::set_var(key, "1");


    match env::current_dir() {
        Ok(path) => println!("The current directory is {}", path.display()),
        Err(e) => println!("Opps: {e}"),
    }
}
```

Output:

```sh 
./env
USER: tac
Shell is set to /bin/bash
The current directory is /media/tac/Sikidy/Videos/Rs_Studies/io_Files/Display/Spike
```

## Display trait

**fmt::Display**  
*Implementing this trait for a type will automatically implement the ToString trait for the type, allowing the usage of the .to_string() method. Prefer implementing the Display trait for a type, rather than ToString.*

*Display is similar to Debug, but Display is for user-facing output, and so cannot be derived.*

```rust
// [source](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html)

use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Decimal floating point number to binary is a draft programming task. 
// It is not yet considered ready to be promoted as a complete task,
// The output might be something like this:
//
//       23.34375  => 10111.01011
//     1011.11101  =>    11.90625
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binding = self.x.to_string();
        let  x: Vec<_> = binding.split(".").collect();
        let x_left = x[0].parse::<i32>().unwrap();
        let x_right = x[1].parse::<i32>().unwrap();
        
        let binding = self.y.to_string();
        let  y: Vec<_> = binding.split(".").collect();
        let y_left = y[0].parse::<i32>().unwrap();
        let y_right = y[1].parse::<i32>().unwrap();

        write!( f, "x: {:08b}.{:08b}, y: {:08b}.{:08b}", x_left, x_right, y_left, y_right)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Binary: {:b}", point);

}
```

## Write String to File

### 1. Write all data to a file at once

**fs::write**  
*Write a slice as the entire contents of a file.*

*This function will create a file if it does not exist, and will entirely replace its contents if it does.*
*This is a convenience function for using File::create and write_all with fewer imports.*


**as_bytes**  
*Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the from_utf8 function.*

#### 1.1 data as &str

```rust
use std::fs;
use std::io::Write;

// 1. Write all data to a file at once
// 1.1 data as &str
fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 1.1. Write all data to a file at once
    match write_data_to_file(&"db_1.1.txt", &data) {
        Ok(_) => println!("Write all data to a file at once"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

#### 1.2 data as [u8] 

call with  data.as_bytes() if its a &str

```rust
use std::fs;
use std::io::Write;

// 1.2 data as [u8] (call with  data.as_bytes() if its a &str)
fn write_data_to_file_as_bytes(
    path: &str, 
    data: &[u8]) -> Result<(), Box<dyn std::error::Error>> 
{
    fs::write(&path, &data)?;

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 1.2. Write all data to a file at once as *[u8]
    match write_data_to_file_as_bytes(&"db_1.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data to a file at once as *[u8]"),
        Err(e) => println!("Opps: {e}"),
    }
}
```


#### The code together

```rust
use std::fs;
use std::io::Write;

// 1. Write all data to a file at once
// 1.1 data as &str
fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;

    Ok(())
}
// 1.2 data as [u8] (call with  data.as_bytes() if its a &str)
fn write_data_to_file_as_bytes(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 1.1. Write all data to a file at once
    match write_data_to_file(&"db_1.1.txt", &data) {
        Ok(_) => println!("Write all data to a file at once"),
        Err(e) => println!("Opps: {e}"),
    }

    // 1.2. Write all data to a file at once as *[u8]
    match write_data_to_file_as_bytes(&"db_1.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data to a file at once as *[u8]"),
        Err(e) => println!("Opps: {e}"),
    }

}
```

### 2. Write all data to a file at once with fs::File & write_all

**fs::File**  
*An object providing access to an open file on the filesystem.*

*An instance of a File can be read and/or written depending on what options it was opened with. Files also implement Seek to alter the logical cursor that the file contains internally.*

*Files are automatically closed when they go out of scope.*

#### 2.1 write &str all at once 

```rust
use std::fs;
use std::io::Write;

// 2.1 write &str all at once
fn write_data_to_file_with_fs_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 2. Write all data to a file at once with fs::File
    // 2.1 write &str all at once 
    match write_data_to_file_with_fs_write_all(&"db_2.1.txt", &data) {
        Ok(_) => println!("Write all data as &str to a file at once with fs::File and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

#### 2.2 write bytes all at once 

```rust
use std::fs;
use std::io::Write;

// 2.2 write bytes all at once
fn write_data_to_file_with_fs_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 2. Write all data to a file at once with fs::File
    // 2.2 handle bytes
    match write_data_to_file_with_fs_with_write(&"db_2.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data as [u8] to a file with fs::File and write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

#### The code together

```rust
use std::fs;
use std::io::Write;

// 2.1 write &str all at once
fn write_data_to_file_with_fs_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

// 2.2 write bytes all at once
fn write_data_to_file_with_fs_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 2. Write all data to a file at once with fs::File
    // 2.1 all at once
    match write_data_to_file_with_fs_write_all(&"db_2.1.txt", &data) {
        Ok(_) => println!("Write all data as &str to a file at once with fs::File and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }
    // 2.2 handle bytes
    match write_data_to_file_with_fs_with_write(&"db_2.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data as [u8] to a file with fs::File and write()"),
        Err(e) => println!("Opps: {e}"),
    }


}
```


### 3. fs::OpenOptions builder

**fs::OpenOptions**  
*Options and flags which can be used to configure how a file is opened.*

*This builder exposes the ability to configure how a File is opened and what operations are permitted on the open file. The File::open and File::create methods are aliases for commonly used options using this builder.*

*Generally speaking, when using OpenOptions, you’ll first call OpenOptions::new, then chain calls to methods to set each option, then call OpenOptions::open, passing the path of the file you’re trying to open. This will give you a io::Result with a File inside that you can further operate on.*
_ [Rust docs: fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)  

#### 3.1 Read/Write. Create if file missing.

Opening a file for both reading and writing, as well as creating it if it doesn’t exist

```rust
use std::fs::OpenOptions;

// 3.1 Read/Write. Create if file missing.
let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("foo.txt");
```

#### 3.2 Append data to a file with fs::OpenOptions 

*Note that setting .write(true).append(true) has the same effect as setting only .append(true).*  
*This function doesn’t create the file if it doesn’t exist. Use the OpenOptions::create method to do so.*

##### 3.2.1 write_all data as &str

```rust
use std::fs;
use std::io::Write;

// 3.2.1 write_all data as &str
fn append_data_to_file_with_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 3.2.1 data as &str ([u8] if using as_bytes())
    match append_data_to_file_with_write_all(&"db_3.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once using write_all()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

##### 3.2.2 append data.as_bytes() with write()

```rust
use std::fs;
use std::io::Write;

// 3.2.2 append data.as_bytes() with write
fn append_data_to_file_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 3.2.2 data as [u8] (&str if not using as_bytes())
    match append_data_to_file_with_write(&"db_3.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as [u8] to a file using write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

##### The Code together

```rust
use std::fs;
use std::io::Write;

Append data to a file
// 3.2.1 write_all data as &str
fn append_data_to_file_with_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

// 3.2.2 append data.as_bytes()
fn append_data_to_file_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";


    // 3.2.1 data as &str ([u8] if using as_bytes())
    match append_data_to_file_with_write_all(&"db_3.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once using write_all()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 3.2.2 data as [u8] (&str if not using as_bytes())
    match append_data_to_file_with_write(&"db_3.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as [u8] to a file using write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```

### 4. Write and Append data to a file with BufWriter

**std::io::BufWriter**  
*Wraps a writer and buffers its output.*

*It can be excessively inefficient to work directly with something that implements Write.*
*BufWriter<W> can improve the speed of programs that make __small and repeated__ write calls to the same file or network socket.*

*It does not help when writing very large amounts at once, or writing just one or a few times. It also provides no advantage when writing to a destination that is in memory, like a `Vec<u8>`.*

*It is critical to call flush before BufWriter<W> is dropped. Though dropping will attempt to flush the contents of the buffer, any errors that happen in the process of dropping will be ignored. Calling flush ensures that the buffer is empty and thus dropping will not even attempt file operations.*

#### 4.1 append data as &str with BufWriter and write_all

```rust
use std::fs;
use std::io::{Write, BufWriter};

// 4.1 append data as &str with BufWriter and write_all
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_and_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    file.write_all(&data.as_bytes())?; // because fn write_all(&mut self, buf: &[u8]) -> Result<()>

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

fn main() {

    // 4.1 append data as &str with write_all 
    match append_data_to_file_with_BufWriter_and_write_all(&"db_4.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```
#### 4.2 append data.as_bytes() with BufWriter and write

```rust
use std::fs;
use std::io::{Write, BufWriter};

// 4.2 append data.as_bytes() with BufWriter & write
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_and_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

fn main() {

    // 4.2 append data.as_bytes() with BufWriter and write
    match append_data_to_file_with_BufWriter_and_write(&"db_4.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```
#### Code all together

```rust
use std::fs;
use std::io::{Write, BufWriter};

// 4.1 append data as &str with BufWriter and write_all
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_and_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    file.write_all(&data.as_bytes())?; // because fn write_all(&mut self, buf: &[u8]) -> Result<()>

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

// 4.2 append data.as_bytes() with write
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_and_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

fn main() {

    // 4.1 append data as &str with write_all 
    match append_data_to_file_with_BufWriter_and_write_all(&"db_4.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 4.2 append data.as_bytes() with BufWriter and write
    match append_data_to_file_with_BufWriter_and_write(&"db_4.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```


## Read File

### 1. read whole file into String.

Read the entire contents of a file into a string.

*This is a convenience function for using `File::open` and `read_to_string` with fewer imports and without an intermediate variable.*  
_ [Rust doc: fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)

```rust
use std::fs;

fn read_to_string(file: &str) -> String {
    match fs::read_to_string(file) {
        Ok(f) => f.to_string(),
        Err(_) => "".to_string(),
    }
}

fn main() {
    let read_from_string = read_to_string("fs.rs");
    println!("{}", read_from_string);
}
```

or

```rust
use std::fs;

fn file_to_string() -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./db.txt")?;

    Ok(content)
}

fn main() {
    println!("1. read whole file into String.");
    let data = file_to_string();
    match data {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Something happened {e}"),
    }
}
```

### 2. read whole file into Vec<u8> then convert string.

Read the entire contents of a file into a bytes vector.


*This is a convenience function for using `File::open` and `read_to_end` with fewer imports and without an intermediate variable.*  
_ [Rust doc: fs::read](https://doc.rust-lang.org/std/fs/fn.read.html)

```rust
use std::fs;
use std::str;

fn file_to_vec_u8() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let content = fs::read("./db.txt")?;

    Ok(content)
}

fn main() {
    println!("2. read whole file into Vec<u8> then convert string.");
    let data = file_to_vec_u8();
    match data {
        Ok(content) => match str::from_utf8(&content) {
            Ok(str_data) => println!("{str_data}"),
            Err(e) => println!("Could not convert: {e}"),
        },
        Err(e) => println!("Something happened {e}"),
    }
}
```

### 3. read file line by line in to a Vec<String>

**fs::File**  
An object providing access to an open file on the filesystem.

*An instance of a File can be read and/or written depending on what options it was opened with. Files also implement Seek to alter the logical cursor that the file contains internally.*

*Files are automatically closed when they go out of scope.*  
_ [Rust doc: fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)  

**File::open**  
Attempts to open a file in read-only mode.

**BufReader**  
The `BufReader` struct adds buffering to any reader.

*A `BufReader` performs large, infrequent reads on the underlying Read and maintains an in-memory buffer of the results.*  
*`BufReader` can improve the speed of programs that make small and repeated read calls to the same file or network socket.*   
*It does not help when reading very large amounts at once, or reading just one or a few times.*   
*It also provides no advantage when reading from a source that is already in memory, like a `Vec<u8>`.*   
_ [Rust doc: BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)

```rust
use std::fs::File;
use std::io::{
    BufRead, // lines()
    BufReader
};

fn file_line_by_line() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file: File = File::open("./db.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut content: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(aline) => content.push(aline),
            Err(e) => println!("Opps: {e}"),
        }
    }

    Ok(content)
}

fn main() {
    println!("3. read file line by line in to a Vec<String>");
    let data = file_line_by_line();
    match data {
        Ok(content) => {
            for line in content {
                println!("{line}");
            }
        }
        Err(e) => println!("Something happened {e}"),
    }
} 
```

### 4. read file byte by byte in to a Vec<u8>

```rust
use std::fs::File;
use std::io::{
    BufReader,
    Read, // bytes()
};

fn file_byte_by_byte() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file: File = File::open("./db.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut content: Vec<u8> = Vec::new();

    for line in reader.bytes() {
        match line {
            Ok(aline) => content.push(aline),
            Err(e) => println!("Opps: {e}"),
        }
    }

    Ok(content)
}

fn main() {
    println!("4. read file byte by byte in to a Vec<u8>");
    let data = file_byte_by_byte();
    match data {
        Ok(content) => {
            for line in content {
                println!("{line}");
            }
        }
        Err(e) => println!("Something happened {e}"),
    }

}
```

### 5. read file byte chunks by byte chunk in to an array [u8]

```rust
use std::fs::File; 
use std::io::{BufRead, BufReader, Read}; 
use std::str;

fn file_byte_chunk_by_byte_chunk(buffer_size: usize) 
    -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {

    let file: File = File::open("./db.txt")?;
    let mut reader: BufReader<File> = 
        BufReader::with_capacity(buffer_size, file);
    
    let mut content: Vec<Vec<u8>> = Vec::new();
    loop {
        let buffer = reader.fill_buf()?;
        let buffer_length: usize = buffer.len();
        
        if buffer_length == 0 { break; }

        content.push(buffer.to_vec());
        reader.consume(buffer_length);
    };
    
    Ok(content)
}

fn main() {
    println!("5. read file byte by byte in chunks of {BUFFER_SIZE} ");
    const BUFFER_SIZE: usize = 4;
    let data = file_byte_chunk_by_byte_chunk(BUFFER_SIZE);
    match data {
        Ok(content) => {
            for line in content {
                match str::from_utf8(&line) {
                    Ok(chunk) => println!("{chunk}"),
                    Err(e) => println!("Could not convert to utf8: {e}"),
                }
            }
        }
        Err(e) => println!("Something happened {e}"),
    }
}
```

---

### Write File: Everything together

```rust
use std::fs;
use std::io::{Write, BufWriter};

// 1. Write all data to a file at once
// 1.1 data as &str
fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;

    Ok(())
}
// 1.2 data as [u8] (&str if not using as_bytes())
fn write_data_to_file_as_bytes(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;

    Ok(())
}

// 2. Write all data to a file at once with fs::File
// 2.1 all at once
fn write_data_to_file_with_fs_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

// 2.2 handle bytes
fn write_data_to_file_with_fs_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::File::create(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

// 3. Append data to a file
// 3.1 write_all data as &str
fn append_data_to_file_with_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    file.write_all(&data.as_bytes())?;

    Ok(())
}

// 3.2 append data.as_bytes()
fn append_data_to_file_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    Ok(())
}

// 4. Write and Append data to a file with BufWriter
// 4.1 append data as &str with write_all
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_with_write_all(
    path: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    file.write_all(&data.as_bytes())?;

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

// 4.2 append data.as_bytes() with write
#[allow(non_snake_case)] 
fn append_data_to_file_with_BufWriter_with_write(
    path: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let file: fs::File = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file: BufWriter<fs::File>  = BufWriter::new(file);

    let remaining = file.write(&data)?;
    if remaining > 0 {
        println!("{remaining} bytes not written");
    }

    file.flush()?; // <--- make sure Os really writes it
 
    Ok(())
}

fn main() {
    let data = "\
He he he

plenty data
 
here.";

    // 1.1. Write all data to a file at once
    match write_data_to_file(&"db_1.1.txt", &data) {
        Ok(_) => println!("Write all data to a file at once"),
        Err(e) => println!("Opps: {e}"),
    }

    // 1.2. Write all data to a file at once as *[u8]
    match write_data_to_file_as_bytes(&"db_1.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data to a file at once as *[u8]"),
        Err(e) => println!("Opps: {e}"),
    }

    // 2. Write all data to a file at once with fs::File
    // 2.1 all at once
    match write_data_to_file_with_fs_write_all(&"db_2.1.txt", &data) {
        Ok(_) => println!("Write all data as &str to a file at once with fs::File and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }
    // 2.2 handle bytes
    match write_data_to_file_with_fs_with_write(&"db_2.2.txt", &data.as_bytes()) {
        Ok(_) => println!("Write all data as [u8] to a file with fs::File and write()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 3. Append data to a file
    // 3.1 data as &str ([u8] if using as_bytes())
    match append_data_to_file_with_write_all(&"db_3.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once using write_all()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 3.2 data as [u8] (&str if not using as_bytes())
    match append_data_to_file_with_write(&"db_3.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as [u8] to a file using write()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 4. Write and Append data to a file with BufWriter
    // 4.1 append data as &str with write_all 
    match append_data_to_file_with_BufWriter_with_write_all(&"db_4.txt", &data) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write_all()"),
        Err(e) => println!("Opps: {e}"),
    }

    // 4.2 append data.as_bytes() with BufWriter and write
    match append_data_to_file_with_BufWriter_with_write(&"db_4.txt", &data.as_bytes()) {
        Ok(_) => println!("Append all data as &str to a file at once with BufWriter and write()"),
        Err(e) => println!("Opps: {e}"),
    }
}
```


### Read File: Everything together

```rust
use std::fs;                             // 1.
use std::fs::File;                       // 3, 4 and 5
use std::io::{BufRead, BufReader, Read}; // 3. 4 and 5 needs Read
use std::str;                            // 2, 5

// 1. read whole file into String.
fn file_to_string() -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./db.txt")?;

    Ok(content)
}

// 2. read whole file into Vec<u8> then convert string.
fn file_to_vec_u8() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let content = fs::read("./db.txt")?;

    Ok(content)
}

// 3. read file line by line in to a Vec<String>
fn file_line_by_line() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file: File = File::open("./db.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut content: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(aline) => content.push(aline),
            Err(e) => println!("Opps: {e}"),
        }
    }

    Ok(content)
}

// 4. read file byte by byte in to a Vec<u8>
fn file_byte_by_byte() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file: File = File::open("./db.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut content: Vec<u8> = Vec::new();

    for line in reader.bytes() {
        match line {
            Ok(aline) => content.push(aline),
            Err(e) => println!("Opps: {e}"),
        }
    }

    Ok(content)
}

// 5. read file byte chunks by byte chunk in to an array [u8]
fn file_byte_chunk_by_byte_chunk(buffer_size: usize) 
    -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {

    let file: File = File::open("./db.txt")?;
    let mut reader: BufReader<File> = 
        BufReader::with_capacity(buffer_size, file);
    
    let mut content: Vec<Vec<u8>> = Vec::new();
    loop {
        let buffer = reader.fill_buf()?;
        let buffer_length: usize = buffer.len();
        
        if buffer_length == 0 { break; }

        content.push(buffer.to_vec());
        reader.consume(buffer_length);
    };
    
    Ok(content)
}

fn main() {
    // 1.
    println!("1. read whole file into String.");
    let data = file_to_string();
    match data {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Something happened {e}"),
    }

    // 2.
    println!("2. read whole file into Vec<u8> then convert string.");
    let data = file_to_vec_u8();
    match data {
        Ok(content) => match str::from_utf8(&content) {
            Ok(str_data) => println!("{str_data}"),
            Err(e) => println!("Could not convert: {e}"),
        },
        Err(e) => println!("Something happened {e}"),
    }

    // - If you feed in say a png file you get the error msg
    // "Could not convert: invalid utf-8 sequence of 1 bytes from index 0"
    //
    // - And if there is no file to read you get
    // "Something happened No such file or directory (os error 2)"

    // 3.
    println!("3. read file line by line in to a Vec<String>");
    let data = file_line_by_line();
    match data {
        Ok(content) => {
            for line in content {
                println!("{line}");
            }
        }
        Err(e) => println!("Something happened {e}"),
    }

    // 4. 
    println!("4. read file byte by byte in to a Vec<u8>");
    let data = file_byte_by_byte();
    match data {
        Ok(content) => {
            for line in content {
                println!("{line}");
            }
        }
        Err(e) => println!("Something happened {e}"),
    }

    // 5.
    println!("5. read file byte by byte in chunks of {BUFFER_SIZE} ");
    const BUFFER_SIZE: usize = 4;
    let data = file_byte_chunk_by_byte_chunk(BUFFER_SIZE);
    match data {
        Ok(content) => {
            for line in content {
                match str::from_utf8(&line) {
                    Ok(chunk) => println!("{chunk}"),
                    Err(e) => println!("Could not convert to utf8: {e}"),
                }
            }
        }
        Err(e) => println!("Something happened {e}"),
    }
}
```

## Removing a File in Rust

**remove_file()**  
Removes a file from the filesystem.
To remove or delete a file in Rust, we can use the  method from the std::fs module.

Note that there is no guarantee that the file is immediately deleted (e.g., depending on platform, other open file descriptors may prevent immediate removal).


```rust
use std::fs;

fn main() {
    // Remove a file
    fs::remove_file("data.txt").expect("could not remove file");
    
    println!("Removed file data.txt");
}
```

## Serde


Serde is "a framework for serializing and deserializing Rust data structures efficiently and generically."

```
Rust structure 
  ↓
  -- Serialize --> Structure in terms of the Serde data model*
  ↓
  -- Data format (JSON/Bincode/etc) --> Convert the Serde data model to the output format
```

\*The Serde data model is a simplified form of Rust's type system.
In the case of most Rust types, their mapping into the Serde data model is straightforward.

### Using derive

Serde provides a derive macro to generate implementations of the Serialize and Deserialize traits for data structures defined in your crate, allowing them to be represented conveniently in all of Serde's data formats.

To use `#[derive(Serialize, Deserialize)]` in the code,

- Add serde = { version = "1.0", features = ["derive"] } as a dependency in Cargo.toml.
- Ensure that all other Serde-based dependencies (for example serde_json) are on a version that is compatible with serde 1.0.
- On structs and enums that you want to serialize, import the derive macro as use serde::Serialize; within the same module and write #[derive(Serialize)] on the struct or enum.
- Similarly *import* `use serde::Deserialize;` and write `#[derive(Deserialize)]` on structs and enums that you want to deserialize.

So in `Cargo.toml` we have:

```toml
...
[dependencies]
serde = { version = "1.0", features = ["derive"] }

# serde_json is just for the example, not required in general
serde_json = "1.0"
...
```

Usage example

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);  
    // Output: serialized = {"x":1,"y":2}

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    // Output: deserialized = Point { x: 1, y: 2 }
}
```

Run it

```sh
$ cargo run
serialized = {"x":1,"y":2}
deserialized = Point { x: 1, y: 2 }
```



Note: 

_Sometimes you may see compile-time errors that tell you:<br/><br/>the trait `serde::ser::Serialize` is not implemented for `...`<br/><br/>even though the struct or enum clearly has #[derive(Serialize)] on it._
<br/>
*This almost always means that you are using libraries that depend on incompatible versions of Serde. You may be depending on serde 1.0 in your Cargo.toml but using some other library that depends on serde 0.9. So the Serialize trait from serde 1.0 may be implemented, but the library expects an implementation of the Serialize trait from serde 0.9. From the Rust compiler's perspective these are totally different traits.*
<br/><br/>
*The fix is to upgrade or downgrade libraries as appropriate until the Serde versions match. The cargo tree -d command is helpful for finding all the places that duplicate dependencies are being pulled in.*

_ [serde.rs doc](https://serde.rs/derive.html)



## Replacing command-line `curl` with Rust `reqwest` & `tokio`

```sh
curl "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m"
```

The reqwest crate provides a convenient, higher-level HTTP Client.

```rust
use reqwest::{Client, Error};

async fn meteo() -> Result<(), Error> {
    const URL1: &str = "https://api.open-meteo.com/v1/forecast?latitude=-18.879190&longitude=47.507904&current=temperature_2m,wind_speed_10m";
    match reqwest::get(URL1).await {
        Ok(resp) => {
            let antananarivo: OpenMeteo = resp.json().await?;
             //= serde_json::from_str(&json).unwrap();
            //println!("Antananarivo\n{:#?}", antananarivo)
            println!("Antananarivo\n current weather is {}{}", 
                antananarivo.current.temperature_2m, 
                antananarivo.current_units.temperature_2m);
        }
        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }

    Ok(())
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Error> {

    meteo().await?;

    Ok(())
}
```

In Cargo.toml

```toml
[dependencies]
reqwest = { version = "0.11.23", features = ["json"] } # reqwest with JSON parsing support
#serde = { version = "1.0.193", features = ["derive"] }
#serde_json = "1.0.108"
tokio = { version = "1.35.1", features = ["full"] } # for our async runtime

```

---

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
<li><a href="../../../index.html">⇦ home</a></li>
<li><a href="../index.html">code</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
