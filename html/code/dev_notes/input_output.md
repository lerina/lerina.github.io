<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
⇦ [lerina.github.io](../../../index.html) - [code](../index.html)  

# Input / Output 

## Console/Terminal User input


### from prompt

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

### from env

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

## fmt::Display

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

## String to File

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

## File to String

```rust
use std::fs; // 1.
use std::fs::File; // 3. and 4.
use std::io::{BufRead, BufReader, Read}; // 3. and 4.needs Read
use std::str; // 2.

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

To remove or delete a file in Rust, we can use the remove_file() method from the std::fs module.

```rust
use std::fs;

fn main() {
    // Remove a file
    fs::remove_file("data.txt").expect("could not remove file");
    
    println!("Removed file data.txt");
}
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
