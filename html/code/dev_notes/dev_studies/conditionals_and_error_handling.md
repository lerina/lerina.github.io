# Conditionals and Error Handling

[source](https://fitech101.aalto.fi/programming-languages/rust/5-conditionals-and-error-handling/)

Control flow is the order in which the computer executes statements in a program. We have already manipulated control flow through calling functions — the computation flow of the program jumps into the called function instead of directly continuing on the next line of the code.

For more fine-grained control, we can use conditional statements such as if statements, which we'll look at next

## If, else if and else

We can use if, else if and else keywords combined with conditions to control the flow of execution, or in other words, to decide what to execute and when. The syntax is similar to many other languages such as C, Java and Python.

As an example, the following code snippet prints out the polarity of the integer x.

```rust
fn main() {
    let x = 0;
    if x > 0 {
        println!("positive");
    } else if x < 0 {
        println!("negative");
    } else {
        println!("neutral");
    };
}
```

The condition operand (e.g. x > 0 in the example) is the expression after the if keyword which determines whether to enter or skip the if block. The condition operand must evaluate to a boolean value, i.e. true or false. Numbers 1 and 0 do not equal true and false in Rust like in the C language.

```rust
fn main() {
    let x = 1;
    if x { // error: expected `bool`, found integer
        println!("x is true!");
    };
}
```

Note also that the condition operand is not surrounded by parentheses, since they are not necessary in Rust unlike in many other languages. The compiler will even warn if we add unnecessary parentheses around the condition operand.

```rust
fn main() {
    let velocity = 0;
    if (velocity > 0) { // warning: unnecessary parentheses around `if` condition
        println!("Moving forward!");
    };
}
```

On the other hand, the curly braces for the if block are mandatory and the if block must contain at least one statement, lest we face the wrath of the compiler.


### Ifs are expressions

Similarly to code blocks, an if statement is an expression like any other expression in Rust. Leaving out semicolons from each branch causes the if expression to evaluate into the value. When the branches end in semicolons, the if expression evaluates to the unit value ().

```rust
fn main() {
    let x = i32::MIN;  // minimum value of i32 integer
    let polarity = if x > 0 {
        "positive";
    } else if x < 0 {
        "negative";
    } else {
        "neutral";
    };
    println!("{x} is {polarity}"); // ()
}
```

By leaving out the semicolon from an if expression, we can return the value of the if expression as a value from a function just like any other value.

```rust
fn polarity(number: i32) -> String {
    if number > 0 {
        "positive".to_string()
    } else if number < 0 {
        "negative".to_string()
    } else {
        "neutral".to_string()
    }; // error: expected `String`, found `()`
}

fn main() {
    let x = i32::MAX;  // minimum value of i32 integer
    let polarity = polarity(x);
    println!("{x} is {polarity}");   // negative
}
```

---

### Ternary operator?

A conditional ternary operator is a common feature in many programming languages that allows concise syntax for simple if-else expressions. Most commonly (for instance in C, JavaScript and Dart) the ternary operator is represented by a question mark (?) for the true condition and a colon (:) for the false condition. For example in C we could do the following.

```c
#include <stdio.h>

int main() {
  int age = 18;
  bool canVote = (age >= 18) ? true : false;
  printf("Can vote: %s", canVote ? "true" : "false");
}
```

Rust does not need such a ternary operator, since the if statements in Rust are expressions, 
we can achieve the same result almost as easily (but more expressively).

```rust
fn main() {
    let age = 18;
    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {can_vote}");
}
```

---

When if is used as an expression, all cases must be covered by the branches and the branches must evaluate to the same type (unless returning early, e.g. with the return keyword). Otherwise, the compiler will complain.

```rust
fn is_small(text: &str) -> bool {
  let length = if text == "small" {
    true
  } else {
    text.len()
  };
  length < 5
}

fn main() {
  println!("{}", is_small("small"));
  println!("{}", is_small("smol"));
  println!("{}", is_small("smoll"));
}
```

---

### Early returning with guards

We can use the return keyword to return a value from a function early. The value after return will be the value of the function call and the function will exit immediately.

```rust
fn early_return() -> i32 {
    return 1;
    2 // warning: unreachable expression
}

fn main() {
    println!("{}", early_return());
}
```

In more complex code, returning early with guards can be a good way to keep the control flow readable by avoiding deeply nested blocks.

```rust
fn get_price(in_stock: i32, price_per_kg: i32, kg: i32) -> i32 {
    if in_stock > 0 {
        if price_per_kg > 0 {
            if in_stock >= kg {
                price_per_kg * kg
            } else {
                -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}
fn main() {
    println!("{}", get_price(10, 15, 0));
}
```

The convention in Rust is to not use return keyword when same behavior is achieved by removing the semicolon from the lines that return a value.

```rust
fn collatz(a: i32) -> i32 {
    if a % 2 == 0 {
        return a / 2;     // Clippy warning: unneeded return statement
    } else {
        return 3 * a + 1; // Clippy warning: unneeded return statement
    }
}
fn main() {
    println!("{}", collatz(15));
}
```


By running the Rust linter Clippy with cargo clippy for the above code, we get a code style suggestion to remove the unnecessary return keywords.

After modifying the code as Clippy suggests, it will still behave in the exact same way.

Clippy will not suggest removing the return keyword when doing so would change the behavior of the program, such as when using early returns.

---

### Comparison operators

An if statement need to be paired with a boolean expression (an expression that evaluates to either true or false) after the if keyword. We can write boolean expressions using comparison operators like we did in the previous examples.

The following table lists the comparison operators available in Rust.


Operator	      Meaning
------------    ------------------------
==	            equals
!=	            not equals
<	              less than
>	              greater than
<=	            less than or equals
>=	            greater than or equals

The logical operators ! (negation), && (and) and || (or) may be applied to boolean expressions. The result will also be a boolean and thus valid for use in an if statement.


Operator	Meaning
--------  ---------
!	        negation
&&	      and
||	      or

As an example, we can use the logical and operator (&&) to reduce the number nesting levels.

```rust
fn get_price(in_stock: i32, price_per_kg: i32, kg: i32) -> i32 {
    if in_stock > 0 {
        if price_per_kg > 0 {
            if in_stock >= kg {
                price_per_kg * kg
            } else {
                -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

fn main() {
    println!("{}", get_price(10, 15, 0));
}
```

---

### Floating-point comparasionwith epsilon

Integer comparison works like we are used to and have already seen.

```rust
fn main() {
    println!("{}", 5 < 6);      // true
    println!("{}", 6 <= 6);     // true
    println!("{}", 1 + 1 == 3); // false
}
```

Floating point comparison on the other hand might not work as we expect due to the way floating point numbers are encoded in bytes — floating point encoding is not a topic for this course but more info can be found here.

```rust
fn main() {
    println!("{}", 3.0 * 0.15 == 0.45); // false
    println!("{}", 3.0 * 0.15 < 0.45);  // true
    println!("{} < {}", 3.0 * 0.15, 0.45);
}
```

The important thing to notice here is that floating point calculations are **not always exact**.

```rust
fn main() {
    println!("{}", 0.09 + 0.01) // 0.0999...
}
```

For more reliable floating point comparison, we can use an error margin suitable for our purposes and see if the values are within the margin's radius of each other. Rust provides a handy constant EPSILON that can be used as a baseline error margin.

```rust
fn main() {
    let a = 3.0 * 0.15; // type gets inferred as f64 from b
    let b = 0.45f64;
    println!("{a} == {b} ± {}", f64::EPSILON);
    println!("{}", (a - b).abs() < f64::EPSILON); // (a - b).abs() evaluates to the absolute difference of the two values
}
```

---

## Matching patterns

Besides the if expression for conditional control flow, Rust has a match expression that is based on pattern matching. The simplest pattern is a literal value, like the boolean value true. Matches are always exhaustive, meaning that all possible values of the type must be covered by the patterns.

```rust
fn main() {
    let fact = true;
    match fact {
        true => println!("fact"),
        false => println!("not fact"),
    };
}
```

The match syntax consists of the match keyword, a scrutinee expression, and match arms. The value of the scrutinee expression is compared to patterns in the arms, which are enclosed in braces {}. Each arm has a pattern, followed by a fat arrow => and a target expression. Like with if and else if conditions, the matching is done from top to bottom, and the first matching arm of the expression is evaluated.

The match expression in Rust is a lot like the switch statement you may have come across in other programming languages, but more functional and powerful as it is an expression and supports pattern matching.

As stated earlier, all values of the type must be covered by the patterns. For booleans it is enough for the patterns to cover the two values true and false. For integers, we need to match the full range an integer can take. We can achieve this by using range patterns with the .. operator. The pattern ..a matches all numbers from minimum (e.g. i32::MIN for i32) to a, a.. from a to maximum, and a..=b from a to b (inclusive range).

```rust
fn main() {
    let number = 5;
    let polarity = match number {
        ..=-1 => "negative!",
        0 => "neutral!",
        1..=i32::MAX => "positive!",
    };
    println!("{polarity}");
}
```

We can use a variable name as the pattern in the final arm in case we don't want to explicitly match all possible values (or it is impossible, like when working with values such as strings that can take an arbitrary number of values). Using a variable as a pattern will match any value and bind it to the variable name.

```rust
fn main() {
    let number = 5;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        n => println!("{n} is not one, two or three!"),
    }
}
```

This leaves us a redundant variable though, since we might as well use the scrutinee instead of the new variable as in the above example. Also, in many cases we don't need to do anything with the value that matched the pattern, but the compiler will warn us about any unused variables, as it should.

To get rid of the warning, we can use the underscore _ wildcard pattern. Like a variable name, this pattern will match any value but also ignore the value, thus keeping us safe from unused variable warnings.

```rust
fn roman_numeral(num: usize) -> String {
    match num {
        1..=3 => "I".repeat(num),
        4 | 9 => "I".to_string() + &roman_numeral(num + 1),
        5..=8 => "V".to_string() + &roman_numeral(num - 5),
        10..=39 => "X".to_string() + &roman_numeral(num - 10),
        _ => String::new(), // underscore is like else with if
    }
}
fn main() {
    println!(
        "{} + {} = {}",
        roman_numeral(14),
        roman_numeral(19),
        roman_numeral(14 + 19)
    );
}
```


The above example also shows how to combine patterns into one with |.

Match target expressions (which come after the =>) work the same as branches in an if expression. The expressions can be any expression as long as they evaluate to a value of the correct type or return early from the function.

```rust
fn is_big(text: &str) -> bool {
    let len = match text {
        "big" => return true,
        text => { 
          let trimmed = text.trim();
          trimmed.len()
        },
    };
    len > 10
}
fn main() {
    println!("{}", is_big("big"));
    println!("{}", is_big("small"));
    println!("{}", is_big("a very long text"));
}
```

### Match guards

We can use match guards to add conditions for when to match arm patterns. A match guard is defined by adding an if keyword and a condition after the pattern in a match arm.

```rust
fn main() {
    let x = 10;
    let polarity = match x {
        0 => "neutral",
        _ if x > 0 => "positive",
        _ if x < 0 => "negative", // problem?
    };
    println!("{polarity}");
}
```

The condition is evaluated after the pattern is matched, and the arm is only evaluated if the condition is true. The compiler isn't however capable of determining whether a collection of guards cover all the possible cases of a pattern. To fix the above example, we can remove the final guard to have the wildcard _ capture all the remaining possible values.


## Handling errors 

### by panicking

Errors are bound to happen in real-world programs and they need to be handled accordingly. Rust has a few mechanisms to handle errors, and one of them is panicking — panicking is the technical term used for crashing and stopping the execution of a Rust program.

A Rust program can panic in two ways, either by an action that leads to panic, such as integer division by zero, or by explicitly calling Rust's panic! macro.

The panic! macro is used to indicate that something went wrong and the program will not continue. It takes a format string as its argument like the print and println macros, and the argument is printed as the error message when the program halts.

```rust
fn main() {
    println!("What is the airspeed velocity of an unladen swallow?");
    println!("What do you mean, an African or European swallow?");
    panic!("I don't know that!");
}
```

As a more practical example, consider a program responsible for handling the state of a drink machine that can only hold 50 drinks in total at a time. If the program states that the machine has more drinks than it can physically hold, it should panic and show an error message.

```rust
fn get_drink_count() -> u8 {
    51
}
fn main() {
    let drinks = get_drink_count();
    if drinks > 50 {
        panic!("Not possible to have {drinks} drinks, invalid state!");
    }
}
```

### Optional values

Many programming languages incorporate a special value called null to represent the absence of a value. Consider for example the previous example function that matched integers to their respective Roman numerals and returned an empty string when given an unsupported number. Assuming that we don't want to panic instead of returning a value, the function behavior would be clearer if in such a case it would return an empty (null) value (an empty string is not an empty value).

Rust does not allow a value of a variable to be empty or null. Instead the possible absence of a value is represented by the Option type

#### The Option type

The Option type is an enum with two variants: Some and None. The Some variant indicates that the value is present, while the None variant indicates that the value is absent.

Using the Option for null values is Rust's way of making the presence of null values explicit in the code. This allows null errors to be easily identified during compilation so that they can be quickly found and fixed before running the program.


```rust
pub fn roman_numeral(num: usize) -> String {
    match num {
        1..=3 => "I".repeat(num),
        4 | 9 => "I".to_string() + &roman_numeral(num + 1),
        5 => "V".to_string(),
        6..=8 => "V".to_string() + &roman_numeral(num - 5),
        10 => "X".to_string(),
        10..=39 => "X".to_string() + &roman_numeral(num - 10),
        _ => String::new(),
    }
}

fn main() {
    println!("{}", roman_numeral(24));
    println!("{}", roman_numeral(0));
}
```

With the Option type, the previous Roman numeral example can be rewritten as

```rust
fn roman_numeral(num: usize) -> Option<String> {
    match num {
        1..=3 => Some("I".repeat(num)),
        4 | 9 => Some("I".to_string() + &roman_numeral(num + 1).unwrap_or_default()),
        5..=8 => Some("V".to_string() + &roman_numeral(num - 5).unwrap_or_default()),
        10..=39 => Some("X".to_string() + &roman_numeral(num - 10).unwrap_or_default()),
        _ => None, 
    }
}
fn main() {
    println!(
        "{} + {} = {}",
        roman_numeral(14).unwrap(),
        roman_numeral(19).unwrap(),
        roman_numeral(14 + 19).unwrap()
    );
}
```

This makes it clear that the function can return a null value, and the caller of the function knows that the null value needs to be handled appropriately. In the example function, the null values are handled using the unwrap_or_default() method, which returns a default value for the given type — an empty string for a String in the example's case.

The inner type of Option is generic, so Option can be used with any type. The inner type of the option needs to be included in Option's type signature with angle brackets <>, like Option<String> in the above example.

#### Printing an Option and debug formatting

Some types in Rust cannot be printed out normally, but support debug formatting which is commonly used for printing more information about the value. Option is one such type.


```rust
fn main() {
    let maybe_x: Option<i32> = Some(5);
    println!("{maybe_x}"); // cannot be formatted with the default formatter
}
```

To print out such values, the debug formatting parameter ? can be used in the format string of the print(ln)! macros using {:?} instead of just the braces {}.

As a side note, printing a String with {:?} will allow us to see the more clearly if the string is empty.

```rust
fn main() {
    let empty_string = String::new();
    println!("{empty_string:?}"); // ""
}
```

---

> dbg!() macro

Sometimes we want to debug what's happening. Writing println!("{variable:?}") every time is quite inconvenient — luckily Rust has a convenient macro for doing the same thing: dbg!.

```rust
fn main() {
    let x = Some(1);
    dbg!(x); // [src/main.rs:3] x = Some(1)
    let y = dbg!(x.unwrap()) + 6; // x.unwrap() = 1
    dbg!(y); // y = 7
}
```

Not only does the dbg! macro print the filename, line number and debugged expression, it also returns the argument so that we can debug parts of expressions without having to extract them to a variable! Similar to the print macros is the format! macro that writes formatted text to a string. The format! macro is especially useful for concatenating strings and data types.

```rust
fn main() {
    let x = Some(1);
    let variable = "x";
    let s = format!("{} is {:?}", variable, x);
    println!("{s}");  // x is Some(1)
}
```

---

### Handling null values

A simple way to handle null errors is by explicitly causing the program to crash if the value is None. This can be done by unwrapping the Option using the method unwrap().

```rust
fn main() {
    let mut maybe3 = Some(3);
    let three: i32 = maybe3.unwrap();
    println!("{three}"); // 3

    maybe3 = None;
    println!("{}", maybe3.unwrap()); // panic, called unwrap() on a None value
}
```

When we want to avoid crashing, a common way to handle null values is to define a default value in case the Option is None — like in the Roman numerals example with the unwrap_or_default(), which returns a default value based on the Options inner type.

We can also provide a custom default value with the unwrap_or method.

```rust
fn main() {
    let mut maybe_a_value = Some("value");
    println!("{}", maybe_a_value.unwrap_or("null")); // value
    maybe_a_value = None;
    println!("{}", maybe_a_value.unwrap_or("null")); // null
}
```

Another useful method is expect, which causes panic like unwrap, but also allows us to specify a custom error message to be shown after the panic.

```rust
fn main() {
    let bob: Option<bool> = None;
    println!("{}", bob.expect("bob does not have a value")); // thread 'main' panicked at 'bob does not have a value'
}
```

### None and its type

While floating point values require explicit type information for certain operations, there are also cases where simply defining a variable is not allowed without explicit typing. For instance, if we try to assign None (a "null" value) to a variable, the compiler would not allow it, because it cannot infer the type of the None value nor the type of the variable.

```rust
#![allow(unused)]
fn main() {
    let x = None;
}
```

Rust is able to hint that None is of type Option<T>, but it also needs to know what it is an option of — that's the <T> part in the type. To fix the error, we need to define the data type of the variable in the code (to see the solution, press the red icon). In the solution, None was arbitrarily chosen to be an Option of i32.

### Destructuring Options and if let

Patterns can be used to destructure complex values such as enums. This enables capturing the possible inner value of an Option in a match arm with concise syntax.

```rust
fn main() {
    let perhaps_a_value = Some("to be or not be");
    match perhaps_a_value {
        Some(value) => println!("The value is {value}"),
        None => println!("There is no value"),
    }
}
```

However, one does not simply assign a variable to the Some variant of an Option because the None case would not be covered.

```rust
fn main() {
    let perhaps_destination = Some("Mordor");
    let Some(destination) = perhaps_destination;
    println!("{destination}");
}
```

There can be times when we are only interested in a single pattern and want to ignore the rest. This can make using match cumbersome since it forces us to handle each possible case. Instead, we might resort to the if statement to check whether the value is Some or None and then use the unwrap method to get the value out of the Some variant.

```rust
fn main() {
    let perhaps_a_value = Some("to be or not to be");
    if perhaps_a_value.is_some() {
        let value = perhaps_a_value.unwrap();
        println!("The value is {value}");
    }
}
```

To make things easier, Rust provides the if let expression. Using the if let syntax we can refactor the code into.

```rust
fn main() {
    let perhaps_a_value = Some("to be or not to be");
    if let Some(value) = perhaps_a_value {
        println!("The value is {value}");
    }
}
```

The if let syntax takes the form if let $pattern = $expression. If the lefthand pattern matches the righthand expression, Rust can safely unwrap the expression some_value into value. The same logic works for all enums in Rust.

```rust
fn main() {
    let some_text = Some("text");

    if let None = some_text {
        println!("The value is None"); // no match, no execution
    }

    if let Some("other") = some_text {
        println!("The value is other"); // no match, no execution
    }

    if let Some("text") = some_text {
        println!("The value is text"); // match, therefore execution
    }
    else if let Some(value) = some_text {
        println!("The value is {value}"); // match on previous branch, no execution
    }
}
```

**Note that if let requires the lefthand and righthand sides to be the same type.**

```rust
fn main() {
    let some_money = "💰";
    if let Some(money) = some_money { // error: mismatched types
        println!("We got {money}!");
    }
}
```


### Recoverable errors

We have already been introduced to the Option type that is used to convey null values and resulting errors. Another error handling type in Rust is the Result type, which is encountered often in typical Rust code — Rust does not have a try-catch clause that is common in imperative languages, instead it has the Result type for handling recoverable (non-crashing) errors.

For example, trying to parse a string into a number with the parse method returns a Result type.

```rust
fn main() {
    // The compiler needs to know the type for parsing
    // the wildcard _ can be used for the type when the compiler is able to infer it.
    let number: Result<i32, _> = "20".parse(); 
    println!("{number:?}");
}
```

#### The Result type

The Result type, much like Option with its variants Some and None, is an enum with two variants, Ok and Err. Unlike None, Err wraps a value which indicates some kind of an error. The compiler needs to always know the types of both Ok and Err variants.

```rust
fn main() {
      let result: Result<&str, ()> = Ok("🦀");
      let error: Result<(), &str> = Err("💥");
      println!("{result:?}, {error:?}");
  }
```

Working with Results is similar to working with Options. We can use multiple methods to work with Results, such as unwrap (may panic), except (may panic), unwrap_or, is_ok, is_err, etc. Here, as well as for other types, the exhaustive list can be found in Rust's documentation.

```rust
fn main() {
    let result: Result<&str, ()> = Ok("🦀"); // The error type is unit, needs explicit type for compiler
    let error = Err("💥"); // No explicit type, both ok and err variant types can inferred from usage
 
    println!("{}, {}", result.unwrap(), error.unwrap_or("🦀"));

    if result.is_ok() {
        println!("The result seems ok");
    }
    if error.is_err() {
        println!("The result seems not ok");
        error.expect("💥💥");
    }
}
```

As with Option variants, we can use the match expression to pattern match the Ok and Err variants and destructure the values inside. Likewise, the if let expression can be used to handle the Ok and Err variants.

```rust
fn main() {
    let mut result: Result<(), String> = Ok(());
    match result {
        Ok(value) => println!("The value is {value:?}"),
        Err(error) => println!("The error is {error}"),
    }

    result = Err("💥".to_string());

    if let Ok(value) = result {
        println!("The value is {value:?}");
    }
    else if let Err(error) = result {
        println!("The error is {error}");
    }
}
```

Returning a Result from a function requires both Ok and Err to be explicitly defined. Below is an example in a program that mimics (poorly) a general artificial intelligence.

```rust
fn agi9000(question: &str) -> Result<String, String> {
    match question {
        "what is the meaning of life?" => Ok("42".to_string()),
        _ if question.ends_with("?") => Ok("probably yes".to_string()),
        _ => Err("this is not a question!".to_string()),
    }
}

fn ask_computer(question: &str) {
    println!("Asking the computer: {question}");
    let answer = agi9000(question);
    match answer {
        Ok(answer) => println!("Computer's answer: {answer}"),
        Err(message) => println!("ERROR: {message}"),
    };
}

fn main() {
    ask_computer("what is the meaning of life?");
    ask_computer("can I have a pet dragon?");
    ask_computer("all your codebase are belong to us.");
}
```

