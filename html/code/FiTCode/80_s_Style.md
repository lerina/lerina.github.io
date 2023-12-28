## Problem Statement 

Here is the COP (Challenge, Opportunity, Problem).

> The simple warehouse program is an information system for managing the
> contents of a storage facility for different kinds of wares. Every ware
> in the warehouse has a name, a description, a storage location, a
> pricetag, information about the number of items of the particular ware
> stored in the warehouse, and the cost of each item. A storage location
> is a "section" (a single letter a-z), plus a shelf (an integer).

The simple warehouse program manages a database of wares and supports
the following operations:

-   Adding a ware
-   Removing a ware
-   Editing the information about a ware
-   Printing the information for a specific ware
-   Listing all wares in the database
-   Undoing the last action
-   Quitting the program

When **starting** the program, the user is presented with the main menu with
the above alternatives and picks a choice by entering the first letter
of each alternative, i.e., `A`, `R`, `E`, `P`, `L`, `U`, or `Q`.

When choosing to **quit**, the user is prompted with a yes/no question: do
you really want to quit? If the answer is `Y`, the program exits. If the
answer is `N`, the program displays the main menu again. If the answer
is anything other than `Y` or `N`, an error message is printed and the
question repeated.

When choosing to **add** a ware, the user is prompted with five consecutive
dialogues asking her to enter the required information for a ware:

-   Name
-   Description
-   Storage location
-   Price
-   Amount

The program then displays the information back to the user and asks
whether this is correct or not. The user has three options:

-   `E`dit the information
-   `S`ave the information
-   `A`bort

The choice is indicated by entering the first character of each option. 
The *first alternative* allows the user to re-enter one piece of
the required information for a ware after which the editer ware is
displayed again with the same question as above.  
The *save alternative* saves the information to the database and then returns to the main menu.  
The *abort alternative* returns to the main menu without saving.

The rest of the actions on the main menu should be fairly self-explanatory.


### Textual Analysis 

To get a feeling for what the program is trying to accomplish, we mine
the specification. This means re-reading it, looking for some particular
kind of information. To understand the program's **core behaviour**, we
pay close attention to the verbs and actions. 
Underline or highlight them is not a bad idea.

For concreteness, I am copying in parts of the specification from above,
**highlighting** as I go.

> The simple warehouse program is an for ... letter a-z), plus a shelf
> (an integer).
>
> The simple warehouse program manages a database of wares and supports
> the following operations:
>
> 1.  **Adding** a ware
> 2.  **Removing** a ware
> 3.  **Editing** the information about a ware
> 4.  **Printing** the information for a specific ware
> 5.  **Listing** all wares in the database
> 6.  **Undoing** the last action
> 7.  **Quitting** the program
>
> When **starting** the program, the user is **presented** with the main
> menu with the above alternatives and **picks a choice** by entering
> the first letter of each alternative, i.e., `A`, `R`, `E`, `P`, `L`,
> `U`, or `Q`.
>
> When **choosing to quit**, the user **is prompted with a yes/no
> question**: ...

Objective: See what actions the program should support (textual analysis, mindmaps, ). 
At this stage there should be nothing about how they should be implemented.

**Divide & Conquer** 

Isolate problems and  sub-problems

### Project setup

```sh
cargo new warehouse
cd warehouse
```

We start with the minimal working program.

```rust
fn main() {
  println!("Welcome to my warehouse program!");
}
```

This program is pretty much hello world, 

```sh
cargo run  
```

but it **compiles and runs**.

### From Reqs to REM 

We convert some requirements into comments

```rust
// src/main.rs

//> 1.  **Adding** a ware

//> 2.  **Removing** a ware

//> 3.  **Editing** the information about a ware

//> 4.  **Printing** the information for a specific ware

//> 5.  **Listing** all wares in the database

//> 6.  **Undoing** the last action

//> 7.  **Quitting** the program

fn main() {
    println!("Welcome to my warehouse program!");
}
```

### Pick a task

Now, we take the actions from the textual analysis and/or mind-map 
and turn them into functions.

For example, for "add", we do:

```rust
//> 1.  **Adding** a ware
fn add_action() {
    println!("Add action is not implemented yet");
}

//> 2. ...
```

In order to have placeholders for code that we are going to write,
we add similar functions for all the other actions.

```rust
//> 2.  **Removing** a ware
fn remove_action() {
    println!("Remove action is not implemented yet");
}

//> 3.  **Editing** the information about a ware
fn edit_action() {
    println!("Edit action is not implemented yet");
}

//> 4.  **Printing** the information for a specific ware
fn print_action() {
    println!("Print action is not implemented yet");
}

//> 5.  **Listing** all wares in the database
fn list_action() {
    println!("List action is not implemented yet");
}

//> 6.  **Undoing** the last action
fn undo_action() {
    println!("Undo action is not implemented yet");
}

//> 7.  **Quitting** the program
fn quit_action() {
    println!("Quit action is not implemented yet");
}
```

Tip:  
**Divide & Conquer**  
Focus on just one small thing at a time.  
**Compile Early. Compile Often.**  
Do not move on to another function until the program compiles.  
**The Compiler Errors are your friends**  
The Rust compiler is your friend. It will often give you the solution to your error.  

### Next

We now have a working program that does absolutely nothing. Let's start
adding some functionality to our program following the specification.

The first six action all require either that there is some data in the
program, or that we are able to add some data. We haven't done any data
design yet, so if we want to start with these, we should that. However,
the last action, quitting, is very simple and does not do much so that
is a good starting point.

We now go back to our mind-map or program outline and extend the specification of `quit`.


Re-reading the specification, we now know that triggering quit should

- enter into a confirmation dialogue, and 
- if the user replies `Y`, then we quit, 
- if `N` we return to the main menu, 
- otherwise repeat the question.

### Code as little as possible between each compile 

To keep the program as simple as possible, we are not going to add
functionality for choosing an action in the main menu just yet. That's
not needed, because the only thing we are going to implement is quit.

So, although it is futile, let's extend the program so that when we run
it, it asks if we want to quit. 

To this end, we will add a `while` loop that loops forever 
and that in each turn of the loop asks us whether we
want to quit. 

This will allow us to *manualy* test both the yes and no answers.

```rust
...
fn main() {
   loop {
        println!("Welcome to my warehouse program!");

        if quit_action() == true { 
            break; 
        }
    }
}
```

And we get an error:

```sh
error[E0308]: mismatched types
  --> src/main.rs:42:29
   |
42 |         if quit_action() == true {
   |            -------------    ^^^^ expected `()`, found `bool`
   |            |
   |            expected because this is `()`

For more information about this error, try `rustc --explain E0308`.
```

`quit_action()` must first be updated to return a `bool`. 
We fix this with the simplest code to pass the test 
by returning `false` as the default return.

```rust
//> 7.  **Quitting** the program
fn quit_action() -> bool {
    println!("Quit action is not implemented yet");

    false
}
```

Compile

```sh
cargo build
```

Ok it build.

Run it. `ctrl-c to stop`

```sh
cargo run
```
Output:
```sh
Welcome to my warehouse program!
Quit action is not implemented yet
Welcome to my warehouse program!
...
Quit action is not implemented yet
^C

```

### Next

Our goal now is to implement the behaviour of the specification which
was just re-iterated above. We are going to do this in a few small
increments. 

-   Print confirmation question
-   Read input
-   Handle `Y` case
-   Handle `N` case
-   Handle remaining cases

Increment 1 is easy. Just change the existing `println!`

```rust
...
//> 7.  **Quitting** the program
fn quit_action() -> bool {
   println!("Do you want to quit? [Y/N]");

   false
}
```

Increment 2 is harder. 

It requires us to know how to read input from a user in Rust. 
This is actually pretty complicated and there are many opportunities for errors. 
Therefore, **temporarily write-out problems caused by hard-to-write code** 
just by delegating it to a yet to be written function.  
In effect pass the buck: **assume the existence of a function that solves the problem.** 

This makes increment 2 really simple:

```rust
...
//> 7.  **Quitting** the program
fn quit_action() -> bool {
    println!("Do you want to quit? [Y/N]");
    let input: String = get_char_input(); // <---- This line was added
        
    false
}
```

**Passing_the_buck** is one of the best tools
in the programming toolbox. You should use it all the time. 

The first rule of passing_the_buck is this:

> *If you don't know immediately how to solve a problem, assume the
> existence of a function that solves this problem for you, and call that
> function.*

This is great because usually when you are programming, you are going
somewhere and you have a particular 
[control flow](http://en.wikipedia.org/wiki/Control_flow) in mind. 
Getting stuck implementing one step of this flow usually means your train of thought
is interrupted and because your mind is now focused on solving the current step, 
you will have a harder time continuing where you got stuck once you are unstuck.

Like we did with the empty action functions from the start, we add a
dummy function to the file called `get_char_input()` to stay true to the
ideal of always having a working program:

```rust
// TODO: implement
fn get_char_input() -> String { // <-- return type derived 
                               // from the call-site (quit_action())

  "Y".to_string()               // <-- a good default because of how 
}                               // increment 3 is specified
```

We can now compile and run our program as a kind of minimal testing.
It compiles, it runs. 

We can go back to increment 3, 
which is dealing with the base case of the user entering a `Y`:

```rust
...
//> 7.  **Quitting** the program
fn quit_action() -> bool {
    println!("Do you want to quit? [Y/N]");
    let input: String = get_char_input(); 
    if input == "Y" { return true; } // <---- This line was added
        
    false
}
```

Increment 4 is dealing with the case of the user inputting a `N`. Turns
out it is already handled, as the function returns `false` as a default.

This leaves us with increment 5, which is the user replying with
something other than `Y` or `N`.

Its more work. 
We must now add a loop to this function to be able to repeat the question.

We start by wrapping the entire function inside a `while`. 
The exit condition of the `while` is that input is either `Y` or `N`. 

Otherwise, the specification dictates', an error message shall be printed and the
question shall be asked again indefinitely. Here is a first attempt at doing this:


```rust
//> 7.  **Quitting** the program
fn quit_action() -> bool {
   loop {                                          // <---- This line was added           
       println!("Do you want to quit? [Y/N]");
       let input: String = get_char_input(); 
       
       if input == "Y" { return true; }
       if input == "N" { return false; }           // <---- This line was added
       println!("Only Y or N are valid answers!"); // <---- This line was added
   }

   false
}
```

The function is now properly implemented, modulo passing_the_buck 
to `get_char_input` the call to the (yet) non-existent function.


### Stepwise refinement: Developing in Increments

The `quit_action()` function was developed in a series of steps. The
choice of the steps was very deliberate. Here are the rules to follow:

If there is any kind of branching, start with the most basic interesting case first
:   In our example, `Y` and `N` are equally basic but only `Y` leads to
    something interesting, i.e., exits the program. The non yes or no
    cases require retrying which is more complicated, so we delay those
    even further. In the end, the order is `Y`, `N`, and "the rest".

Always finish a straight-line version first
:   This means that any code that involves loops, you start by focusing
    on getting one iteration of the loop working in the working program.
    Once that is done, you can more easily wrap the working code inside
    a loop. Note that by this rule, the Y case would still be
    implemented before "the rest" even if Y called some other functions
    to save the database etc.

Each increment should accomplish something sensible
:   Each increment should result in a working program.

Each increment should ideally have a test that drives the coding or at the very least
:   Each increment should ideally have a testable outcome
:       For example, when we implemented (skipped...) increment 4, we should also have changed the `get_char_input()` to return N to test this. And similar for increment 5.

### Step 5: The buck stops here!

We finished `quit_action()`, but mostly because we passed the buck to a non-existant function. 
To finish implementing this part of the program, we will now implement `get_char_input()`. 

However, the reason why we didn't write this piece of code down immediately 
from the start was because we thought it was complicated and did not know 
exactly how to do it. 

So, how do we proceed now?

Well, we cannot magically suddenly grasp how to read user input. This
knowledge generally comes from somewhere. Let's cover the three most
basic cases:

1.  Google/Stack Exchange (usenet, bbs, in the 80's)
2.  A text book
3.  Man pages

You find interesting code? You try it out!

Here is the golden rule of trying code out: **start with a new empty
file that just tests the code you're trying to get to run.**

The reason for this is **isolation**. It may be that the reason
why you are stuck on something is because there is an error in your code
somewhere that breaks some part of the code under study.
by isolating the code you are looking at in a single file, there
is a lot less disturbance. You want to be working at all times with the
smallest possible thing that can work. 
Remember: *good programmers like lazy.*  

If the problem is getting user input, then writing a small test program
for that is easy. To simplify copying the solution across to the "real
program" you can even name the functions right from the start:

```rust
    fn get_char_input() {
        todo!();
    }

    fn main () {
        println!("You pressed: '{}'", get_char_input());
    }
```

This program will call `get_char_input()` and print out the character
between single quotes to avoid confusion. We can now spend the next 15
minutes on getting the code that we are finding online to work in this
file, and then simply copy `get_char_input()` across to the other Rust
file.

Our Spike for example could yield something like this

```rust
// get_char.rs

/* source inspiration
[Rustdoc std::io::stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
https://www.becomebetterprogrammer.com/rust-read-user-input-stdin/
*/
use std::io;
//use std::result::Result;

//fn get_char_input() -> Result<String, &'static str> {
fn get_char_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin
        .read_line(&mut user_input)
        .expect("Failed to read line");
    /*
        match user_input.get(0..1) {
            Some(ans) => Ok(ans.to_string()),
            None => Err("something went wrong"),
        }
    */
    match user_input.get(0..1) {
        Some(ans) => ans.to_string(),
        None => " ".to_string(),
    }

    //}; //.chars().nth(0).ok_or(" ")?;
    //Ok(ans.to_string())
}

fn main() {
    println!("enter a character");
    println!("the char is {}", get_char_input());
    //println!("the char is {}", get_char_input().unwrap());
}
```
compile with
```sh
rustc get_char.rs
```

Once it compiles and runs and we can confirm it does the intended job we can use the cleaned-up code in our project.

```rust
// src/main.rs
use std::io;
...

fn get_char_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // get the first char in a unicode safe way
    match user_input.get(0..1) {
        Some(ans) => ans.to_string(),
        None => " ".to_string(),
    }
}

...
fn main() {
    ...
}
```

Now we test our code `cargo run`

```sh
Welcome to my warehouse program!
Do you want to quit? [Y/N]
N
Welcome to my warehouse program!
Do you want to quit? [Y/N]
2
Only Y or N are valid answers!
Do you want to quit? [Y/N]
Y
```

woot! Progress!

### What's next?


To keep things DRY existed before the acronym was coined. 
We have repetitions. 
In both cases, I have to handle the failure mode, i.e., 
when the user presses something that isn’t part of the protocol.

Time to "factoring out" these repetitions. Thats the term used before `refactoring` appeared in the 90's  . 

What I want to do now is to improve the code for get_char_input() 
so that it can handle the failure for me. 

A simple way to do that is to pass both the question 
and the valid alternatives to the get_char_input() function to be used like this:

```rust
// src/main.rs
use std::io;
...
let input = get_char_input("Do you want to quit?", "YN");
let input = get_char_input("Choose menu item!", "AREPLUQ");
```

We pass `get_char_input` a dialog message to be printed let's call it `dialog_msg`
and a list of possible commands, let's call it `cmd`.

This time 

```rust
fn get_char_input(dialog_msg: &str, cmd: &str) -> String {
    println!("{dialog_msg}");

    loop {
        let mut user_input = String::new();

        let stdin = io::stdin();
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // get the first char in a unicode safe way
        let ans = match user_input.get(0..1) {
            Some(ans) => ans.to_string(),
            None => " ".to_string(),
        };

        if cmd.contains(&ans) { return ans; } else {
            println!("Only [{cmd}] are valid answers.");
        }
    }
}
```


And update quit_action 

from 

```rust
//> 7.  **Quitting** the program
fn quit_action() -> bool {
    loop {
        let input: String = get_char_input("Do you want to quit? [Y/N]", "YN");

        if input == "Y" {
            return true;
        }
        if input == "N" {
            return false;
        }
    }
}

```
to

```rust
//> 7.  **Quitting** the program
fn quit_action() -> bool {

    let input: String = get_char_input("Do you want to quit? [Y/N]", "YN");

    if input == "Y" { return true; }

    false
}
```

Great!


Now that we are done with quitting the program, it is time to move on to the next action. 

However, in order to get to the next action, we must first extend the main loop of the program 
so that the user can choose between whatever action we choose to implement next, and quitting.

Looking at the specification, the user’s input is again obtained reading a character. 
Great that we have already implemented get_char_input() to do exactly that. 

Following the instructions on how to develop in increments, etc... 
and looking at how we implemented the dialogue in quit_action(), 
we can relatively quickly write something like this:

```rust
fn main() {
    println!("Welcome to my warehouse program!");
    let mut should_quit = false;

    while should_quit == false {
      print_main_menu();
      let input = get_char_input("Choose menu item!", "AREPLUQ");

      if input == "A"        { add_action(); }
      else if input == "Q"   { should_quit = quit_action(); }
      else                   { println!("Invalid choice!"); }
    }
}
```

We print the main menu, make a call to get_char_input() to get the user input. 
We change the loop to use a `should_quit` variable to control whether the main loop should continue or not. 

Now lets have a full menu. 

```rust

fn print_main_menu(){
    println!("--- Main menu ---\n");
    //> 1.  **Adding** a ware
    println!("[A]dd a ware");
    //> 2.
    println!("[R]emove a ware");
    //> 3.
    println!("[E]dit the information about a ware");
    //> 4.
    println!("[P]rint the information for a specific ware");
    //> 5.  
    println!("[L]ist all wares in the database");
    //> 6.  
    println!("[U]ndo the last action");
    //> 7.  
    println!("[Q]uit the program");
}
```

And use it in our main loop. Lets also refactor these If conditions and replace it with a more idiomatic Rust code

```rust
fn main() {
    println!("Welcome to my warehouse program!");
    let mut should_quit = false;

    while should_quit == false {
      print_main_menu();
        match get_char_input("Choose menu item!", "AREPLUQ") {
            "A" => add_action(),
            "R" => remove_action(),
            "E" => edit_action(),
            "P" => print_action(),
            "L" => list_action(),
            "U" => undo_action(),
            "Q" =>  should_quit = quit_action(); ,
            _ =>  println!("Invalid choice!")
        }
    }
}
```

We get an error. get_char_input returns a string.

rather than elevating to a String each menu choice as in `"Q".to_string()`   
we can do use `&get_char_input("Choose menu item!", "AREPLUQ")[0..]`

```rust
fn main() {
    println!("Welcome to my warehouse program!");
    let mut should_quit = false;

    while should_quit == false {
      print_main_menu();
        match &get_char_input("Choose menu item!", "AREPLUQ")[0..] {
            "A" => add_action(),
            "R" => remove_action(),
            "E" => edit_action(),
            "P" => print_action(),
            "L" => list_action(),
            "U" => undo_action(),
            "Q" =>  should_quit = quit_action(),
            _ =>  println!("Invalid choice!")
        }
    }
}
```

Great! It runs

```sh
--- Main menu ---

[A]dd a ware
[R]emove a ware
[E]dit the information about a ware
[P]rint the information for a specific ware
[L]ist all wares in the database
[U]ndo the last action
[Q]uit the program
Choose menu item!
Q
Do you want to quit? [Y/N]
Y
```

Let's change the confirmation question 

```rust
...
fn quit_action() -> bool {

    let input: String = get_char_input("Do you Really want to quit? [Y/N]", "YN");
    ...
```

What's next?

> “Bad programmers worry about the code. Good programmers worry about data structures and their relationships.”
_ Linus Torvalds

Let's proceed with adding a ware. Editing, removing, listing, undoing, etc. all presuppose the existence of some data in the database. Wemight as well have a "database" and add things into it first.

Anyways, adding something new is often simpler than editing or removing. 
This makes it a no brainer good place to start.

We go back to the specification again, and read about the add action. 
We extend our mind-map accordingly:

```
                             .-- Add all ware data
main_menu ------|-- Add ----/--- Display the result
                |           \
                |            `-- Ask confirmation (save/edit/abort)
                ⋮
                |-- Quit ------- Comfirmation dialog
```

To understand
a program's core data, we simply **mine the specification** again, this
time looking for [nouns](http://en.wikipedia.org/wiki/Noun). 

Let's do that now!

Again, for concreteness, I am copying in the specification and
highlighting as I go:

> Every **ware** in the warehouse has a **name**, a **description**, a
> **storage location**, a **pricetag**, information about the **number
> of items** of the particular wares stored in the warehouse, and the
> **cost of each item**. A storage location is a **section** (a single
> letter a-z), plus a **shelf** (an integer).
>
> The simple warehouse program manages a **database** of wares and
> supports the following operations:
>
> 1.  Adding a **ware**
> 2.  Removing a **ware**
> 3.  Editing the information about a **ware**
> 4.  Printing the information for a specific **ware**
> 5.  Listing all wares in the **database**
> 6.  Undoing the last **action**
> 7.  Quitting the program

Looking at the highlighted information above, and reading the
surrounding text, a pattern emerges.  

There is a **database**, which stores **wares**, that each store name, 
description, storage location, pricetag, and number of items.  

```
database {
    ware,
    ware, 
    ...
}
```

There is also a mentioning of **action**, which makes sense: in order 
to undo an action, we need to somehow record what the program did just before, 
and enough information to undo it.  

We don't know exactly what this information is yet, but that's fine.  

It probably does not make sense to implement `undo` until there are actions
to be undone.  

The specification gives us a (mostly) very good idea for a data type for **ware**. 
- Name and description are clearly strings. 
- Price and number of items (which I will be calling amount from now on) are clearly integers.

The storage location is less clear, however.  
Rather that getting stuck on this detail,  
let's pass_the_buck and assume that there is a type `Storage_location_t` that solves the problem. 

Now, we can write the type for ware:

```rust
struct Ware {
  name: String,
  description: String,
  storage_location: Storage_location_t, 
  price: i32,
  amount: i32,
}
```

By pass_the_buck, we now have a type `ware`. However, in the
interest of making progress and working in an incremental fashion, we
are also going to apply another great simplification trick: **dodging**.

Getting information from the user at all is a great first step in an
incremental development methodology. Once that is done and works, we can
start caring about making sure that the information is correct. 

It is time to **dodge**.

Dodging means **temporarily** simplifying the specification. 

In the current example, a good dodge is to say: 
for now, I am going to only store string data in wares, 
and worry about integers and storage locations later. 

This allows us to use a single function for reading input from the user 
to complete the entire specification for adding a ware, modulo saving it to the database.

Temporarily, we change to type of `ware` to this:


```rust
struct Ware {
    name: String,
    description: String,
    storage_location: String, // TODO: storage_location_t, 
    price: String,            // TODO: i32,
    amount: String,           // TODO: i32,
}
```

And then we can make a version of `get_char_input()`, for example
`get_string_input()`. We will probably also pass_the_buck here and assume its
existence on the first pass through the code. 
We will also assume the existence of a `add_to_db()` function, 
which does nothing except prints out the obligatory disclaimer of it not yet being implemented.

Here is a list of suitable steps for almost implementing 
the simplified `add_action()`.
Each step notably results in a program that can compile and run, but
probably does not do anything useful.

1.  Read all the inputs as strings and save them into a ware object,
    assuming the existance of several functions (This could quite
    possibly be several increments.): 
    a. `get_string_input()`,

```rust
fn get_string_input(dialog_msg: &str) -> String {
    println!("{dialog_msg}");

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().to_string()    
}
```

We check if it builds `cargo build`.
Great it builds. Next:

 
    b. `print_ware()` (to print a ware to the user before answering the save, edit, abort), 

We can make a print_ware() function that takes a Ware type as a parameter or we go the idiomatic way and give 
Ware the method `print_ware()`. Let's go idiomatic, and also give it a constructor

```rust
impl Ware {
    fn new( name: String,
            description: String,
            storage_location: String, 
            price: String,
            amount: String,) -> Self {

        Ware {
            name,
            description,
            storage_location,
            price, 
            amount,
        }
    }

    //multi line printing is a bit messy
    fn print_ware(&self) {
        println!("\
name: {}
description: {}
storage_location: {}
price: {}
amount: {}
",          self.name, self.description, 
            self.storage_location, 
            self.price, self.amount);
    }//^-- fn print_ware
}
```

Lets try using these in `add_action`

```rust
//> 1.  **Adding** a ware
fn add_action() {
    println!("Add action simulation");
    let item = Ware::new("laptop".to_string(), 
                         "Top of the line gamer ware".to_string(), 
                         "a2".to_string(), //A storage location is a **section** (a single letter a-z), 
                                                //plus a **shelf** (an integer). 
                         "1240.00".to_string(), 
                         "1".to_string());
    item.print_ware(); 
}

```
Compile. Still good.

```sh
Welcome to my warehouse program!
--- Main menu ---

[A]dd a ware
[R]emove a ware
[E]dit the information about a ware
[P]rint the information for a specific ware
[L]ist all wares in the database
[U]ndo the last action
[Q]uit the program
Choose menu item!
A
Add action simulation
name: laptop
description: Top of the line gamer ware
storage_location: a2
price: 1240.00
amount: 1
```
Great!

    c. `edit_ware()`, (for when the user selects edit above) and 

We're going to use another dummy ware item for `edit_action`.
We migh as well extract a function that we can use in both `Add` and `Edit`.

```rust
fn make_dummy_item() -> Ware {
    Ware::new("laptop".to_string(), 
             "Top of the line gamer ware".to_string(), 
             "a2".to_string(), 
             "1240.00".to_string(), 
             "1".to_string()
    )
}
```
And `Edit_action` looks like this

```rust
//> 3.  **Editing** the information about a ware
fn edit_action() {
    println!("Edit action simulation");

    let mut item = make_dummy_item();

    println!("Item before edit:");
    item.print_ware();

    for k in ["name", "description", "location", "price", "amount"] {
        let ans = get_string_input(&format!("New {}: (enter/return to skip)", k));

        if ans !="" {
            match k {
                "name" => item.name = ans.to_string(),
                "description" => item.description = ans.to_string(),
                "location" => item.storage_location = ans.to_string(),
                "price" => item.price = ans.to_string(),
                "amount" => item.amount = ans.to_string(),
                _ => (),
            }
        }
    }

    println!("\nItem after edit:");
    item.print_ware();
}
```

    d. `add_to_db()`.

We have no db. Will use the file system to store our data in a text file.
We are going to save an item as a string in the text file.
The first thig we can notice is that we have something similar. `print_ware`
How about we create a `ware_to_string` function and use that in print_ware, so we can extract the functionality without breaking anything.

```rust
    
    fn print_ware(&self) {
        println!("{}", self.ware_to_string());
    }

    fn ware_to_string(&self) -> String {
        format!("\
name: {}
description: {}
storage_location: {}
price: {}
amount: {}
",          self.name, self.description, 
            self.storage_location, 
            self.price, self.amount)
    }//^-- fn ware_to_string
```

`cargo run` everything should still be ok.

Now we can use `ware_to_string` to generate the string to be save in a file.

```rust
use std::io::{Read, Write};
...
fn add_to_db(item: Ware) -> Result<(), std::io::Error> {
    // open the db file
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;
    // read its content into a new string
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    // append new item & save to file    
    content.push_str(&item.ware_to_string());
    // add delimiter
    content.push_str("\n");
    writeln!(f, "{}", content);

    Ok(())
}
...
fn add_action() {
    println!("Add action simulation");
    let item = make_dummy_item();
    item.print_ware();
    if let Err(e) = add_to_db(item) { // consume it  
        eprintln!("Couldn't write to file: {}", e);
    }
}
```


Next we can stop using the dummy item and ask the user to enter the real data
We need to ask for this in both Add and Edit. lets extract it

```rust
fn user_input(item: &mut Ware){
    for k in ["name", "description", "location", "price", "amount"] {
        let ans = get_string_input(&format!("New {}: (enter/return to skip)", k));

        if ans !="" {
            match k {
                "name" => item.name = ans.to_string(),
                "description" => item.description = ans.to_string(),
                "location" => item.storage_location = ans.to_string(),
                "price" => item.price = ans.to_string(),
                "amount" => item.amount = ans.to_string(),
                _ => (),
            }
        }
    }
}
```
And use like this

```rust
fn add_action() {
    println!("Add action");
    //let item = make_dummy_item();
    let mut item = Ware::new("name: ".to_string(),
                            "description: ".to_string(),
                            "storage_location: ".to_string(), 
                            "price: ".to_string(),
                            "amount: ".to_string()
    );
    user_input(&mut item);
    item.print_ware();
    if let Err(e) = add_to_db(item) { // consume it  
        eprintln!("Couldn't write to file: {}", e);
    }
}
...
fn edit_action() {
    println!("Edit action simulation");
    let mut item = make_dummy_item();
    println!("Item before edit:");
    item.print_ware();

    user_input(&mut item);

    println!("\nItem after edit:");
    item.print_ware();
}

```

Edit also needs to save to the db file. 
But its needs to know where it found the data to save.
Its seems we need to make `add_to_db` read the db and put each *record* in a string.

We give Ware the ability to turn its self into a record

```
   fn to_record(&self) -> String {

        format!("{};{};{};{};{}", 
                self.name,
                self.description,
                self. storage_location, 
                self.price,
                self.amount
        )
    }    
```


A new function `read_db` should read the db and return a vector of Strings. 


```rust
fn read_db() -> Result<String, std::io::Error> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;
    
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    
    Ok(content)
}

```


<!-- WE WENT TOO FAST. DO EDIT name ONLY first, no loop.  
4.  In 1. above, we are writing straight-line code. No loops, and the
    only branching is the calls to `edit_ware()` and `add_to_db()` based
    on the users input. This means we do not yet implement the case
    where the user makes an edit, gets a printout and makes the edit
    again. This support we add in this step.
-->

5.  Make price and amount ints in ware and replace the two calls to
    `get_string_input()` with calls to an assumed `get_int_input()`.

6.  Implement `get_int_input()`.

7.  Decide how to do the storage location. And implement that. This is
    likely several steps given that we cheated on the type for storage
    location before.

The last thing we will do is to deal with the database -- a final piece
of the puzzle missing to finish this action.

Attention


**Multiple Possible Ways Forward**

Undo is a good example of a choice in how we choose to go forward. 

For example, we could chose to implement undo for each action that we
implement, i.e,. after add we implement undo add, after edit we
implement undo edit, etc. 

Or we simply wait until all actions are implemented and implement undo 
for all of them in a single hit. 

There is no right or wrong here, just two different ways of working, each with
its own merit. 

For example, 
it is easy to argue that implementing undo X right after implementing X 
is the best way because at this point, we probably have the best understanding of how X works. 

On the other hand, it is easy to argue that if we implement the entire undo behaviour 
at the same time, it is easier to come up with a clean design for it that will work for all cases. 

If we started with a design for undo add, we might not yet see all the information that must be saved. 
(For example, undo edit will require us to store the original values of the edited data!)


### 3.9 Step 8: Think a Little -- Code a Little 

"Smart data structures and dumb code works a lot better than the other way around."  
_ Eric S. Raymond 

Choosing the proper database design is of course very important for the
program at hand. There are multiple alternatives ranging from simple to
hard, for example:

1.  A statically sized array of wares
2.  A dynamically sized in-memory structure like a list or a tree
3.  An external database back-end

There is however one very important realisation at this stage and that
is that right now, **it does not matter** (right now).

The rationale is that we are trying to piece by piece construct a piece
of software that fulfils the specification. We are just about to wrap-up
the part of the program that will be adding the first item in the
database. We don't have the code to remove items, edit them, etc. So
really, we could not care less at this point how the database is
designed.

This is a great opportunity to dodge again. This time, we are going to
dodge in three discrete steps:

1.  The database holds only one single element
2.  The database is an array of fixed maximal size
3.  Make the final decision once the program's done otherwise

The first database design will allow implementing everything except
adding and removing with multiple wares, and listing multiple wares. The
second design will allow us to do all that. The third design will make
the program less of a [Mickey Mouse
program](http://www.catb.org/jargon/html/M/mickey-mouse-program.html){target="_blank"}.

Following the first database design, we can now define a struct for the
database like so:

.......



### 3.10 Step 9, and beyond 

"Controlling complexity is the essence of computer programming."  
_ Brian Kernighan 

Think/code/refactor/remove cheats. Repeat until done. Find a good
balance.


### 3.11 Summary and Conclusions


"Those that can, do. Those that can't, complain."  
_ Linus Torvalds 


Almost all hard problems can be broken down into less-hard subproblems,
which in turn can be broken down to even less-hard subsubproblems, etc.
After a while of breaking work down into smaller and smaller tasks, the
tasks finally become simple, and that is the right time to start solving
them. That's SIMPLE in a nutshell.

As soon as we hit a bump in the road -- a problem we don't know
immediately how to solve, or don't know how to solve well, or just
cannot be bothered with solving right now -- we keep things simple by
cheating and dodging. We reserve ourselves that right by always making
clear notes about our cheats and dodges and by promising ourselves that
we will set things straight in the future. And by keeping our promises.

## 4 A Summary of SIMPLE in Ten Steps

Start with a high-level work breakdown structure
:   Mine your specification for data (nouns) and actions (verbs --
    behaviour/functions). Make simple drawings, e.g., mindmaps, to
    record your insights. Making things look easy is almost always good.

Write code to test the validity of your thinking
:   ...not to drive the thinking! Thinking should invariably come before
    coding, especially thinking about how you check that your thinking
    is valid.

Always have a working program
:   In combination with cheating, this usually means inserting dummy
    functions.

Compile after every change
:   Fix errors now, not later. Fix errors one by one. Fix errors in the
    order they were printed. Take time to actually read the compiler
    message so you know you are fixing the right thing.

Run the program "all the time" to spot errors
:   This requires always working code -- preferably in combination with
    automated test that don't involve ocular inspection, etc.

Recursively break your problems up into smaller sub problems
:   Only start solving problems when they start feeling easy. Make a
    task for each problem or subproblem to put on the stack, take tasks
    from the stack in a reasonable order (preferably easiest first);
    when the stack is empty -- you are done!

Break each task up into increments and start with the easy ones
:   Generate new tasks to put on the stack as you go. If suitable, start
    with a straight-line version (without any if-statements). When the
    straight-line version works add conditionals, one by one. Start with
    the most basic or the most insteresting cases. When you are writing
    a loop, do the above steps first and add the looping step last.

Whenever you run the risk of getting stuck, cheat
:   Don't forget to push new tasks on the stack that undoes the
    cheating, later. This records the cheat, which is great.

Use dodging to help breaking complex cases up into several less complex ones
:   Don't forget to push new tasks on the stack that undoes the dodge,
    later. This records the dodge, which is great.

Alternate between thinking, coding and refactoring

:   -   thinking -- not so much that you get stuck, though,
    -   coding -- but never without first thinking about what to code,
        and occasionally
    -   refactoring -- especially to address your cheats and dodges.

    Refactoring is good to do between larger tasks. Make it a habit to
    continuously go back and refactor your solutions so that recently
    gotten insights rub off on older code too.




