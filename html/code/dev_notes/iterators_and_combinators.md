<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
⇦ [lerina.github.io](../../../index.html) - [code](../index.html)  

# Iterators and Combinators

---

        //function combinators example
        fn main() {
         let vector = vec![1,3,4,5,3];

         //all
         //all is basically folding with && and since the identity element for && is true 
         //it is what gets returned by all. 

         let greater_than_zero = vector.iter().all(|&x| x > 0 ) ;
         println!("greater than zero {}",greater_than_zero);

        let a: Vec<i32> = vec![1, 2, 3, 4];
        println!("{}", a.into_iter().all(|x| x > 1));

        // any on the other hand is a fold with || and the identity for || is false 


         //list of words using flat map
         let lines_vec = vec!("hello,how","are,you");
         let words_vec = lines_vec.iter().flat_map(|&x| x.split(",")).collect::<Vec<&str>>();
         println!("{:?}", words_vec);
        }

---

## Iterators

### Creating Iterators in Rust

We can create an iterator by converting a collection into an iterator. 
There are three ways to create an iterator.

1. Owned iterator using `into_iter()` method
2. Borrowed iterator using `iter()` method
3. mutable iterator using `iter_mut()` method

### Owned iterator

Using the into_iter() method on a collection will iterate on the same element of the collection in each iteration. Thus, the collection will no longer be available for reuse as the value moves within the loop.

```rust
fn main () {
    let v1 = vec![1, 2, 3,];

    // consume v1
    for x in v1.into_iter() {
        println!("{}", x);
    }

}
```



Note: By default the `for` loop will apply `into_iter()` to the collection. 

Thanx to *sugar syntaxing*, these two ways to loop through an iterator are the same.

```rust
for item in mycollection.into_iter() {
    // code
}

for item in mycollection {
    // code
}
```

### Borrowed iterator

Using the `iter()` method on a collection will borrow (reference) each element of the collection in each iteration. Thus, the collection will be available for use after we have looped through it.

```rust
fn main () {
    let v2 = vec![1, 2, 3,];

    // borrow v2
    for x in v2.iter() {
        println!("{}", x);
    }

    println!("We can still use v2: {:?}", v2);
}
```

### mutable iterator

Using the `iter_mut()` method on a collection will mutably borrow each element of the collection in each iteration. It means we can modify the collection in place.

```rust
fn main() {
    let mut languages = vec!["Python", "Ruby", "C"];
    
    // using iter_mut() to iterate through a collection
    for language in languages.iter_mut() {
        // modify the item in the collection
        if *language == "Ruby" {
            *language = "Rust";
        }
        println!("{}", language);
    }

    
    // the modified collection is available here
    println!("languages = {:?}", languages);
}
```

### Iterators are lazy

All Rust collection types (arrays, vectors, maps, ...) are not iterable by default. 
Using the `iter()`/`iter_mut()`/`into_iter()` methods we get an iterable.  

```rust
fn main() {
    let v = vec!["Hello", "my", "friend", "!"].into_iter();

    v.for_each(|word| println!("{}", word) );
}
```

## Iterator adapters

Iterator adapters are used to transform it into another kind of iterator by altering its behavior

### collect

Iterator to collection

`collect()` can take anything iterable, and turn it into a relevant collection. 


```rust
fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

    let y: Vec<u64> = x.collect();

    println!("{}",y[9]);
}
```
### from_iter

```rust
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();

    let hm: HashMap<u64, u64> = HashMap::from_iter(x);
    println!(":?", hm);
}

```

### next

The next() method is used to fetch individual values from the iterator.

The next() method of an iterator can be used to traverse through the values in the iterator.
Every iterator in Rust must implement a next() method. 

The next() method either returns Some value or None.
None is returned when the iterator reaches the end of the sequence

### reduce

`reduce` accumulates over an iterator by applying a closure
If the iterator is empty, returns None; otherwise, returns the result of the reduction.

reduce() can be used to use the first element as the initial value, 
if the accumulator type and item type is the same.

```rust
fn main() {
    // reduce
    let values = vec![1, 2, 3, 4, 5].into_iter();

    let sum = match values.clone().reduce(|acc, x| acc + x){
        Some(s) => s,
        None => 0,
    };
    println!("sum of {:?} is: {sum}", values);
}
```


### fold

Folds every element into an accumulator by applying an operation, returning the final result.

fold() takes two arguments: an initial value, and a closure with two arguments: an ‘accumulator’, and an element. The closure returns the value that the accumulator should have for the next iteration.

The initial value is the value the accumulator will have on the first call.

After applying this closure to every element of the iterator, fold() returns the accumulator.

_ [Rust doc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

```rust   
    let a = [1, 2, 3];

    // 10 + the sum of all of the elements of the array
    let sum_plus_10 = a.iter().fold(10, |acc, x| acc + x);
    println!("sum plus 10 of {:?} is: {sum_plus_10}", a);

```

## Combinators

### filter

Creates an iterator which uses a closure to determine if an element should be yielded.

Given an element the closure must return true or false. The returned iterator will yield only the elements for which the closure returns true.  
[Rust doc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

```rust
fn main() {
    let v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let even_v: Vec<&i32> = v.iter().filter(|&x| x % 2 == 0 ).collect();
                           //v.iter().filter(|&x| x % 2 == 0 ).collect::<Vec<&i32>>();

    println!("even_v: {:?}", even_v);
}
```


### inspect

```rust
fn main() {

    let v = vec![-1, 2, -3, 4, 5].into_iter();
    println!("\ninitial v: {:?}", v);

    let positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();

    println!("positive_numbers: {:?}",positive_numbers);
}
```


### map

Takes a closure and creates an iterator which calls that closure on each element.

map() transforms one iterator into another, by means of its argument: something that implements FnMut. It produces a new iterator which calls this closure on each element of the original iterator.

map() is conceptually similar to a for loop. However, as map() is lazy, it is best used when you’re already working with other iterators. If you’re doing some sort of looping for a side effect, it’s considered more idiomatic to use for than map().  

```rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

Note:

```rust
// don't do this:
(0..5).map(|x| println!("{x}"));

// it won't even execute, as it is lazy. Rust will warn you about this.

// Instead, use for:
for x in 0..5 {
    println!("{x}");
}
```
_ [Rust doc](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)


```rust

fn main() {

    let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v.map(|x| x.to_string().to_uppercase()).collect();
    println!("{:?}", w);
}
```
```rust
fn main() {
    let v = vec![1,3,4,5,3];

    //map values
    let map_add_1 = v.iter().map(|&x| x +1).collect::<Vec<i32>>();
    println!("{:?}", map_add_1);
}
```

### filter_map

Creates an iterator that both filters and maps.

The returned iterator yields only the values for which the supplied closure returns Some(value).

`filter_map` can be used to make chains of filter and map more concise

This

```rust
let a = ["1", "two", "NaN", "four", "5"];

let mut iter = a.iter().filter_map(|s| s.parse().ok());

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(5));
assert_eq!(iter.next(), None);
```

Instead of this

```rust
let a = ["1", "two", "NaN", "four", "5"];

let mut iter = a.iter().map(|s| s.parse())
                       .filter(|s| s.is_ok())
                       .map(|s| s.unwrap());

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(5));
assert_eq!(iter.next(), None);
```

filter_map has the advantage of dealing with Option instead of bool

```rust
fn main() {
     let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {                         // filter
                Some(String::from(x).to_uppercase()) // map
            } else {
                None
            }
        })
        .collect();

    println!("{:?}. living out the `!`", w);
}
```
### count

```rust
fn main() {
    let v = vec![1,3,4,5,3];

    //count number of items
    let item_count = v.iter().count();
    println!("count  is {}",item_count);
}
```

### max

```rust
fn main() {
    let v = vec![1,3,4,5,3];

    //find max 
    let max = v.iter().max().unwrap();
    println!("max is {}",max);
}
```
### chain

Takes two iterators and creates a new iterator over both in sequence.

chain() will return a new iterator which will first iterate over values from the first iterator and then over values from the second iterator.

In other words, it links two iterators together, in a chain:
chain merges two iterators.

```rust
fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut both = a1.iter().chain(a2.iter());
    for _ in 0..7 {
        println!("{:?}", both.next());
    }
}
```


### flatten

Creates an iterator that flattens nested structure.

This is useful when you have an iterator of iterators or an iterator of things that can be turned into iterators and you want to remove one level of indirection.

```rust
fn main() {
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    println!("{:?}", data);
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    println!("{:?}", flattened);
}
```

### Zip

MOD THIS

```rust
fm main() {
    let vector = vec![1,3,4,5,3];

    //zip with index
    let index_vec = 0..vec_count;
    let index_zipped_vector = vector.iter().zip(index_vec).collect::<Vec<(&i32,usize)>>(); 
    println!("zipped vector is {:?}",index_zipped_vector);
}
```

MOD NEEDED

## Option

### unwrap_or

Use a default value

```rust
fn option_unwrap_or() {
    let _port = std::env::var("PORT").ok().unwrap_or(String::from("8080"));
}
```

### or

Use a default Option value

```rust
// config.port is an Option<String>
let _port = config.port.or(std::env::var("PORT").ok());
// _port is an Option<String>
```

### and_then

Call a function if Option is Some

```rust
fn port_to_address() -> Option<String> {
    // ...
}

let _address = std::env::var("PORT").ok().and_then(port_to_address);
```
### or_else

Call a function if Option is None

```rust
fn get_default_port() -> Option<String> {
    // ...
}

let _port = std::env::var("PORT").ok().or_else(get_default_port);
```

### is_some 

Returns true if an Option is Some(value)

```rust
let a: Option<u32> = Some(1);

if a.is_some() {
    println!("will be printed");
}

let b: Option<u32> = None;

if b.is_some() {
    println!("will NOT be printed");
}
```

### is_none 

Returns true is an Option is None

```rust
let a: Option<u32> = Some(1);

if a.is_none() {
    println!("will NOT be printed");
}


let b: Option<u32> = None;

if b.is_none() {
    println!("will be printed");
}
```

## Result

### ok

Convert a Result to an Option

```rust
fn result_ok() {
    let _port: Option<String> = std::env::var("PORT").ok();
}
```

### or
Use a default Result if Result is Err

```rust
fn result_or() {
    let _port: Result<String, std::env::VarError> =
        std::env::var("PORT").or(Ok(String::from("8080")));
}
```

### map_err 

Converts a `Result<T, E>` to a `Result<T, F>` by calling a given function

```rust
fn convert_error(err: ErrorType1) -> ErrorType2 {
    // ...
}


let _port: Result<String, ErrorType2> = std::env::var("PORT").map_err(convert_error);
```

### and_then

Call a function if Results is Ok

```rust
fn port_to_address() -> Option<String> {
    // ...
}

let _address = std::env::var("PORT").and_then(port_to_address);
```

### map_or

Call a function and default value

```rust 
let http_port = std::env::var("PORT")
    .map_or(Ok(String::from("8080")), |env_val| env_val.parse::<u16>())?;
```

### map

Chain a function if Result is Ok

```rust
let master_key = std::env::var("MASTER_KEY")
    .map_err(|_| env_not_found("MASTER_KEY"))
    .map(base64::decode)??;
```


### is_ok 

Returns true is an Result is Ok

```rust
if std::env::var("DOES_EXIST").is_ok() {
    println!("will be printed");
}

if std::env::var("DOES_NOT_EXIST").is_ok() {
    println!("will NOT be printed");
}
```

### is_err 

Returns true is an Result is Err

```rust
if std::env::var("DOES_NOT_EXIST").is_err() {
    println!("will be printed");
}

if std::env::var("DOES_EXIST").is_err() {
    println!("will NOT be printed");
}
```

## Composing combinators


- Try to parse an array of strings into numbers
- filter out invalid results
- filter numbers less than 0
- collect everything in a new vector

```rust
fn main() {
    let a = vec![
        "1",
        "2",
        "-1",
        "4",
        "-4",
        "100",
        "invalid",
        "Not a number",
        "",
    ];

    println!("before: {:?}", a);

    let only_positive_numbers: Vec<i64> = a
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .filter(|x| x > &0)
        .collect();

    println!("after: {:?}", only_positive_numbers);
}
```

This replaces a big loop with complex logic with a few lines of idiomatic Rust.

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
