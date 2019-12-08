<img class="topPix" src="../pix/computer-brain-interface.svg" alt="Writings" />

<div class="container">
<header class="main-header clearfix">

<nav class="main-menu">
<ul>
<li class="main-menu__item">[Home](/)</li>
<li class="main-menu__item">[Selected writings](/writings/)
</li>
<li class="main-menu__item">Programming
<ul><li>Rust Introduction</li></ul>
</li>
<li class="main-menu__item">[About](/about.html)</li>
</ul>
</nav><!-- nav -->
</header><!-- header -->

<span id="top"></span>
<section class="sponsors-wrapper clearfix">
<main class="content-area">
&lt; <a href="./index.html#prog_txt">back to TOC</a>

# One Language, two learning tracks

There is alot of material out there to learn Rust. 
However until recently Rust was a moving target that kept transforming itself for the better. 
But that also means that alot of tutorials floating around on the web are obsolete.

Hence its wise to <a href="https://www.rust-lang.org/learn" target="_blank"> have a look at the official information.</a>
It should be your starting point if you have experience in any other programming language.

With the 2018 edition, Rust vows to be stable, dependable, without braking changes. 
So now programmers and developers can safely invest time to learn and play with this wonderful modern computer language.

<a href="https://www.rust-lang.org/tools/install" target="_blank">[First start here: `Install Rust`]</a> and come back 
to learn by doing.

- If you are totally new to programming or you want to learn Rust in a more leasurly and slower pace,  
 you can embark on a learning journey by playing <a href="http://razafy.com/index.html#tRPG">`The Rust Programming Game` (tRPG)</a>
 
- If you have experience in other languages and want to learn  
 without too much non essential information to start with take the `Rapid Tour Of Rust`.  

This both kind of practical learning will harmoniously complement 
the wonderful <a href="https://doc.rust-lang.org/stable/book/" target="_blank">Official Rust documentation ("The Book")</a> 
and contributions by the Rust community.

---

# A Rapid Dive into Rust

Our Learning Progression will follow the `read & type`, `type & correct`, `complete & run` pattern.

## Preparing for action
Basic facts to get started

we need to SHRINK THIS! 5 code samples 
1.Basic, mut, Operators
2. Scope, Owership, Borrowing, 

- Basic data types
    * Integer Types
    * Floats
    * Bool
    * Char
    * &str
    * Unit type
    * Array
    * Tuples
- Mutability
- Operators
  * Arithmetic Operators : `+ - * / %`
  * Comparison Operators : `== != < > <= >=`
  * Logical Operators : `! && ||`
  * Bitwise Operators : `& | ^ << >>`
  * Assignment and Compound Assignment Operators
  * Type Casting Operator : as
- Scope and Functions
- Ownership and Borrowing
  * ownership
  * Slice
  * borrow checker
- Flow Control
  * Conditional
    + if - else if - else
    + match
  * Repetitions
    + loop
    + while
    + for 
- rustc
- Stack and Heap Memory
- Strings, Vectors
- Struct and Enum
- Type system
- Traits and Methods
- Option Result
- Option Error

### Basic data types

REPHRASE_ME: Variables in Rust are associated with a specific data type. 
The data type determines the size and layout of the variable's memory, 
the range of values that can be stored within that memory 
and the set of operations that can be performed on the variable.


#### Integer Types in Rust

| **Length**    | **Signed**    | **Unsigned**  |
|:----------|:----------|:----------|
| *8-bit*     | i8	    | u8        |
| *16-bit*    | i16	    | u16       |
| *32-bit*	| i32	    | u32       |
| *64-bit*	| i64	    | u64       |
| *128-bit*	| i128	    | u128      |
| *arch*	| isize	    | usize     |

#### Floating-Point Types

The default type is f64

| **Size**   |       |  precision |
|--------|-------|-------------|
| *32-bit* | f32   | Single |
| *64-bit* | f64   | Double |

#### bool

booleans true and false

```rust
let is_happy = true;
let is_sad: bool = false;
```

#### char
characters 'a'

```rust
let x = 'x';
let cool = '😎';
```

#### &str
string literals "abc",

#### Unit Type
the unit type ()

#### Array

fixed-size list of elements of same data type
 

let a = [1,2,3,2020];
let b = ["hello", "world"];
let c = ["repeat_me"; 3]; // ["repeat_me", "repeat_me", "repeat_me"]
let mut e: [i32; 0] = []; //empty array ready to accept i32 type elements

note 1: that index is usize and starts at 0 as in:
```
a[0] // 1
b[0] // "hello"
c[0] // "repeat_me"
d[0] // []
```

REPHRASE_ME: Array indexing starts at 0 and bounds checking is done at runtime. 
Bounds checking is used to prevent accessing memory that is out of bounds, 
for instance, trying to access the element after the end of an array.


note 2: Arrays are fixed size. Its element count can not be changed. `mut` only the elements mutable.
We use Vectors for dynamic sized collections.

#### Tuples 
fixed-size ordered list of elements of different(or same) data types

let a = (1, 1.5, (true, false), 'a', "Hello, world!");

### Mutability
Safety and easy concurrency being a primary reason for Rust, variables are Immutable by Default, hence 
the term Variable bindings is often used.
 
To make Variable bindings mutable, an explicit use of the keyword `mut` is mandatory. 

### Operators
#### Arithmetic Operators : `+ - * / %`
Note `+` is also used for array and string concatenation

#### Comparison Operators : `== != < > <= >=`
#### Logical Operators : `! && ||`
#### Bitwise Operators : `& | ^ << >>`
#### Assignment and Compound Assignment Operators
#### Type Casting Operator : `as`

### Scope and Functions
#### Scope 
Block scope is delimited by `{` between `}` or a module (source file)

#### Function

REPHRASE_ME: Functions are declared with the keyword fn
▸ When using arguments, you must declare data types.
▸ By default functions return empty tuple (). If you want to return a value, return type must be specified after ->



#### Const
Constants have a global scope and represent values that cannot be changed.

const PAY: f32 = 65.0;

### Ownership and Borrowing
#### Slice
<quote>Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.</quote>_the_book  
REPHRASE_ME: A slice is a view into a contiguous sequence: it can be a view of the whole array, or a part of it.

```rust
let a = [10, 20, 30, 40, 50];
let first_and_second_elem = &a[0..3]; // [10, 20]
let another_way_first_and_second_elem = &a[..3]; // [10, 20]
let third_to_last_elem = &[2..]; // [30, 40,50]
let the_fifth_element = &a[4]; // [50]
```

#### borrow checker

### Flow Control 
#### Conditional
##### if - else if - else
##### match
#### Repetitions
##### loop
##### while
##### for
Use the `for` loop over elements of an iterator, a conviniant structure to access the elements of a collection or sequence of values.

### rustc
### Stack and Heap Memory
### Strings, Vectors
### Struct and Enum
### Type system
### Traits and Methods
### Option Result
### Option Error

# Action time 

Type this code listing and correct it until you get the following output

| 3 + 1 = 4
| 14 + 1 = 15
| 15 + 1 = 16
| 19 + 1 = 20


```rust
// filename: add_one_to_an_integer.rs                                     ➊     ❶
//
// USAGE 
//      Compile with:  
//            rustc add_one_to_an_integer.rs
//      Run with:     
//            ./add_one_to_an_integer


/// Increment by one                                                           ❷
/// Expect an integer, returns an integer
///
/// asserteq!(4, add_one_to_an_integer(3)); 
///
fn add_one_to_an_integer(val: i32) -> i32 {                                    ❸
    val + 1                                                                 ❹
}


fn main() {                                                                    ❺
    let input = vec![3,14,15,19];                                           ❻
    
    for i in input.iter() {                                                 ❼
        println!("{} + 1 = {}", i, add_one_to_an_integer(*i) );             ❽
    }
    
}

/* --- OUTPUT
3 + 1 = 4
14 + 1 = 15
15 + 1 = 16
19 + 1 = 20
*/
```

❶  
❷   
❸  
❹  
❺  
❻  
❼  
❽  

</main>
</section><!-- sponsors-wrapper -->

</div><!-- container -->

<footer class="footer">


-   [zoom]()
-   [email](mailto:learningrustrpg@gmail.com)
-   [github.com/lerina](https://github.com/lerina)


<div id="copy"><em>&#xa9;</em> 2019  &nbsp; <a href="http://razafy.com" target="_blank"> <span class="le">le</span><span class="ri">ri</span><span class="na">na</span>  ^_^ </a></div>

</footer><!-- footer -->
