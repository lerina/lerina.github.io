# Traits and Generics

Learning objectives

-   You know how to create and implement traits in Rust
-   You know how to use traits to implement generic functions and structs
-   You know how to use trait objects

## Traits

Traits are used to define shared behaviour for any Rust data type. For instance, the Display trait is used to print values of any type to the console and the Clone trait is used to create a copy of any type. Similarly, numerical operations like addition (+) and multiplication (*) are defined for types with traits Add and Mul.
Defining a trait

Traits are defined using the trait keyword. The trait keyword is followed by the name of the trait and a list of associated functions of the trait. The functions are defined in the same way as they are in a regular implementation block, but can be left without the function body, ending in a semicolon (;) instead. — such trait functions are similar to abstract methods in object-oriented languages.

Consider for instance the definition of the Display trait in std::fmt, which enables printing for a type.

```rust
use std::fmt::{Formatter, Error};
trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
fn main() {}
```

Like associated functions for types, trait functions can be methods by taking self as the first parameter alongside other parameters. No need to worry about the '_ syntax in the Formatter's inner type. It's a lifetime specifier which we'll discuss in the next part of the course.

Traits can also define default implementations for methods. For instance, we can create a trait Shout that has a default implementation of a shout method.

```rust
trait Shout {
    fn shout(&self) {
        println!("hey!");
    }

    fn shout_loudly(&self) {
        println!("HEY!");
    }
}
fn main() {}
```

### Trait bounds

Rust does not support inheritance for types, but it does support extending traits through what Rust calls trait bounds.

Trait bounds can provide functionality or behavior for the unknown self type since they tell the implementing type that it must first implement another trait (the type is bound to the trait). Trait bounds for traits are defined by adding a colon and the binding trait after the trait name (trait TraitName: RequiredTraitName).

With trait bounds, we have access to the behavior of the bound trait. And so, we can make the default implementation for shout more practical by requiring the Display trait for printing self and also for converting self to a string with to_string() for further processing (Display gives ToString trait as a freebie).

```rust
use std::fmt::Display;

trait Shout: Display {
    fn shout(&self) {
        println!("{}!", self);
    }

    fn shout_loudly(&self) {
        println!("{}!", self.to_string().to_uppercase());
    }
}
fn main() {}
```

This makes Shout a subtrait of Display since it extends the functionality of Display. Likewise, Display would be called a supertrait of Shout

### Implementing traits

Implementing a trait for a type uses the same impl keyword we used when implementing associated functions for types. To implement a trait for a type, we write impl Trait for Type { ... }.

Let us first implement the Display trait for the type FullName so that we can print it with the println! macro. To do this, we implement the fmt function of the Display trait with the help of the write macro for writing our desired display string to the formatter struct given as a parameter.

```rust
#![allow(unused)]
use std::fmt::{Display, Formatter, Error};

struct FullName {
    firstname: String,
    lastname: String,
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.firstname, self.lastname)
    }
}

fn main() {
    let name = FullName {
        firstname: "Matti".to_string(),
        lastname: "Meikäläinen".to_string(),
    };
    println!("{}", name);
}
```
  
Note that we used the derive attribute previously to implement the traits like Debug for debug printing. It generates the implementation code automatically from a struct's or enum's code, but not all traits can be automatically derived. The Display trait is one such underivable trait, and so are any custom traits we define. Technically speaking though, any trait can be automatically derived by implementing derive for them, which we'll discuss later when looking at macros and attributes.

Now that our FullName implements Display, we can have it implement the Shout trait too.


```rust
#![allow(unused)]
use std::fmt::{Display, Formatter, Error};

struct FullName {
    firstname: String,
    lastname: String,
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.firstname, self.lastname)
    }
}

trait Shout: Display {
    fn shout(&self) {
        println!("{}!", self);
    }

    fn shout_loudly(&self) {
        println!("{}!", self.to_string().to_uppercase());
    }
}

impl Shout for FullName {}
fn main() {
    let name = FullName {
        firstname: "Matti".to_string(),
        lastname: "Meikäläinen".to_string(),
    };
    println!("{}", name);
    name.shout();
    name.shout_loudly();
}
```

We could override the default implementations of the shout and shout_loudly methods if we wanted to.

```rust
#![allow(unused)]
use std::fmt::{Display, Formatter, Error};

struct FullName {
    firstname: String,
    lastname: String,
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.firstname, self.lastname)
    }
}

trait Shout: Display {
    fn shout(&self) {
        println!("{}!", self);
    }

    fn shout_loudly(&self) {
        println!("{}!", self.to_string().to_uppercase());
    }
}

impl Shout for FullName {
    fn shout(&self) {
        println!("{}!", self.to_string().to_uppercase());
    }
    fn shout_loudly(&self) {
        println!("{}!!!", self.to_string().to_uppercase());
    }
}
fn main() {
    let name = FullName {
        firstname: "Matti".to_string(),
        lastname: "Meikäläinen".to_string(),
    };
    println!("{}", name);
    name.shout();
    name.shout_loudly();
}
```


---

Implementing functions with the names shout and shout_loudly in an impl Type block does not mean that the type now implements the Shout trait. Implementing a trait is done only with the impl Trait for Type syntax — the derive macros (like #[derive(Clone)] use the syntax too, albeit under the hood.

This is in contrast to the duck typing paradigm: "If it walks like a duck and it quacks like a duck, then it must be a duck".

---

### Trait visibility

Trait functions are always public so they don't need to be marked public explicitly. Traits themselves are importable items and as such need to explicitly set public using the pub keyword. Further, traits and their implementations need to be in scope to be able to work with them.

```rust
#![allow(unused)]
struct FullName {
    firstname: String,
    lastname: String,
}

mod shout {
    use std::fmt::{Display, Formatter, Error};
    use super::FullName;

    trait Shout: Display {
        fn shout(&self) {
            println!("{}!", self);
        }

        fn shout_loudly(&self) {
            println!("{}!", self.to_string().to_uppercase());
        }
    }

    impl Display for FullName {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "{} {}", self.firstname, self.lastname)
        }
    }

    impl Shout for FullName {}
}

fn main() {
    let name = FullName {
        firstname: "Matti".to_string(),
        lastname: "Meikäläinen".to_string(),
    };
    println!("{}", name);
    name.shout();
    name.shout_loudly();
}
```

---

Name conflict

A type may have a function with the same name as a trait function.

For example, if the type has a default function and it also implements the Default trait (which gives the associated function default), writing FullName::default() calls the function in the type's impl block, not the one defined by the trait. To call the Default trait implementation's default function, we need a new special syntax. This syntax is known as fully qualified syntax and is quite common in Rust code.

To call the default function provided by impl Default, we have to call <FullName as Default>::default.

```rust
#![allow(unused)]
struct FullName {
    firstname: String,
    lastname: String,
}

impl Default for FullName {
    fn default() -> Self {
        Self {
            firstname: "Matti".to_string(),
            lastname: "Meikäläinen".to_string(),
        }
    }
}

impl FullName {
    fn default() -> Self {
        Self {
            firstname: "Maija".to_string(),
            lastname: "Meikäläinen".to_string(),
        }
    }
}

fn main() {
    let maija = FullName::default();
    println!(
        "{} {}",
        maija.firstname, maija.lastname
    );

    let matti = <FullName as Default>::default();
    println!(
        "{} {}",
        matti.firstname, matti.lastname
    );
}
```

The ambiguity of FullName::default is a likely source of confusion. It is advisable to avoid giving names to associated functions which are already used by traits in the standard library, such as Default::default or Iterator::next.

As a side note, the Default trait can be implemented by a derive attribute which gets the default of all the constituent types. If we derive Default for FullName, the default FullName would have first and last names set to empty strings.

---

## Generics

Generics are a powerful feature of many programming languages that allow code to be written in a way that is more flexible and reusable to avoid duplicated code. Generic data types provide a convenient way to write code that can work with a variety of types. They allow code to operate on abstract (as opposed to concrete) types. Abstract types are filled in by some other part of the code usually with type inference or by using turbofish syntax (::<T>). For instance the abstract type T in Vec<T> can be inferred from a variable type when creating a new vector with Vec::new().

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
}
```

The abstract types can be thought of as variables similar to function parameters and are often labelled as single uppercase characters T, U and so forth, as we can see for example in the documentation for tuple, but they don't need to be (e.g. Result uses E for its error type).

The provision of such generic data types is commonly known as polymorphism (The word is Greek where poly means many and morph means form or shape), or more precisely, parametric polymorphism.

The generic types in Rust get [monomorphized](https://en.wikipedia.org/wiki/Monomorphization){target="_blank"} at compile time. This means that the compiler generates a separate concrete item for each type that is used with the generic item. This makes using generics as fast (during runtime) as using only concrete types, but may cause slowness during compile time and will naturally take space in the resulting compiled binary. In other words, generic functions work as a template for creating new functions during compilation.

Let's consider an example with functions to duplicate values of various types by copying or cloning them.

```rust
#![allow(unused)]
fn duplicate(int: i32) -> (i32, i32) {
    (int, int)
}

// error: the name `duplicate` is defined multiple times
fn duplicate(string: String) -> (String, String) {
    (string.clone(), string)
}
fn main() {}
```

In Rust, we cannot create two functions with the same name (in the same scope) so we need a unique name for each function. This code for duplicating two types is somewhat verbose, and more so if it needs to support more types in the future.

We have actually gone through one form of polymorphism in Rust already before as we could use enums to have a collection of multiple different types. However, if we resorted to an enum for polymorphism here, we could work with only one function, but that would be quite the hassle.

```rust
#![allow(unused)]
enum Type {
    I32(i32),
    String(String),
}

fn duplicate(value: Type) -> (Type, Type) {
    match value {
        Type::I32(int) => (Type::I32(int), Type::I32(int)),
        Type::String(string) => (
            Type::String(string.clone()),
            Type::String(string),
        ),
    }
}
fn main() {}
```

Generics to the rescue!

### Generic types and functions

To create a generic function, we append the function name with angle brackets <>, and insert generic type parameters within, like <T> (not to be confused with parameter type). A generic type parameter is similar to a function parameter, but instead of taking a value argument, it takes a type argument. The generic type T can then be used in the function declaration as well as inside the function scope like any other type.

With generics, we can have a clean and concise duplicate function that works with any type.


```rust
#![allow(unused)]
fn duplicate<T>(t: T) -> (T, T) {
    (t.clone(), t)
}

fn main() {
    let (a, b) = duplicate(5f32);        // T = f32 is inferred
    let (a, b): (i8, i8) = duplicate(5); // T = i8 is inferred
    let (a, b) = duplicate::<usize>(5);  // T = usize is turbofished 

    let (a, b) = duplicate::<String>("5".to_string()); // T = String is turbofished
}
```

However, the code doesn't compile due to, you probably already guessed it, the generic type T does not implement Clone and therefore there is no method clone to call.

Examining the compiler error, we see that in order to use the clone method, we need to add a trait bound to the generic type T so that T implements Clone, similarly as we needed to add a Display trait bound for the subtrait Shout to print self in a trait method.

We can add multiple trait bounds to a generic type parameter by separating them with a + sign. For example, we can add a Debug trait bound to the generic type T in duplicate so that we can print the value of t in the println! macro.

```rust
#![allow(unused)]
use std::fmt::Debug;

fn duplicate<T: Clone + Debug>(t: T) -> (T, T) {
    println!("Duplicating {t:?} to ({t:?}, {t:?})");
    (t.clone(), t)
}
fn main() {
    let (a, b) = duplicate(5f32);
    let (a, b): (i8, i8) = duplicate(5);
}
```

---

Where syntax 🔍

With multiple trait bounds, the <> syntax can become quite verbose and difficult to read. Rust provides also where syntax to add trait bounds to generic types, which is useful for longer trait bounds, and especially when the trait bound is too long to fit on a single line.

```rust
#![allow(unused)]
use std::fmt::Debug;

fn tuplify<T, U>(t: &T, u: &U) -> (T, U)
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    (t.clone(), u.clone())
}
fn main() {
    let (a, b) = tuplify(&5f32, &"5".to_string());
    println!("({a:?}, {b:?})");
}
```

---

### Impl trait

The duplicate and tuplify functions use the generic type T in their parameter type as well as in their return type. Sometimes the function only needs to be generic over its input or its output. In such cases, we can use shorter syntax with by writing merely impl Trait in place of a generic type.

To see this in action, let's first create a generic greet function which says hello to a given value as long as the value is printable. In case we don't remember or know the type of the input, we can give our parameter a generic type with no bounds whatsoever.

```rust
#![allow(unused)]
fn greet<T>(value: &T) {
    println!("Hello {value}!");
}
fn main() {}
```

Then, from the compiler error, we can see that the type T does not implement the std::fmt::Display trait. So we can either add that as a trait bound, or replace the generic type parameter with just impl std::fmt::Display.

```rust
#![allow(unused)]
use std::fmt::Display;

fn greet(value: &impl Display) {
    println!("Hello {value}!");
}

fn main() {
    greet(&5f32);
    greet(&5);
    greet(&"🤭".to_string());
    greet("👋");
}
```

Except, we have a problem with greet("👋"); because the compiler doesn't know the size of the string literal "👋" at compile time, and the size is implicitly required by any type parameter through the Sized trait. We can fix this by following compiler instructions, which tells us to relax the constraint with the special syntax + ?Sized that removes the implicit Sized constraint in impl Trait.

The two versions of the greet function, (1) the one with the named generic T and trait bound, and (2) the one with just the &impl Display for type, behave in exactly the same way. The setback when using the impl Trait syntax as opposed to having a generic type is that there is no way to refer to the type, for example to constrain two values to have the same generic type T. The lack of a generic type parameter also means that we cannot use the turbofish syntax to specify the concrete type.

```rust
#![allow(unused)]
use std::fmt::Display;

fn greet(value: &impl Display) {
    println!("Hello {value}!");
}

fn main() {
    greet<i8>(&5);
}
```

Returning impl Trait is also subtly different to having a generic return type. As an example, with impl Clones in place of generic types in the duplicate function from the earlier example would not allow assigning values from the function call to an integer tuple.

```rust
#![allow(unused)]
fn duplicate(t: impl Clone) -> (impl Clone, impl Clone) {
    (t.clone(), t)
}

fn main() {
    let (a, b) = duplicate(5f32); // works fine
    let (a, b): (i8, i8) = duplicate(5); // error
}
```

Running the above example reveals that the compiler identifies a mismatch between the return type (impl Clone, impl Clone) and the expected type (i8, i8) of the variable. We can also see that the compiler calls the types of the form impl Trait as opaque types. The term opaque refers to the only thing known about such types is that they implement the trait and nothing else. Knowing only that the returned values implement the trait Clone, the compiler has no way to identify that such values are of type i8, causing the error. Thus, using a generic type is much more useful here — using opaque types obfuscates the relation of the parameter type and return type which leads to very limited usability.

### Generics in structs and enums

The standard library contains a plethora of structs with generic data types, like the Option<T>, Result<T, E> and Vec<T>. There are even some primitive types with generics, like &T and &mut T (any borrowed type and any mutably borrowed type). Generics in structs can be used with similar syntax as with generic functions.

As an example, we'll create a generic Optional data type using an enum identical to that of Option<T>. We write enum Optional<T> to declare T as a generic type parameter which can (and has to) be used inside the enum. Each concrete Optional type, like Optional<i32> or Optional<String>, is unique, which means we can implement different methods for different Optionals based on the type of T.

```rust
#[allow(unused)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Optional<T> {
    Some(T),
    None
}

impl Optional<i32> {
    fn five() -> Self {
        Self::Some(5)
    }
}

impl Optional<String> {
    fn five() -> Self {
        Self::Some("five".to_string())
    }
}

fn main() {
    let some_five = Optional::five();
    println!("{some_five:?}");
}
```

Like the solution code shows, we can specify the concrete type of Optional using the turbofish operator. Type inference wouldn't work here, because both five functions are distinct, and type inference cannot select between distinct functions.

Note also that we can use the derive attribute to automatically implement traits for structs with abstract types as we would normally. The deriving is done for each concrete use of the abstract type if applicable. For instance, with #[derive(Clone, Copy)], Optional<i32> would get Copy and Clone but Optional<String> would get only Clone.

---

Type-safe dice rolling

Unit structs are useful for parameterizing generic data types to write more abstract code with distinct functions with custom logic.

Let's create a dice rolling game mock-up with a generic Game<D> data type and different dice D6 and D12 represented by unit structs.

```rust
#![allow(unused)]
use rand::Rng; // We need the declaring trait in scope for gen_range

#[derive(Debug)]
struct D6;

#[derive(Debug)]
struct D12;

#[derive(Debug)]
struct Game<D> {
    die: D,
    rolls: usize,
}

impl Game<D6> {
    fn roll(&mut self) {
        let outcome: u8 = rand::thread_rng().gen_range(1..=6);
        self.rolls += 1;
        println!(
            "Roll number {} of a 6 faced die resulted in {outcome}",
            self.rolls
        );
    }
}

impl Game<D12> {
    fn roll(&mut self) {
        let outcome: u8 = rand::thread_rng().gen_range(1..=12);
        self.rolls += 1;
        if outcome == 12 {
            println!("A lucky 12!");
        } else {
            println!("Rolling a 12 faced die resulted in {outcome}");
        }
    }
}

fn main() {
    let mut game = Game {
        die: D6,
        rolls: 0,
    };
    for _ in 0..5 {
        game.roll();
    };

    println!("Switching games");
    let mut game = Game {
        die: D12,
        rolls: 0,
    };
    for _ in 0..5 {
        game.roll();
    }
}
```

The different games (Game<D6> and Game<D12>) as well as the roll functions are distinct, which we can see in the following example with deliberate illegal code.

```rust
#![allow(unused)]
use rand::Rng;
#[derive(Debug)]
struct D6;
#[derive(Debug)]
struct D12;
#[derive(Debug)]
struct Game<D> {
    die: D,
    rolls: usize,
}
impl Game<D6> {
    fn roll(&mut self) {
        let outcome: u8 = rand::thread_rng().gen_range(1..=6);
        self.rolls += 1;
        println!(
            "Roll number {} of a 6 faced die resulted in {outcome}",
            self.rolls
        );
    }
}
impl Game<D12> {
    fn roll(&mut self) {
        let outcome: u8 = rand::thread_rng().gen_range(1..=12);
        self.rolls += 1;
        if outcome == 12 {
            println!("A lucky 12!");
        } else {
            println!("Rolling a 12 faced die resulted in {outcome}");
        }
    }
}
fn main() {
    let mut game = Game {
        die: D6,
        rolls: 0,
    };

    game = Game {
        die: D12,                 // error: mismatched types
        rolls: 0,
    };
    Game::<D12>::roll(&mut game); // error: incorrect type
    Game::roll(&mut game);        // error: multiple applicable items in scope
}
```

This dice rolling example follows the Rust idiom, that we should make the type system work for us to spot bugs at compile time.

We have assumed here that the game cannot change dice in the middle of it. If we wanted the dice to change mid-game we would have to use enums or dyn trait objects.

---

### Generic implementations

To implement methods and traits for a generic Optional<T>, we need to parameterize the impl block with a generic type.

Let's create an unwrap method for all Optional types, which given a Some returns the wrapped value and otherwise panics. Implementation blocks can be parameterized with generics using impl<T> syntax.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Optional<T> {
    Some(T),
    None
}

impl Optional<i32> {
    fn five() -> Self {
        Self::Some(5)
    }
}

impl Optional<String> {
    fn five() -> Self {
        Self::Some("five".to_string())
    }
}

impl<T> Optional<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!("Cannot unwrap a None value"),
        }
    }
}

fn main() {
    let five = Optional::<i32>::five();
    print!("unwrapping {five:?} gives ");
    println!("{}", five.unwrap());
}
```


We can write multiple impl blocks with different generic parameters and trait bounds to specify additional behavior required for calling the associated functions within.

For example, let's add a method called unwrap_or_default, which unwraps the Optional<T> value or returns the Default value of T. We can write trait bounds on a generic impl the same way as with functions and structs, impl<T: Trait>.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Optional<T> {
    Some(T),
    None
}

impl<T> Optional<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!("Cannot unwrap a None value"),
        }
    }
}

impl<T: Default> Optional<T> {
    fn unwrap_or_default(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => T::default(),
        }
    }
}

fn main() {
    let optional_number = Optional::<i32>::None;
    println!(
        "optional_number.unwrap_or_default() results in {}",
        optional_number.unwrap_or_default()
    ); // works

    let optional_number_ref = Optional::<&i32>::None;
    println!(
        "optional_number_ref.unwrap_or_default() results in {}",
        optional_number_ref.unwrap_or_default()
    ); // error: trait bounds were not satisfied
}
```

As we can see in the error, &i32 does not implement Default, which means that the method unwrap_or_default is not implemented for Optional<&i32>. Most references do not implement Default except for a couple of slice types, like &str and &[T], which both return an empty slice from their implementation.

### Generic methods

Like any other function, associated functions can be generic as well. The data types generic associated functions are implemented for are usually generic, but don't have to be.

For the sake of complexity, let's implement Option::map for Optional. The implementation is straight-forward: we just match and call f with the wrapped value wrapping the result in Some. However, the type signature will be harder to come up with, because we need map to accept functions.

Functions and closures in Rust implement traits known as the Fn traits. The traits Fn, FnOnce and FnMut are described in more detail in The Rust Book chapter 13.1. We will not go into detail on the different Fn traits but instead pick the most general one, Fn, as the trait bound for F. Fn traits are written using special syntax to indicate the signature of the function. In our case we need F to take in a value of T and return a value of T. This is achieved by the trait bound Fn(T) -> T.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Optional<T> {
    Some(T),
    None
}
impl<T> Optional<T> {
    fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        match self {
            Self::Some(t) => Self::Some(f(t)),
            Self::None => Self::None,
        }
    }
}

fn main() {
    let five = Optional::Some(5);
    let six = five.map(|x| x + 1);
    println!("{six:?}");
}
```


Because Optional<T> can hold any type, there is no reason to restrict the mapping function to return a value of the same type. To allow for more generic mapping, we can add another type parameter U to the method.

```rust
#![allow(unused)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Optional<T> {
    Some(T),
    None
}
impl<T> Optional<T> {
    fn map<F: Fn(T) -> U, U>(self, f: F) -> Optional<U> {
        match self {
            Self::Some(t) => Optional::<U>::Some(f(t)),
            Self::None => Optional::<U>::None,
        }
    }
}

fn main() {
    let five = Optional::Some(5);
    let five_string = five.map(|x| x.to_string());
    println!("{five_string:?}");
}
```

To make the return type correct, we need to also return the Some and None variants of to the resultant type Optional<U>. Note that the Self type is specified as Optional<T> so Self<U> would be Optional<T><U> which is invalid.

Generic traits

A generic trait, like a generic function or a generic struct or enum, is parameterized by one or more generic type parameters. The syntax for creating generic traits is the same old angle brackets appended after the trait's name.

The Rust standard library provides two particularly useful and closely related traits, From and Into that we'll take a closer look into. While the as operator is used to cast primitive types using built-in compiler logic, the From and Into traits provide a way to implicitly convert custom types into other types.

The traits are defined as follows.

```rust
#![allow(unused)]
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

trait Into<T>: Sized {
    fn into(self) -> T;
}
fn main() {}
```

Let's consider an example with two custom data types, Kilometers and Miles

```rust
#![allow(unused)]
#[derive(Copy, Clone, Debug)]
struct Miles(f32);

#[derive(Copy, Clone, Debug)]
struct Kilometers(f32);

const KILOMETERS_PER_MILE: f32 = 1.609344;
fn main() {}
```

The floating point values contained within the structs carry different meanings, which conveys that arithmetic operations between Kilometers and Miles should not be computed without prior conversion.

To cast Kilometers into Miles, we can implement the generic From<Kilometers> trait for Miles.

```rust
#![allow(unused)]
#[derive(Copy, Clone, Debug)]
struct Miles(f32);

#[derive(Copy, Clone, Debug)]
struct Kilometers(f32);

const KILOMETERS_PER_MILE: f32 = 1.609344;

impl From<Miles> for Kilometers {
    fn from(value: Miles) -> Self {
        Self(value.0 * KILOMETERS_PER_MILE)
    }
}

fn main() {
    let miles = Miles(5.0);
    let kilometers: Kilometers = miles.into(); // into?
    println!("{miles:?} is {kilometers:?}");
}
```

The conversion now works from Kilometers to Miles not only by using from but also through into. We did not implement the into method for Kilometers in the code, so where does the implementation come from? The answer is: from the standard library via a blanket implementation. Blanket implementations are implementations of a trait on any type that satisfies given trait bounds.

The blanket implementation providing us with free Into implementations in the standard library is defined in simplified form as in the following example.

```rust
#![allow(unused)]
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}
pub trait Into<T>: Sized {
    fn into(self) -> T;
}
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
fn main() {}
```


Similarly, the standard library provides a blanket implementation of ToString for any type that implements Display, which is why we get to_string method for all T: Display.


## Associated types

Associated types are a way of defining types that can be used in a trait's associated functions. They can be considered an advanced concepts, but are still fairly common.

For instance, numerical operators such as +, -, ==, < and > are made available for types using traits that leverage associated types.

As an example, we'll implement addition for Vecs which joins the vectors together as this is not featured in the standard library. The trait for addition is aptly named Add.

The Add trait, being the most complex trait we will see on the course, needs some introduction. Its path in the standard library is std::ops::Add, which needs to be imported in order to be used. The trait is declared as follows.

```rust
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
fn main() {}
```

The trait is parameterized by two types. A generic type used for the value in the right-hand side of the addition is set to Self by default using default type parameters syntax: <Rhs = Self>. The Add trait also has an associated type, Output, which is used as the return type of the addition. The level of abstraction makes addition very powerful in Rust as it allows for adding different types of values together producing an output value with a potentially different type too.

This will not work, however, because we have broken Rust's orphan rules, which state that we shall not implement a foreign trait for a foreign type. We'll get an error message warning us about this and a suggestion to define a trait or new type instead.

```rust
use std::ops::Add;
// This is the same as: impl<T> Add<Vec<T>> for Vec<T>
impl<T> Add for Vec<T> {
    type Output = Self;

    fn add(mut self, rhs: Vec<T>) -> Self::Output {
        self.extend(rhs);
        self
    }
}
fn main() {}
```

---

Orphan Rules

 order to get around this limitation, we follow the compiler's suggestion and define a new type.

A newtype is a tuple struct with exactly one field. Newtypes are used in many functional programming languages (including Rust) to create alternative trait implementations for types, or to work around the orphan rules.

Let's first create a newtype struct Wrapper<T> which holds a Vec<T>.

```rust
#[derive(Debug)]
struct Wrapper<T>(Vec<T>);

fn main() {
    let vec = Wrapper(vec![1, 2, 3]);
    println!("{vec:?}");
}
```

Then let's implement Add for all Wrapper<T>s returning another Wrapper<T> from the addition operation.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Wrapper<T>(Vec<T>);

impl<T> Add for Wrapper<T> {
    type Output = Self;

    fn add(mut self, rhs: Wrapper<T>) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}

fn main() {
    let a = Wrapper(vec!["I", "have"]);
    let b = Wrapper(vec!["been", "wrapped"]);
    let ab = a + b;
    println!("{ab:?}");
}
```

A newtype does not inherit the methods and traits defined on the inner type, thus we always need to use .0 in order to operate on the Vec inside the Wrapper.








