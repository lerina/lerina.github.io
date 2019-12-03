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

- basic data types
    * Integer Types
    * Floats
    * bool
    * char
    * &str
    * String
- scope
- mutability
- borrow checker
- rustc

### Basic data types

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

| **Size**   |       |
|--------|-------|
| *32-bit* | f32   |
| *64-bit* | f64   |



### Scope





### Variables are Immutable by Default
Safety and easy concurrency being a primary reason for Rust, 
by default variables are immutable. An explicit notation to make them
mutable is touse the keyword `mut`. 

### Borrow Checker

### Compiling with rustc


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
