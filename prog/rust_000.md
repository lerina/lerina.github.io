

There is alot of material out there to learn Rust. However until recently Rust 
was a moving object that kept transforming itself for the better. 
But that also means that alot of tutorials are obsolete.

Hence its wise to <a href="https://www.rust-lang.org/learn" target="_blank"> have a look at the official information to get relevant up-to-date suggestions</a>

With the 2018 edition, Rust vows to be stable, dependable, without braking changes. 
So now we can safely invest time to learn and play with this wonderful modern computer language.

<a href="https://www.rust-lang.org/tools/install" target="_blank">[First start here: `Install Rust`]</a> and come back to learn by doing, old style.

- If you want to learn Rust in a more leasurly and slower pace,  
 you can embark on a learning journey by playing <a href="http://razafy.com/index.html#tRPG">`The Rust Programming Game` (tRPG)</a>
 
- If you have experience in other languages and want to learn  
 without too much non essential information to start with take the `Rapid Tour Of Rust`.  
Our Learning Progression will follow the `read & type`, `type & correct`, `complete & run` pattern.

This both kind of practical learning will harmoniously complement the wonderful Official Rust documentation ("The Book") and contributions by the Rust community.

---

## A Rapid Tour Of Rust

Type this code listing and correct until you get the following output

| 3 + 1 = 4
| 14 + 1 = 15
| 15 + 1 = 16
| 19 + 1 = 20

```rust
// filename: add_one_to_an_integer.rs
//
// USAGE 
//      Compile with:  
//            rustc add_one_to_an_integer.rs
//      Run with:     
//            ./add_one_to_an_integer

fn add_one_to_an_integer(val: i32) -> i32 {
    val + 1
}

fn main() {
    let input = vec![3,14,15,19];
    
    for i in input.iter() {
        println!("{} + 1 = {}", i, add_one_to_an_integer(*i) );
    }
    
}

/* --- OUTPUT
3 + 1 = 4
14 + 1 = 15
15 + 1 = 16
19 + 1 = 20
*/
```
