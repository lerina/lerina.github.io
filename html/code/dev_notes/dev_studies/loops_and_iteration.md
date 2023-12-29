# Loops and iteration

[source](https://fitech101.aalto.fi/programming-languages/rust/7-loops-and-iteration/)

## Looping

We often want to execute one or more statements in our code more than once. Programming code instructions that allow us to repeatedly execute parts of code are usually called loops. Rust provides us with three types of loops: loop, while, and for.

### Loop

The simplest form of a loop in Rust is an infinite loop. By placing the keyword loop in front of a block expression that serves as the body of the loop, we can infinitely repeat executing the block expression.

```rust
loop {
    println!("Waiting for a cosmic ray to hit the CPU and exit this loop...");
}
```
In many cases we don't want to loop forever, but stop at some point. Breaking a loop can be done with a break statement in the body of the loop — return would also work, but that would exit the enclosing function too, not just the loop.

```rust
fn main() {
    let mut i = 0;
    let mut p = 1;
    println!("The powers of 2:");
    loop { // infinite loop
        println!("2^{}={}", i, p);
        i = i + 1;
        let np = 2 * p; // next p
        p = np;
        if p > 100 {
            break;
        };
        // end of the body of the loop -> jump to the beginning of the loop
    }
}
```

Like if statements, loops too can be used as expressions. The value of the loop expression is the value specified with break <value>; (using break works just like using return, only it returns from a surrounding loop expression and not the surrounding function).

```rust
fn main() {
    let mut i = 0;
    let mut p = 1;
    let limit = 99;
    let (v, e) = loop {
        i = i + 1;
        let np = 2 * p; // next p
        if np > limit {
            break (p, i + 1); // break with value
        }
        p = np;
    };
    println!("The biggest power of 2 below {limit} is 2^{e}={v}");
}
```

We may also want to skip the rest of the loop body and jump to the beginning of the loop. This can be done with the continue statement.

```rust
fn main() {
    let mut year = 2023;
    loop {
        year += 1;
        if year % 4 != 0 {
            continue;
        }
        if year % 100 == 0 && year % 400 != 0 {
            continue;
        }
        println!("The next leap year is {}", year);
        break;
    };
}
```

### While

The while loop, while very similar to loop, requires an additional stopping condition in the form of while <boolean expression> {<body of the loop>} — loop is equivalent to while true. The body of the loop is executed as long as the condition evaluates to true.

```rust
fn main() {
    let mut i = 0;
    let mut p = 1;
    println!("The powers of 3:");
    while p < 100 {
        println!("3^{}={}", i, p);
        i += 1;
        p *= 3;
    }
}
```

---

> Cohersing a do_while

The condition of a while-loop can be any expression (including one with side effects such as mutating variables) as long as the expression evaluates to a boolean value. Blocks are expressions in Rust, which means that we can do a dirty trick to turn Rust's "while do" into a "do while" loop.

```rust
fn main() {
    let mut i = 0;
    let mut p = 1;
    println!("The powers of 3:");
    while {
        // do
        println!("3^{}={}", i, p);
        i += 1;
        p *= 3;
        // while
        p < 2000
    } {
        // the actual body of the loop
        // do nothing here
    }
}
```

While this works, it is heavily frowned upon because it is much less readable than the Rust's while when used normally. Please refrain from using such hacks.

---

The break and continue statements work in while loops just like with loop.

```rust
fn main() {
    let mut year = 2000;
    let mut leap_years = vec![];
    while year < 2023 {
        year += 1;
        if year % 4 != 0 {
            continue;
        }
        if year % 100 == 0 && year % 400 != 0 {
            continue;
        }
        leap_years.push(year);
    };
    println!("The leap years between 2000 and 2023 are {leap_years:?}");
}
```

### For

The third type of loop in Rust is the for loop that can be used to iterate over a set of values. The for loop can be much more concise than loop or while, for instance when we want to loop over a range of values.

Similarly as we used ..= for a range pattern previously, we can use the same syntax to create a range of values to iterate over with for.

```rust
fn main() {
    for i in 0..=9 {
        print!("{i}");
    }
}
```

The same code using a while loop would look something like

```rust
fn main() {
    let mut i = 0;
    while i <= 9 {
        print!("{i}");
        i += 1;
    }
}
```

Leaving out the = in the range pattern would create a range that does not include the last value, so 0..3 would iterate over the values 0, 1, and 2.

```rust
fn main() {
    for i in 0..10 {
        print!("{i}");
    }
}
```

--- 

Note:
The ..= syntax is used for both ranges and patterns. 
In patterns, however the = is required due to current compiler limitations (overlap with slice syntax parsing). 
Trying to use an exclusive range as a pattern causes an error.

```rust
fn main() {
    let x = 5;
    match x {
        0..10 => println!("x is between 0 and 9"), // Cause error!
        // use  0..=9 => ... 
        _ => println!("x is not between 0 and 10"),
    }
}
```

---

Besides ranges, we can use a for loop to iterate over other sets of values like vectors, arrays and hash maps.

The next example iterates over an array of degrees Celsius to create a new vector of the degrees converted to Fahrenheit.

```rust
pub fn celsius_to_fahrenheit(f: f64) -> f64 {
    f * 9.0 / 5.0 + 32.0
}

fn main() {
    let celsiuses = [1.0f64, 3.0, 10.0, 100.0];

    let mut fahrenheits = vec![];
    for celsius in celsiuses {
        fahrenheits.push(celsius_to_fahrenheit(celsius));
    }
    println!("C: {celsiuses:?}");
    println!("F: {fahrenheits:?}");
}
```


Using for to iterate over a hash map gives a tuple of the key and value of each entry.

```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([
        ("bear", "🐻"),
        ("wolf", "🐺"),
        ("fox", "🦊"),
    ]);

    for (animal, emoji) in map {
        println!(":{animal}: -> {emoji}");
    }
}
```

### Modifying through iteration

Using a for loop on an iterable collection implicitly calls the into_iter method of the collection to get an iterator of immutable values over the collection. The for loop then iterates over the values in the iterator.

To be able to modify values in a mutable collection using a for loop, we can call the iter_mut method to get an iterator over mutable references to the collection values.

In the next example, the array of degrees Celsius are converted in-place to degrees Fahrenheit.

```rust
pub fn celsius_to_fahrenheit(f: f64) -> f64 {
    f * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut temperatures = [1.0f64, 3.0, 10.0, 100.0];

    for temp in temperatures.iter_mut() {
        *temp = celsius_to_fahrenheit(*temp);
    }
    println!("{temperatures:?}");
}
```

The HashMap collection has some more options that allow us to easily iterate over just its values or keys (check out the documentation). For instance, we can loop over mutable references to the values of the map with the help of the values_mut method.

```rust
use std::collections::HashMap;
const INFLATION_RATE: f32 = 1.07;

fn inflate_prices(map: &mut HashMap<&str, f32>, years: i32) {
    for price in map.values_mut() {
        *price = *price * (INFLATION_RATE.powi(years));
    }
}

fn main() {
    let mut food_prices = HashMap::from([("beetroot", 1.2), ("cabbage", 1.1), ("carrot", 1.0)]);
    println!("before inflation {food_prices:#?}");
    inflate_prices(&mut food_prices, 10);
    println!("after 10 years of inflation {food_prices:#?}");
}
```

The iter_mut method also would works on HashMap but it would give us mutable references to both the keys and values instead of just the values.

Remember that when passing a mutable value to a function as a reference, the reference needs to be explicitly marked as mutable with &mut. Otherwise the function will receive an immutable reference and the values cannot be modified therein. We also need to dereference the references with the * operator.


## Iterators and adapters

We already familiarized ourselves with iterators when working with for to iterate over collections, but let's now dive a bit deeper into the topic.

Iterators are types that provide useful methods for inspecting and processing iterable data structures like arrays and vectors. The iterator methods come in two flavours: iterator adapters are methods that return a new (possibly modified) iterator and iterator consumers are methods that consume the iterator. With the help of iterator adapters and consumers, iterators can often be used to do same things as for loops but more expressively and concisely using functional syntax.

Iterators in Rust are an example of zero-cost-abstractions. They offer convenient functionality while posing no additional computational overhead compared to operating directly on the collection.

Let's look at how we can convert a list of degrees Celsius to degrees Fahrenheit, like we did before, but this time using the iterator consumer for_each.

```rust
pub fn celsius_to_fahrenheit(f: &mut f64) {
    *f = *f * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut temperatures = [1.0f64, 3.0, 10.0, 100.0];

    temperatures
        .iter_mut() // .iter_mut() returns an Iterator that iterates over mutable references
        .for_each(celsius_to_fahrenheit);

    println!("{temperatures:?}");
}
```

The for_each method of Iterator takes in a function and applies it on each value in the iterator. Notice that we used the same iter_mut as with the for loop. However, we needed to modify the celsius_to_fahrenheit function to take in a mutable reference to the value.

### Consuming iterators

Let's now take a look how we can use the adapter map to transform the values from an iterator.

```rust
fn miles_to_kilometers(miles: &f64) -> f64 {
  miles * 1.609344
}

fn main() {
    let miles = [1.0f64, 3.0, 10.0, 100.0];
    let kilometers = miles
        .iter() // .iter() returns an Iterator over immutable references
        .map(miles_to_kilometers);
    println!("{kilometers:?}");
}
```

By running the code, we see that calling map didn't actually apply the given function but the values in the iterator appear to be the same as original. This is because iterators are lazy and do not compute their values until they are consumed. Iterator laziness has multiple benefits in terms of performance, especially when chaining adapters or when needing only some of the values from an iterator.

To get the values from the iterator, we can use the collect method of the iterator. This method consumes the iterator and collects the values into a collection. The solution to the above example shows how to collect the values into a Vec<f64>.

When we want to iterate over owned values, we can use the into_iter method. This moves or copies the values of a collection into the returned iterator, depending on whether the collection is copiable. If the collection owns the values (is not e.g. a slice), the iterator values are also owned.

```rust
fn celsius_to_fahrenheit(f: f64) -> f64 {
    f * 9.0 / 5.0 + 32.0
}

fn main() {
    let celsiuses = [1.0f64, 3.0, 10.0, 100.0];
    let fahrenheits = celsiuses
        .into_iter()
        .map(celsius_to_fahrenheit)
        .collect::<Vec<f64>>();

    println!("degrees Celsius: {celsiuses:?}");
    println!("degrees Fahrenheit: {fahrenheits:?}");
}
```

Here, instead of specifying the type of the fahrenheits variable, we used the "turbofish" syntax ::<> to specify the output type of the collect method.


---

> Generic function output and the turbo::fish

Some methods, like String's parse and Iterator's collect, have generic output types. Whenever they are used, the compiler requires us to specify the target type of the output. This can be done either by explicitly specifying a type of the output variable

```rust
fn main() {
    let number: i32 = "42".parse();
    println!(r#"Thus "42" becomes {number:?}"#);
}
```

or by using the "turbofish" syntax `::<>`.

```rust
fn main() {
    let number = "42".parse::<i32>();
    println!(r#"Thus "42" becomes {number:?}"#);
}
```

In some cases, the type information can also be inferred from the context, like when returning a value from a function.

```rust
fn power_up(values: &[i32], power: u32) -> Vec<i32> {
    values
        .iter()
        .map(|v| v.pow(power))
        .collect()
}
fn main() {
    let values = [1, 2, 3];
    let powers = power_up(&values, 2);
    println!("{powers:?}");
}
```

Sometimes some of the type information can be inferred, but not all of it. In such cases, we can omit writing the whole type explicitly by using the _ symbol for the inferrable part(s).

```rust
fn main() {
    let numbers = [-3i32, -2, -1, 0, 1, 2, 3];
    let absolutes = numbers
        .into_iter()
        .map(i32::abs)
        .collect::<Vec<_>>();
    println!("{absolutes:?}");
}
```

In the example above, the iterator is collected to a Vec and the type of the elements is inferred from the type of the iterator, which in turn is inferred from the type of the array numbers.

---

In case we need owned values, but into_iter returns references, we can use the copied or cloned method (depending on the type) to get owned copies of the values — we could of course alternatively use map with a function that calls to_owned() on its parameter.

```rust
fn are_positive(numbers: &[i32]) -> Vec<bool> {
    numbers
        .into_iter()
        .map(i32::is_positive)
        .collect()
}

fn main() {
    let numbers = [-2, -1, 0, 1, 2, 3];
    let numbers_positive = are_positive(&numbers);
    println!("{numbers_positive:?}");
}
```

Some other consuming methods than collect are for example the for_each that works like for loop, nth to get the nth value, and count that computes the amount of items in the iterator — we used nth previously to get the nth character in a string (in the "Indexing strings?" info snippet in the slices section). Rust also provides specialized consuming methods for numerical iterators, such as sum and product.

The following example shows how we get an error if we try to use an iterator after it has been consumed with count. To fix the example, we can use iter() again to create a new iterator when one is needed.

```rust
#![allow(unused_mut)]
fn miles_to_kilometers(miles: &f64) -> f64 {
  miles * 1.609344
}
fn main() {
    let miles = [1.0f64, 3.0, 10.0, 100.0];
    let mut miles_iter = miles.iter();
    let number_of_distances = miles_iter.count();
    let total_kilometers = miles_iter
        .map(miles_to_kilometers) // whoops?
        .sum::<f64>();
    println!("{total_kilometers} kilometers in {number_of_distances} trips");
}
```

The nth method does not consume the iterator fully, but only until the nth value. This means that we can use the iterator again after calling nth. We need to be careful though, as the iterator will continue from where we left off.

```rust
fn main() {
    let philosopher = "Ἀριστοτέλης";
    let mut chars = philosopher.chars(); // chars() returns an iterator over the characters in a string
    let third = chars
        .nth(2)
        .unwrap();
    let number_of_chars = chars.count(); // whoops, counts only the remaining chars
    println!("The length of {philosopher} is {}", philosopher.len()); // number of bytes
    println!("The number of chars in {philosopher} is {number_of_chars}"); // number of chars
    println!("The third character of {philosopher} is {third}.");

}
```

### Iterating one at a time

All iterators have a method next which may or may not give back a value. Let's create an iterator to inspect an array using the next method.

```rust
fn main() {
    let arr = [1, 2];
    let mut arr_iter = arr.iter();
    let one = arr_iter.next();
    let two = arr_iter.next();
    let three = arr_iter.next();
    println!("{one:?}, {two:?}, {three:?}");
}
```

The next method yields values of type Option. After all, there might not be a next value in the iterator. We can see this when calling next() thrice for an array of size 2 in the above example.

Notice that we defined the arr_iter variable as mutable. This is required even though the array itself doesn't change — the iterator does change. The iterator "keeps getting smaller" each time we take the next value from it. The next method immediately consumes a value from the iterator, meaning that is is not lazy. The consuming iterator methods like collect and nth work non-lazily, i.e. eagerly, by calling next repeatedly.


## Closures

Closures can be thought of as anonymous functions with concise syntax. A closure is an expression which we can call like regular functions. To create a closure, we write parameters inside pipes | and follow them with the body of the closure.

```rust
#![allow(unused)]
fn main() {
    let add = |a, b| a + b;
    println!("1 + 1 = {}", add(1, 1));
}
```

The key difference to functions is that closures have access to variables in the scope they are defined in. In other words, closure capture their enclosing scope, hence closure. This gives closures a neat advantage over ordinary functions. Notice how in the previous example we didn't have to specify the types of the parameters a and b or the return type of the closure. The compiler is able infer the types from the context and the body of the closure so we don't have to specify them explicitly.

```rust
fn  add   (a: i32, b: i32) -> i32 { a + b } // Obligatory type annotations and braces
let add = |a: i32, b: i32| -> i32 { a + b }; // Optional type annotations
let add = |a, b|                  { a + b }; // Optional braces
let add = |a, b|                    a + b  ;
```

Capturing the enclosing scope also means that we can use variables from the enclosing scope in the body of the closure.

```rust
fn main() {
    let base = "main";
    let with_file_extension = |extension| {
        let mut s = base.to_string();
        s.push_str(extension);
        s
    };
    println!("{}", with_file_extension(".rs"));
    println!("{}", with_file_extension(".py"));
}
```

### Closures and ownership

When capturing variables in closures, we need take care to adhere to Rust's ownership rules like we do when defining and using variables.

A closure can capture variables in three ways: immutable borrow, mutable borrow, or by move (take ownership). The compiler infers which one to use based on the closure body. Imagine a function that decides automatically whether it should take a mutable or immutable reference to a variable based on the body of the function. That's a closure for you. It won't solve all problem's though.

In the example below, we double the value of variable i from the enclosing scope in the body of the closure. It doesn't compile however. Check the compiler error message to see an error message about immutable and mutable borrows that is familiar to what we saw when first looking into ownership and borrowing.

```rust
fn main() {
    let mut i = 2;
    let mut double_i = || i = i * 2; // capture i by borrowing mutably
    double_i(); // use mutable borrow
    println!("{i}"); // borrow immutably
    double_i(); // use mutable borrow again (not allowed)
}
```

As a comparison, what we did here with the closure is equivalent to the following with just variables.

```rust
fn main() {
    let mut i = 2;
    let j = &mut i; // borrow i mutably
    *j = *j * 2; // use mutable borrow
    println!("{i}"); // borrow i immutably
    *j = *j * 2; // use mutable borrow again (not allowed)
}
```

Note also that we needed the mut keyword for the closure to allow the closure to modify the variable i from the enclosing scope. Like variables, closures are immutable by default and won't take mutable references implicitly. We can also force a closure to take ownership instead of borrowing with the move keyword. It is mostly used in parallel computing with multiple execution threads so we won't be needing move for this course.

If the closure ownership handling feels like a lot to handle, no worries! We'll mainly be using closures without capturing variables from the enclosing scope, so there goes that problem

### Iterator methods and closures

Previously, we looked at an example, where degrees Celsius were converted into degrees Fahrenheit using for_each. In the example, we had to use a new version of the function that modified the floating point numbers instead of returning the result of the computation. With closures, we can still use the old non-mutating celsius_to_fahrenheit in the call to for_each to mutate the values in a collection.

```rust
fn main() {
    fn celsius_to_fahrenheit(f: f64) -> f64 {
        f * 9.0 / 5.0 + 32.0
    }

    let mut temps = [1.0f64, 3.0, 10.0, 100.0];
    temps
        .iter_mut()
        .for_each(|temp| *temp = celsius_to_fahrenheit(*temp));

    println!("{temps:?}");
}
```

We haven't used the for_each method before for printing, but always resorted to the for loop. This is because the println! macro is not a function, so we cannot pass it as an argument to for_each. However, we can easily wrap the macro in a closure and pass it to for_each.

```rust
fn main() {
    let temps = [1.0f64, 3.0, 10.0, 100.0];
    temps
        .iter()
        .for_each(|temp| println!("{temp}℃"));
}
```

Likewise, we can use the map method together with a closure without having to specify extra functions with parameter types and all.

```rust
fn main() {
    let miles = [1.0f64, 3.0, 10.0, 100.0];
    let kilometers = miles.iter().map(|m| m * 1.609).collect::<Vec<_>>();
    miles
        .iter()
        .zip(kilometers.iter())
        .for_each(|(m, k)| println!("{m} miles equals {k} kilometers"));
}
```

Here, we also used the zip method to iterate over two collections at the same time. The zip adapter method takes in another iterator and returns a new iterator that yields tuples of the values from the two iterators

### Filtering and finding

Yet another useful iterator adaptor, filter, can be used effectively in conjunction with closures. The filter method takes in a function that returns true if the value should remain and false if it should be skipped.

Let's filter an array using a simple comparison predicate.

```rust
fn main() {
    let temps = [1.0f64, 3.0, 10.0, 100.0];
    let low_temps = temps.iter().filter(|temp| **temp < 15.0);
    low_temps.for_each(|low_temp| {
        println!("It's {low_temp}°C. So cold!");
    });
}
```

In the example above, the closure takes a double reference: &&f64 so we need to dereference it twice with ** to get the actual value. This is because, unlike the map adaptor, the filter adaptor does not need ownership of the values it is filtering, in this case the values have type &f64. If filter took ownership of the values, it would need to return them back too...

Let's filter a map of country-population pairs next.

```rust
use std::collections::HashMap;
fn main() {
    let country_populations = HashMap::from([
        ("Finland", 5_500_000),
        ("Estonia", 1_300_000),
        ("Sweden", 10_200_000),
        ("Norway", 5_300_000),
        ("Denmark", 5_800_000),
        ("Iceland", 400_000),
    ]);

    let over_5mil = country_populations.iter()
        .filter(|(_, population)| **population >= 5_000_000);
    for (country, population) in over_5mil {
        println!("{country} has population {population}");
    }
}
```


Because the iterator yields tuples, we can conveniently destructure them inside the pipes without needing to specify the types of the components. The underscore variable _ is used to ignore the first unused component of the tuple. The compiler warns about unused variables by default, so we need to explicitly tell it to ignore the variable with the underscore when we don't want to see such warnings.

To find a value which matches a given predicate, we can use the find method on an iterator.

```rust
fn main() {
    let sentence = "This is how we split strings by whitespace";
    let mut words = sentence.split_whitespace();

    let three_letters = words.find(|word| word.len() == 3);
    print!("{three_letters:?}"); // Some("how")

    words.for_each(|word| print!(" {word}"));
    println!("!");
}
```

The find method returns the first match, but the rest of the iterator is left intact like with the nth method.

Remember that we could not remove from a vector directly by value but only by index? Well, actually we can, but we need a function or a closure. Vec has a retain method, which is very similar to the filter method. It retains, i.e. keeps, all the values from the vector that match a given predicate. If we want to remove only a single value, we can use Iterator's position method to get the index of the first matching value. Then remove with Vec's remove based on the index.

```rust
fn main() {
    let mut temps = vec![1.0f64, 3.0, 10.0, 100.0];
    temps.retain(|temp| *temp < 15.0);
    println!("{temps:?}"); // [1.0, 3.0, 10.0]

    if let Some(index) = temps
        .iter()
        .position(|temp| (*temp - 3.0).abs() < f64::EPSILON)
    {
        temps.remove(index);
    }
    println!("{temps:?}"); // [1.0, 10.0]
}
```

---

> Common iterators

The Rust standard library provides us with plenty of other iterator methods, 
for example reduce, fold, take_while, flatten, max_by_key. Here are some examples of how to use them.

```rust
fn main() {
    let a = [1, 5, 3, 2];

    // Compute max value using reduce
    let max = a.iter().reduce(|a, b| if a >= b { a } else { b });
    assert_eq!(max, Some(&5));

    // Compute product using fold
    let product = a.iter().fold(1, |accumulator, a| accumulator * a);
    assert_eq!(product, 1 * 5 * 3 * 2);

    // Take all values until the first non-increasing value using take_while
    let mut prev = i32::MIN;
    let increasing = a
        .iter()
        .take_while(|next| {
            if **next > prev {
                prev = **next;
                true // keep taking
            } else {
                false // stop taking
            }
        })
        .collect::<Vec<&i32>>();
    assert_eq!(&increasing, &[&1, &5]);

    // Flatten a vector of tuples using flatten
    let flat = a.iter().map(|num| [*num, 0])
        .flatten().collect::<Vec<i32>>();
    assert_eq!(flat, &[1, 0, 5, 0, 3, 0, 2, 0]);

    // Find the longest word using max_by_key
    let numbers = [(1, "one"), (2, "two"), (3, "three"), (4, "four")];
    let longest_number = numbers.iter().max_by_key(|(_num, word)| word.len());
    assert_eq!(longest_number, Some(&(3, "three")));
}
```

### Enumerating iterators

Sometimes we would like to access an element's index in addition to the element itself. Calling enumerate on an iterator returns a new iterator which adds an incrementing counter to each item in the form of a pair.

```rust
fn main() {
    let a = ["a", "b", "c"];
    let mut e = a.iter().enumerate();
    println!("{:?}", e.next()); // Some((0, "a"))
    println!("{:?}", e.next()); // Some((1, "1"))
    println!("{:?}", e.next()); // Some((2, "c"))
    println!("{:?}", e.next()); // None
}
```

Next, we'll use a for loop to iterate over the enumerated iterator and print the indices and values stored in each pair.

```rust
fn main() {
    let a = ["a", "b", "c"];
    for (i, item) in a.iter().enumerate() {
        println!("{i} {item}");
    }
}
```

For a slightly more practical example let's use enumeration to split a vector into two vectors pushing every other element to the same vector.

```rust
fn split_adjacent(items: &Vec<&str>) -> (Vec<String>, Vec<String>) {
    let mut evens = vec![];
    let mut odds = vec![];
    for (i, item) in items.iter().enumerate() {
        if i % 2 == 0 {
            evens.push(item.to_string());
        } else {
            odds.push(item.to_string());
        }
    }
    (evens, odds)
}

fn main() {
    let letters = vec!["a", "b", "c", "d", "e", "f", "g"];
    let (evens, odds) = split_adjacent(&letters);
    println!("{evens:?}");
    println!("{odds:?}");
}
```

Here in split_adjacent function, instead of returning back references, we push new Strings created from the given string slice references to the corresponding vectors. If we wanted to directly push the &str references to the vectors (returning a pair of Vec<&str>), we would have to deal with the advanced concept of generic lifetimes. We will cover that option later in the course.

When collecting the results of enumerated string references, we cannot use the cloned method to get owned copies from the references. This is because cloned works only on iterators that yield references but the iterator yields tuples (usize, &String), which are not references but owned values (only the second value of the tuple is a reference). Instead, we can clone the &Strings inside the tuple with the map method.

```rust
fn main() {
    let strings = ["a".to_string(), "b".to_string(), "c".to_string()];
    let enumerated: Vec<(usize, String)> = strings
        .iter()
        .enumerate()
        .cloned()
        .collect();
    println!("{enumerated:?}");
}
```

In this example, we could also opt to clone the string references using cloned prior to enumerating with enumerate, or use into_iter to iterate over owned values instead of references. However, such options are not always available, e.g. when working with collections or iterators as function parameters.

---



















