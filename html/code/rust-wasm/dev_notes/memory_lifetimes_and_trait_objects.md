<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# Memory, Lifetimes and Trait objects

Learning objectives

- You know the basics of memory management in Rust
- You know what unsafe Rust is and in what situations it may needed
- You know how Rust lifetimes work and how and when to use lifetime annotations
- You know how to use trait objects and know the difference between dynamic and static dispatch

## Stack, Heap and Ownership

Memory management can be split into the following categories correlating with the lifecycle of information:

- Creation. How is memory allocated in order to store the information?
- Processing. How is information accessed, modified and moved in memory?
- Destruction. How is the previously allocated memory freed for other use after the information is no longer needed?

Modern programming languages mostly have built-in automatic memory management (e.g. Python and JavaScript), whereas in classic programming languages, such as C, memory has to be mostly managed manually with code.

In Rust, memory is not managed automatically by a garbage collector — but neither is it the programmer's responsibility to manage memory manually (unlike in e.g. C or C++ where memory is allocated and deallocated in code with functions such as malloc and free). In fact, memory cannot be managed manually in terms of allocations and deallocations in safe Rust where the Rust borrow checker care of allocations and deallocations. Sometimes the ownership rules are too strict however, which is why there is also unsafe Rust, a sort of a sublanguage of Rust that is free of the limits of the borrow checker and ownership system. 

To understand memory management in Rust, it is important to understand what stack and heap are, two essential regions of memory available for a program at runtime.

In short, the stack is a region of memory which is used to store information about the active functions and local variables in a program. It is fast because it is fixed size and modified the same way as a data structure, last in first out (LIFO); when a function is called, memory required for the function is reserved from the top of the stack and the memory is freed when the function returns. This makes deallocation and allocation possible by simple moving the stack pointer up and down. It also means that the size of the stack is limited, a stack overflow happens when the stack does not have room to accommodate memory assigned to it.

The heap is a region of memory that is used to store information dynamically at runtime. The heap has less limitations compared to stack as memory can be allocated at any unreserved portion at any time in the heap but this also makes the heap more expensive to use as more complex management is required to ensure safe allocations.

In case you haven't yet read the Stack and Heap section from the Rust book, we highly recommend doing so now.

The following table summarizes the practical differences between the stack and the heap in a rule of thumb fashion.

Stack	                    Heap
----------------            ------------------
Very fast access	        Slower access
Fixed size	                Virtually unlimited size
Efficient space management	Space can become fragmented


We'll next look at some code examples that highlight how the stack and heap are used in Rust and how they relate to ownership


### Stack and pointers

All functions and their related information such as local variables (variables inside functions) in Rust are stored on the stack. When calling a function, the arguments and local variables get pushed onto the stack, among other things.

```rust
#![allow(unused)]
fn main() {
    let x: i32 = 5;
}
```

In this simplified example, when the function main is called, the variable x is allocated on the stack with 32 bits of memory. Once the program returns from the main function call, all local variables are deallocated, which invalidates the memory address of them including x.

A pointer is a number which stores a memory address. It's important to note that references in Rust are not just pointers, as we will see later.

The memory address of x can be printed by formatting a reference to x using the pointer-format parameter :p.

```rust
fn main() {
    let x = 5;
    println!("Memory address of x {:p}", &x);
}
```

The memory address which is printed out as a hexadecimal (indicated by the prefix 0x) number lies on the stack.

Memory addresses to the stack rarely stay the same when there are multiple functions being called.


```rust
fn plate1() {
    let x = 5;
    println!("x on plate1 {:p}", &x);
}

fn plate2() {
    let y = 5;
    println!("y on plate2 {:p}", &y);
    plate1();
}

fn main() {
    plate1();
    plate2();
}
```

Pushing plate1 onto the call stack on top of main leads to the local variable x being stored in a different location as opposed to when plate1 is pushed on top of plate2.

The location of x when plate1 is called from plate2 is smaller, because the stack grows backwards in regard to memory addresses.

The pointers which are printed always have a fixed size based on the architecture of the processor running the program. For 64-bit processors, a memory address is always represented by a 64-bit number. To account for different CPU architectures, we use the usize integer type, which always matches the pointer size of the target architecture.

The usize numbers are used when indexing, setting the length or getting the length of arrays and vectors.

---

null pointer

A pointer, being just a number, is usually left as the value 0 in order to indicate that it points to nothing. This is known as the null pointer. The null pointer was invented by a famous computer scientist Tony Hoare, who now calls his invention a "billion-dollar mistake".

Using, i.e. dereferencing, a null pointer without checking whether it is null first will always cause problems. Nowadays, dereferencing a null pointer leads almost certainly to a crash at runtime known as a segmentation fault.

In Rust, references can never point to nothing, or contain the address 0. If we wanted to have optional references, we would put the reference inside an Option.

---

Values than can be copied (types that implement the Copy trait), like i32 and usize or tuples and arrays that only contain copiable values, are also stored on the stack. This is the reason why Rust implicitly copies such values; it is cheap to allocate them on the stack. Because of the implicit copying, we rarely encounter issues with ownership regarding stack stored values (excluding issues related to references of course).

Values that can't be copied, like String which has a dynamic size and is therefore a bad option for storage in the stack, are stored on the heap.

Another bad option for storage in the stack is large collections. The stack has a limited size, which is usually a few megabytes. If we try to allocate a large array on the stack, we will get a stack overflow error at runtime.

```rust
#![allow(unused)]
fn main() {
    let big_array = [0; 10_000_000];
}
```
To overcome such an annoyance, we can force the compiler to allocate the big array on the heap instead of the stack by using for instance a Vec to allocate the values on the heap instead of on the stack.

### Heap and ownership

The heap is an area of memory used to store data more dynamically than on the stack. It can also be used to store large collections of values more safely than stack. In programming languages with a garbage collector, the heap is managed automatically. In Rust, most heap memory is managed automatically by the compiler according to the rules of ownership.

> Reminder: rules of ownership and references ⏰

- Each value in Rust is owned by a variable.
- Each value can have only one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- At any given time, you can have either one mutable reference &mut or any number of immutable references &.
- References must always point to valid instances of types.

Many data types, like Vec, String and HashMap use memory on the heap to store their contained values. When adding elements to a Vec, a continuous chunk of memory is automatically allocated on the heap for storing the values. The address to the memory can be accessed using the as_ptr method.


```rust
fn main() {
    let mut v: Vec<usize> = Vec::new();
    println!("Address of Vec variable {:p}", &v);
    println!("Address to Vec values in heap {:p}", v.as_ptr()); // pointer to unallocated memory
    println!("Vec capacity {}", v.capacity()); // amount of memory allocated for the vector
    for i in 0..1_000 {
        let capacity = v.capacity();
        v.push(i);
        if v.capacity() != capacity {
            println!("Vec capacity increased to {}", v.capacity());
        }
    }
    println!("Address to Vec values in heap {:p}", v.as_ptr()); // pointer to allocated memory
}
```

In the example, we create an empty Vec which initially contains a pointer to unallocated memory. Writing to unallocated memory would be a memory bug, so the vector has to make a heap allocation upon any push that would make the length of the vector greater than its capacity, starting from the first push. Try changing Vec::new() to Vec::with_capacity(1_000) to do the heap allocation beforehand.

The heap allocated memory is not moved when the Vec is moved. It is the Vec struct itself that contains a pointer to the heap is on the stack that gets its ownership moved to the other variable.

```rust
fn use_vec(v: Vec<i32>) {
    println!("Address of v {:p}", &v);
    println!("Address to values in heap {:p}", v.as_ptr());
}

fn main() {
    let v: Vec<i32> = (1..1_000).collect();
    println!("Address of v {:p}", &v);
    println!("Address to values in heap {:p}", v.as_ptr());
    use_vec(v);
}
```

Notice that the address in the heap is the same in both cases. The memory in the heap is not affected when the Vec is moved. It is the Vec struct itself that contains a pointer to the heap that gets its ownership moved to the other variable, and the struct itself is stored on the stack. Moving a Vec is fast as the elements remain as they are in the heap, only the pointer needs to be copied to the stack for the function call.

Once the Vec's owner goes out of scope, the heap memory gets freed by the borrow checker as the value is dropped.

---

as_ref borrowing option

Suppose we have a reference to an optional vector. How can we access elements in the vector? Consider the following counterexample.


```rust
#![allow(unused)]
fn unwrap_ref(opt: &Option<String>) -> &String {
  &opt.unwrap()
}

fn main() {
  let opt = Some("string".to_string());
  let string = unwrap_ref(&opt);
}
```

The example doesn't compile, because we cannot call unwrap on a &Option<T>. The unwrap method has the following signature.

```rust
pub fn unwrap(self) -> T
```

Calling opt.unwrap() tries to dereference the opt first in order to get an owned self. However, as Option<String> is not Copy, dereferencing would try to move the value out of a shared reference, which is illegal.

The trick here is to convert the &Option<String> into a Option<&String>. The latter type is Copy and unwrapping it consumes the option returning the reference.

Let's create this function and call it as_ref. We will use a new keyword ref, which is used to borrow during destructuring or pattern matching.

```rust
#![allow(unused)]
fn as_ref<T>(opt: &Option<T>) -> Option<&T> {
  match *opt {
      Some(ref t) => Some(t),
      None => None,
  }
}

fn unwrap_ref(opt: &Option<String>) -> &String {
  as_ref(opt).unwrap()
}
fn main() {
  let opt = Some("string".to_string());
  let string = unwrap_ref(&opt);
}
```

Matching does some magic in the background, where *opt actually doesn't get evaluated immediately on the line match *opt. Try binding *opt to a variable first to see what happens.

In the Some(ref t) pattern, we match the Some variant, but instead of moving the contents (t) out of the option, we just borrow them using the ref pattern.

In a match arm, the & pattern is the opposite of the ref pattern, as & dereferences the content, but ref borrows it.

Option's already have a method for doing this reference transformation — the confusingly named as_ref method.

---

### Cloning from (and to) the heap

Most types in Rust implement the Clone trait, which is usually used to create another "identical" copy of a value. For Copy types, this means a bitwise copy in the stack, but for non-Copy types the clone implementation can be really anything.

For instance, the Vec type has a clone method which creates a new Vec with the same contents as the original by allocating new memory in the heap and storing copies of the values (using .clone() for the inner values) of the original vector.

To demonstrate a potential downside of this, let's see what happens to a Vec of references when cloned.

```rust
fn main() {
    let v = vec!["a", "b", "c"];
    println!("Heap address {:p}", v.as_ptr());
    println!(" a's address {:p}", v[0]);

    let w = v.clone();
    println!("Heap address {:p}", w.as_ptr());
    println!(" a's address {:p}", w[0]);
}
```
The address in the heap for the stored &strs changes, because during cloning, another chunk of heap memory is allocated for the clone. However the static strings are still at the same address because only the & references got copied in the heap, not the actual values behind the references.

Cloning data to avoid ownership issues is unadvised, as it can potentially lead to bugs, and it usually incurs efficiency penalties. Using references and working with lifetimes is usually the better way to go.

## Breaking the rules with unsafe

All types are not Copy because some types can't be safely copied. A common reason is that the type contains a pointer to some data on the heap.

Consider the next example where we try to copy a vector v into variable w and then use both vectors.

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut w = v;                                 // create a "copy" of v
    println!("{:p} {:p}", v.as_ptr(), w.as_ptr()); // both point to the same address in the heap
    w.clear();                                     // sets the length of the vector to 0
    w.push(5);                                     // [5]
    println!("{:?}", v);                           // [5, 2, 3]
}
```
Copying a Vec struct, which is on the stack, would result in two Vecs which are both responsible for freeing the same memory in the heap. This is not allowed by the ownership rule, which states that each value can have only one owner at a time.

Rust has a feature, known as unsafe Rust, which allows programmers to get around some restrictions of safe Rust and work instead with raw pointers among other things. To write unsafe Rust, we must use the keyword unsafe before a code block or a function.

It is important to mention that despite its name, unsafe Rust does not mean it is unsafe to use or run, rather that it is not necessarily safe as per the Rust's ownership system. It is a way of saying to the compiler: "I have checked the correctness, including memory safety, of the following code myself."

In the next example, we use an unsafe function Vec::from_raw_parts incorrectly.


```rust
fn main() {
      let mut v = vec![1, 2, 3];
      // create a copy of v
      let mut w = unsafe { Vec::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
      println!("{:p} {:p}", v.as_ptr(), w.as_ptr()); // both point to the same address in the heap
      println!("{v:?}");                             // [1, 2, 3]
      w.clear();                                     // sets the length of the vector to 0
      w.push(5);                                     // [5]
      println!("{v:?}");                             // [5, 2, 3]
} // segmentation fault
```

This code is extremely volatile and causes undefined behavior, because both vectors free the same allocated memory. This memory safety bug is known as a double free.

Some common and good reasons to use unsafe Rust are:

    Interacting with C code, which is unsafe because C is unsafe.
    Safe Rust alternative is too complicated or not performant enough
    Writing low-level code where usage of raw pointers is required

Going into detail in any of these topics is out of the scope of this course, but you can read more about them and unsafe Rust altogether in the Rustonomicon.

## Lifetimes

Variables (which are always on the stack) constantly go out of scope and the memory used by them is constantly shuffled around during execution. How can Rust possibly keep track and ensure the validity of all references? With lifetimes!

Lifetimes define the sections of code where some piece of data, or a reference to it, is still valid. If this rings a bell, that's probably because the lifetime of a reference is synonymous to the scope of a reference, which we covered back in the part 4 of the course.

The compiler is always in charge of lifetimes, the programmer can only communicate relationships between lifetimes. The compiler can't always make the right decisions about relationships between some lifetimes, so we, the programmer, need to occasionally step in and provide lifetime annotations for the compiler.

Lifetime annotations are defined with a tick ', followed by the name of the lifetime, e.g. 'a. Consider the following simple example.

```rust
fn main() {
    let s = "☃️";      // ══════════════╗
    let r = &s;        // ═════════╗'a  ║
    println!("{r}");   // ═════════╝       ║
    println!("{s}");   //               ║
} // ═══════════════════════════════════╝

```
The code compiles fine, because the variable is valid until its value is dropped or moved, which in this case happens at the end of the main function, and that encompasses the whole lifetime of the reference r, denoted by 'a.

Now, if we make s a String and move it before the lifetime of the reference r ends, we get a compiler error.

```rust
fn take_string(s: String) {
    println!("{s}");
}

fn main() {
    let s = "☃️".to_string();    // ══════════════╗
    let r = &s;                  // ═════════╗'a  ║
    take_string(s);              // ═════════║════╝
    println!("{r}");             // ═════════╝
}
```

### Functions and lifetimes

What if we want to return a reference from a function? If the value we want to return is a reference to a value that is owned by the function, we have but one option: to annotate our return value reference type with a special lifetime specifier 'static.


```rust
fn str_from_function() -> &str {
    "😪"
}

fn main() {
    let str = str_from_function();
    println!("{str}");
}
```

The 'static lifetime specifier can fix many lifetime issues but often it is not the best choice. A 'static lifetimes means that the reference can be valid for the entire duration of the program. Even when the compiler suggests adding a 'static lifetime, it is best to think if that is really necessary and what you want for the reference.

Then, what about the case where we want to return a value from a function that is constructed from a reference parameter?

```rust
fn last_chars(s: &str, n: usize) -> &str {
    &s[n..]
}

fn main() {
    let s = String::from("Hello");
    println!("{}", last_chars(&s, 3));
}
```

Well, it turns out that our example works just fine. The compiler is smart enough to figure out that the returned reference should be valid for the lifetime of the parameter s.

However, if we go ahead and make the return value of last_chars 'static, we'll quickly run into problems.

```rust
fn last_chars(s: &'static str, n: usize) -> &'static str {
    &s[n..]
}

fn main() {
    let s = String::from("Hello");
    println!("{}", last_chars(&s, 3));
}
```

Note that the output reference of type &'static str depends on the input reference s. This means that as the output can live for the whole duration of the program, so needs the input, and the input must be 'static too. Then we can no longer take in reference from local variables, because they won't live long enough.

Then, we get into a whole different kind of a problem if we have multiple references that the return value might depend on.

```rust
fn longer(s: &str, t: &str) -> Option<&str> {
    if s.len() == t.len() {
      return None;
    }

    if s.len() > t.len() {
        Some(s)
    } else {
        Some(t)
    }
}

fn main() {
    let s = String::from("Hello");
    let t = String::from("World!");
    println!("{:?}", longer(&s, &t));
}
```

The compiler complains that it can't figure out what the lifetimes of function parameters and returned reference should be. This is because the return value could be from either s or t. We can fix this by following the compiler advice to add a generic lifetime parameter 'a and specify it for each of the references. The fix informs the compiler that the returned reference should be valid for the lifetimes of both of the parameters s and t. In practice, this causes the return value's lifetime to be the same as the smaller lifetime of the two parameters.

To elaborate the problem in the above example further, let's see what happens when we give the two parameters distinct lifetime parameters and use only one of them for the return type.

```rust
fn longer<'a, 'b>(s: &'a str, t: &'b str) -> Option<&'a str> {
    if s.len() == t.len() {
      return None;
    }

    if s.len() > t.len() {
        Some(s)
    } else {
        Some(t)
    }
}

fn main() {
    let s = String::from("Hello");
    let t = String::from("World!");
    println!("{:?}", longer(&s, &t));
}
```

This doesn't compile because the longer returns a reference with a lifetime of at least 'a but the function may return a reference with lifetime of 'b, and the compiler cannot know for certain that 'b will be outlive 'a. Thus the compiler suggests adding a lifetime bound of 'a to 'b using the syntax: 'b: a' (even though we could just use 'a for both of the parameters). Lifetime bounds are similar to trait bounds (T: U), trait bounds require a type that implement a trait T to implement the bound trait : U and lifetime bounds require a lifetime 'a to be at last at least as long as the bound lifetime (: b).

Lifetimes can be difficult to get right. While most likely we can get by with just a single lifetime specifier, it is a good idea to not constrain ourselves unnecessarily with lifetimes.

Consider the following example, which **does not work**.

```rust
fn extract_suffixed<'a>(a: &[&'a str], b: &'a str) -> Vec<&'a str> {
    a.iter()
        .filter(|s| s.ends_with(b))
        .copied()
        .collect::<Vec<_>>()
}

fn main() {
    let mut suffix = "fi/".to_string();
    let slice = ["aalto.fi", "helsinki.fi", "rust-lang.org"];
    let suffixed1 = extract_suffixed(&slice, &suffix); // immutable borrow of suffix with lifetime 'a
    suffix.remove(suffix.len() - 1); // mutable borrow of suffix while lifetime 'a is still active
    let suffixed2 = extract_suffixed(&slice, &suffix);
    println!("{suffixed1:?}"); // end of lifetime 'a
    println!("{suffixed2:?}");
}
```

The reason this does not work is that the function extract_suffixed tells that both the suffix parameter and the &strs inside the slice parameters should have the same lifetimes. So in the main function, the lifetime of the reference suffix: &str that is passed to the function needs to live as long as the values inside slice, which is also passed to the function. Yet the suffix.remove(suffix.len() - 1) method call takes a mutable reference to suffix, which is not ok for the borrow checker: the immutable borrow of suffix for the previous function call is still alive.

The solution is to remove the constraint 'a from the function parameter b to relax the lifetime constraint. We can give it for instance another lifetime 'b, or even better, we could simply elide, i.e. omit, the lifetime parameter from b altogether. The parameter b is not referenced in the returned value, so the compiler can determine its lifetime without the need explicit annotations.

```rust
fn extract_suffixed<'a>(a: &[&'a str], b: &str) -> Vec<&'a str> {
    a.iter()
        .filter(|s| s.ends_with(b))
        .copied()
        .collect::<Vec<_>>()
}

fn main() {
    let mut suffix = "fi/".to_string();
    let slice = ["aalto.fi", "helsinki.fi", "rust-lang.org"];
    let suffixed1 = extract_suffixed(&slice, &suffix);
    suffix.remove(suffix.len() - 1);
    let suffixed2 = extract_suffixed(&slice, &suffix);
    println!("{suffixed1:?}");
    println!("{suffixed2:?}");
}
```

### Lifetime elision rules

So, when exactly can lifetimes be elided? Like ownership, lifetime elision has three rules.

1. Each parameter that is a reference gets its own lifetime parameter: fn f(x: &str, y: &str) is the same as fn f<'a, 'b>(x: &'a str, y: &'b str).
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn f(x: &str) -> &str is the same as fn f<'a>(x: &'a str) -> &'a str.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters: fn f(&self, x: &str) -> &str is the same as fn f<'a>(&'a self, x: &str) -> &'a str.

To provide a simple rule of thumb, (in most cases) lifetimes can be elided from functions according to the following two rules:

1. The function takes in at most one reference, or
2. The function returns nothing that contain references

Note that it is also possible for the compiler to be able to infer the lifetimes of references by the lifetime elision rules in a way that is semantically incorrect, i.e. the code may compile but has a bug related to lifetimes. See for instance the nice example in this blog post about common lifetime misconceptions (the whole blog may be an insightful read).

As a summary, lifetimes are generics like generic types and lifetime annotation parameters are defined within the familiar angle brackets <>. However, unlike with generic type parameters, we cannot pass "concrete" lifetimes to functions since the concrete lifetimes are always the responsibility of the compiler.

One important thing to note is that when lifetime parameters are used alongside generic type parameters, they go inside the same <> brackets as the generic type variables.

```rust
fn get_biggest<'a, T>(a: &'a T, b: &'a T) -> &'a T
where
    T: PartialOrd + ?Sized + 'a,
{
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let mut a = "1".to_string();
    println!("{}", get_biggest(a.as_str(), &"2"));
    a = "3".to_string();
    println!("{}", get_biggest(a.as_str(), &"2"));
}
```
Also lifetime bounds can be added to trait bounds, a lifetime bound for a type, e.g. T: 'a, means that all references in T must outlive lifetime 'a

---

The placeholder lifetime `'_`

Like with generic types, we can use the _ in place of a lifetime parameter name to let the compiler figure out the lifetime whenever this is possible (although in the case of lifetimes we can omit the use of a lifetime parameter altogether).

Such inference is not possible in the case of choosing from two parameter lifetimes, but consider the Display trait that, which we saw when looking at traits.


```rust
use std::fmt::{Formatter, Error};

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The Formatter struct requires a lifetime annotation for a reference in one of its (private) fields, but instead of creating a named lifetime parameter or eliding it, the code for Display uses the '_ placeholder lifetime to make it clear that Formatter has a field that is a reference and needs to abide by the lifetime rules without the additional syntax of the generic lifetime parameter.

Similarly, we can use lifetime placeholders in function definitions or any other place that can have elidable generic lifetime parameters if we want to make lifetimes explicit in code.

```rust
fn last_chars(s: &'_ str, n: usize) -> &str {
    &s[n..]
}

fn main() {
    let s = String::from("Hello");
    println!("{}", last_chars(&s, 3));
}
```

---

### Lifetimes everywhere

As we saw with functions, lifetimes are most of the time elided, i.e. left out of the code. This makes writing Rust much more concise and, to an extent, easier. But, even though we can't see the lifetimes they are still there and at times we need to think about them. Here are some places where a lifetime parameter may be lurking.

- a struct method
- a function which takes references
- a function which returns references
- a trait method
- a closure

As an example, let's look at a struct, FullName, which contains references to two strings slices.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy)]
struct FullName {
    first_name: &str,
    last_name: &str,
}
fn main() {}
```

Compiling this code fails with missing lifetime specifier.

We therefore need to use generic or 'static lifetime specifiers. Restricting the string to a static lifetime would most likely be too restrictive, so let's add a lifetime parameter 'source for the struct.


```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy)]
struct FullName<'source> {
    first_name: &'source str,
    last_name: &'source str,
}

fn main() {
    let first = "String".to_string(); // ══════════════╗'first
    let last = "Slice".to_string();   // ════════╗'last║
                                      //         ║         ║
    let name = FullName {             // ══╗'name║         ║
        first_name: &first,           //   ║         ║         ║
        last_name: &last,             //   ║         ║         ║
    };                                //   ║         ║         ║
    println!("{name:?}");             //   ║         ║         ║
} // name, last, first are dropped ════════╩═════╩═════╝
```

In the example, when creating a FullName, the compiler infers the concrete lifetime for 'source, which in this case represents the lifetime 'last — the smaller of the two lifetimes 'first and 'last.

When implementing associated functions for a struct with references, we need to annotate the lifetime of the struct for the implementation block, at least with a placeholder.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy)]
struct FullName<'source> {
    first_name: &'source str,
    last_name: &'source str,
}

impl FullName<'_> {
    fn new<'a>(first_name: &'a str, last_name: &'a str) -> FullName<'a> {
        FullName {
            first_name,
            last_name,
        }
    }
}

fn main() {
    let first = "String".to_string();        // ══════════════╗'first
    let last = "Slice".to_string();          // ════════╗'last║
                                             //         ║         ║
    let name = FullName::new(&first, &last); // ══╗'name║         ║
    println!("{name:?}");                    //   ║         ║         ║
} // name, last, first are dropped   ═════════════╩═════╩═════╝
```

We would probably be better off using the struct's lifetime specifier than the placeholder lifetime _ though, because then we can return Self and also have parameter lifetimes bound to the structs lifetime easily.


```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy)]
struct FullName<'source> {
    first_name: &'source str,
    last_name: &'source str,
}

impl<'source> FullName<'source> {
    fn new(first: &'source str, last: &'source str) -> Self {
        FullName {
            first_name: first,
            last_name: last,
        }
    }
    fn rename_first<'a: 'source>(&mut self, first: &'a str) {
        self.first_name = first;
    }

    fn rename_last<'a: 'source>(&mut self, last: &'a str) {
        self.last_name = last;
    }
}

fn main() {
    let first = "String".to_string();            // ═════════════════╗'first
    let last = "Slice".to_string();              // ═══════════╗'last║
    let mut name = FullName::new(&first, &last); // ═════╗'name║         ║
    println!("name: {name:?}");                  //      ║         ║         ║
    name.rename_first("Pizza");                  // ══╗'a║         ║         ║
    println!("name: {name:?}");                  //   ║    ║        ║         ║
} // name, last, first are dropped   ═════════════════╩══╩═════╩═════╝
```

---

static and 'static

The static memory area contains memory where "constant" non-mutable data is stored.

Static memory is often used for global variables, but is more difficult to initialize than local variables.

Static memory is not invalidated until the end of the program. This lifetime is known as the static lifetime.

Here is an example with a static variable which contains a string slice.


```rust
static MAN: &str = "Socrates";

fn man() -> &'static str {
    MAN
}

fn main() {
    println!("{} is a mortal man", man());
}
```

Statics aren't used very often in Rust because constants are usually more convenient and don't require storing a value for the duration of the program.

However, sometimes we may want to have access to a global variable that cannot be initialized at compile time, or is expensive to initialize but only needed when the program is run with certain parameters. An example would be to read configuration parameters that are used all over the program from a file that is not present during compilation. In such a case, we would need a static variable.

Such lazy initialization can be done easily with a OnceCell struct from the once_cell crate (check out the link for an example). By the way, OnceCell is also available in the standard library as an experimental nightly-only feature, and has just recently been stabilized — this paragraph should be updated at some point.

Note also that mutating any static variable in Rust is an unsafe operation.

---

## Trait objects and dynamic dispatch

In the beginning of the course, we may have given the impression that the type of every value in Rust must be known at compile-time. This is true for most part, but we can opt out of it in a special case — albeit with a cost. Rust has special generic types called trait objects, which do not have a concrete type during compile-time.

They are named as such since they are defined through a trait and resemble objects in object-oriented programming (OOP) in that they are containers for both data and behaviour (unlike structs which are separate from their implemented behaviour). If you are familiar interfaces in OOP, trait objects are highly similar to objects created with an interface as the type. We don't need to care of the exact type of the object/instance, only that the needed functionality is there.

---

> Dynamic dispatch and the cost of using trait objects 💸

The conversion of a trait object into a concrete type is not done and cannot be done until runtime because the compiler does not know all the types that may use the trait object. This sort of implementation of polymorphic operation at run-time is referred to as dynamic dispatch. Contrast this with how generic data types are monomorphized into concrete implementation during compile time — the implementation of a polymorphic operation during compile time is known as static dispatch.

Dynamic dispatch comes with some overhead cost as opposed to static dispatch, since resolving the type at runtime is an operation like any other that needs to be computed. As to how big that cost is or the details of how Rust implements dynamic dispatch for trait objects, we will not cover those in this course. Anyhow, for non-performance intensive operations, the cost of using dynamic dispatch is often not noticeable and is often easily worth the added flexibility that the trait objects provide. Also, it is in some cases possible for the compiler to unambiguously infer the concrete type(s) of a trait object and in such cases dynamic dispatch is not needed — we shouldn't rely on that that though.

---

Trait objects provide a convenient way to store references to data of different types, but which implement the same trait. This is useful for storing references to different types of data in a single collection.

Trait objects are created by using the dyn keyword and the trait name. The dyn keyword is required to highlight that the trait object is dynamically dispatched and carries a runtime cost. The keyword wasn't required however until the Rust 2021 edition, but using it is recommended for any version that supports the keyword.

Trait objects are dynamically sized types like str, so they have be used through some kind of a pointer, like a reference.

```rust
trait Speak {
    fn speak(&self) -> &'static str;
}

struct Socrates;
impl Speak for Socrates {
    fn speak(&self) -> &'static str {
        "The hour of departure has arrived and we go our ways; I to die, and you to live. Which is better? Only God knows."
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> &'static str {
        "Meow?"
    }
}

fn main() {
    let speakers: Vec<dyn Speak> = vec![Socrates, Cat];
    for speaker in speakers {
        println!("{}", speaker.speak());
    }
}
```

However, trait objects have somewhat strict rules to ensure that the trait object is safe to use. We can't have e.g. dyn Speak in a Vec but Vec<&dyn Speak> works fine. The details of the safety rules are specified in the object safety section in the Rust reference.

Another option to have trait object is to wrap the trait object in a smart pointer, such as Box to force heap-allocation of values. Having the value heap allocated through a smart pointer is the only option to have owned trait objects in a collection.

```rust
trait Speak {
    fn speak(&self) -> &'static str;
}

struct Socrates;
impl Speak for Socrates {
    fn speak(&self) -> &'static str {
        "The hour of departure has arrived and we go our ways; I to die, and you to live. Which is better? Only God knows."
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> &'static str {
        "Meow?"
    }
}

fn main() {
    let speakers: Vec<Box<dyn Speak>> = vec![Box::new(Socrates), Box::new(Cat)];
    for speaker in speakers {
        println!("{}", speaker.speak());
    }
}
```

If we have a collection of concrete types and want to store them in a collection of trait objects, we can cast the type into a trait object with the as keyword, effectively erasing the compilers knowledge of the type.


```rust
trait Speak {
    fn speak(&self) -> &'static str;
}

struct Socrates;
impl Speak for Socrates {
    fn speak(&self) -> &'static str {
        "The hour of departure has arrived and we go our ways; I to die, and you to live. Which is better? Only God knows."
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> &'static str {
        "Meow?"
    }
}

fn main() {
    let cats = vec![Cat, Cat, Cat];
    let mut speakers: Vec<Box<dyn Speak>> = cats
        .into_iter()
        .map(|cat| Box::new(cat) as Box<dyn Speak>)
        .collect();
    speakers.push(Box::new(Socrates));
    for speaker in speakers {
        println!("{}", speaker.speak());
    }
}
```

Trait objects are especially useful when we are implementing a library. If we want to allow the users of the library to store their own types in a generic collection, the option of using enums goes out of the window — an enum in an external crate cannot be extended with new variants without modifying the external crate itself. For a practical example, see the GUI library example in the Rust Book.

We can also use trait objects for more convenient error handling, by having a function return any Result where the error type implements the Error trait.

Consider for instance a situation where we both read from input (a source for io::Error) and parse the input to integers (a source for num::ParseIntError). In order to use the ? operator for avoiding constant unwrapping we need a generic type that can be both of the return types — the trait object Result<T, Box<dyn Error> is just that.

```rust
use std::error::Error;
use std::io;
use std::result::Result;

fn read_int() -> Result<i32, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let x = read_int()?;
    let y = read_int()?;
    println!("x + y = {}", x + y);
    Ok(())
}
```

The return types are coerced into the trait object type Box<dyn Error> so there is no need for be explicit casting.

---

Smart pointers

Smart pointers are pointer like references (&), but also have additional metadata and capabilities. The most familiar smart pointer to us by now are probably the String and Vec struct that enable having dynamically sized collections by storing values in the heap instead of the stack (only fixed size values are compatible for storage in the stack).

There are many other smart pointers in Rust, which are used for different purposes. The simplest example of a smart pointer is the Box<T> struct, which has the single purpose of storing its wrapped value in the heap (e.g. Box::new([0; 1000])) instead of on the stack. The stack will only contain a pointer to the heap where the value is stored.

More complex smart pointers are for instance the Rc<T> and RefCell<T> which together enable shared ownership. The Rust book provides a chapter on smart pointers that covers Box, Rc and RefCell.

Another common and useful smart pointer is the Cow<'a, B>, i.e. clone-on-write pointer that can boost program performance by performing potentially expensive cloning only when it is actually needed. A practical introduction to Cow can be read from [this blog post](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55){target="_blank"} by Konstantin Grechishchev.

---

### Summary of new syntax and keywords

Symbol	        Description
------------    ---------------
`unsafe`        Defines an unsafe code block or function
`<'a>`	        A generic lifetime parameter
`'a`            A generic lifetime annotation
`'static`       An annotation for a static lifetime
`dyn`           Prefix for a trait object's type

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


