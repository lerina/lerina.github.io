<style>  
body {line-height: 1.2;}
pre {line-height: 90%;}
</style>
<main>

## Starting TDD: 

### TDD in Rust

Have a look at [Testing ](https://rust-cli.github.io/book/tutorial/testing.html) from the Command Line Applications in Rust book.

Testing the small units that you build your complete application from, these are called “unit tests”.

#### Making your code testable

To figure out what we should test, see what your program features are.
Write unit tests for what the program is supposed to do, 
to ensure that your most important piece of logic works.
Write the tests in a way that is not dependent on any of the setup code you have around it.

```rust
//main.rs

fn main() {
    // ...
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
```
Code in the main function can’t easily call it. 
This is easily fixed by moving these piece of code into functions.

```rust

    fn find_matches(content: &str, pattern: &str) {
        for line in content.lines() {
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
    }

```
Then you can call these functions in your test, and assert what its output are.

Functions that prints directly to stdout, i.e., the terminal are a bugger.
(It could also be a sign that you have written a function that is firmly integrated in the context it is used in.)

```rust
#[test]
fn find_a_match() {
    find_matches("lorem ipsum\ndolor sit amet", "lorem");
    assert_eq!( // uhhhh
```

How can we make this testable? 
We’ll need to capture the output somehow. 
Rust’s standard library has some neat abstractions for dealing with I/O (input/output) 
and we’ll make use of one called `std::io::Write`. 
This is a trait that abstracts over things we can write to, which includes strings but also `stdout`.

The behavior that we abstract over is “write to it”. Examples for the types that implement (“impl”) it include: The terminal’s standard output, files, a buffer in memory, or TCP network connections. 

With that knowledge, let’s change our function to accept a third parameter. It should be of any type that implements Write. This way, we can then supply a simple string in our tests and make assertions on it. Here is how we can write this version of find_matches:

```rust
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
```

The new parameter is mut writer, i.e., a mutable thing we call “writer”. Its type is impl std::io::Write, which you can read as “a placeholder for any type that implements the Write trait”. Also note how we replaced the println!(…) we used earlier with writeln!(writer, …). println! works the same as writeln! but always uses standard output.

Now we can test for the output:

```rust
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
```

Note:  
Since stdout expects bytes (not strings), we use std::io::Write instead of std::fmt::Write. As a result, we give an empty vector as “writer” in our tests (its type will be inferred to Vec<u8>), in the assert_eq! we use a b"foo". (The b prefix makes this a byte string literal so its type is going to be &[u8] instead of &str).


To now use this in our application code, we have to change the call to `find_matches` in main 
by adding `&mut std::io::stdout()` as the third parameter. 
Here’s an example of a main function that builds on what we’ve seen in the previous chapters and uses our extracted find_matches function:

```rust
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
```

Note:  
We could also make this function return a String, but that would change its behavior. Instead of writing to the terminal directly, it would then collect everything into a string, and dump all the results in one go at the end.


Exercise for the reader:  
`writeln!` returns an `io::Result` because writing can fail, for example when the buffer is full and cannot be expanded. Add error handling to find_matches.

We’ve just seen how to make this piece of code easily testable. We have

- identified one of the core pieces of our application,
- put it into its own function,
- and made it more flexible.

Even though the goal was to make it testable, the result we ended up with is actually a very idiomatic and reusable piece of Rust code. That’s awesome!



### Arrange-Act-Assert

Unit tests are nothing mysterious. They’re just code, executable code written in the same language that you write your application in. Each unit test forms the first use of the code you want to write. It calls the code just as it will be called in the real application. The test executes that code, captures all the outputs that we care about, and checks that they are what we expected them to be. Because the test uses our code in the exact same way that the real application will, we get instant feedback on how easy or difficult our code is to use. This might sound obvious, and it is, but it is a powerful tool to help us write clean and correct code.

- Arrange all necessary preconditions and inputs.
- Act on the object or method under test. AKA Call the object.
- Assert that the expected results have occurred. 

### Basic TTD rules:

- Write only enough test code for the test to fail
- Only write production code if you have a failing test
- Write only enough production code to pass the test

There are 5 steps in the TDD flow:

1. Read, understand, and process the feature or bug request. One by One.
2. Translate the requirement by writing a unit test. Run the unit test and let it fail as no code is implemented yet.
3. Write and implement the code that fulfills the requirement, one edit at a time. Run all tests and they should pass, if not repeat this step.
4. Clean up your code by refactoring.
5. Rinse, lather and repeat.

Note:

A unit test is simply a test that covers a small portion of logic, like an algorithm, for example. Unit tests should be deterministic. By “deterministic” we mean that unit tests should never have side-effects like calls to external APIs that deliver random or changing data. Instead, you’d use mock data in place of data that could potentially change over time.

How to write unit tests that fail

There are a couple different ways to write unit tests that fail.

- Write a test that references a function in the code that doesn’t exist yet. Proceed with the reverse picasso system. start with the bare minimum and gradually flesh-it-out.

- Alter the assert statement to make it fail. An assert statement says what value the code being tested is expected to return; this kind of statement is a key aspect of a unit test. The assert statement should reflect the feature or bug fix request.

### Working backward from outcomes

One thing we notice right away is just how unimportant the actual code that makes this test pass is. Everything in this test is about defining the expectations of that code. We are setting boundaries around why our code is useful and what we expect it to do. We are not constraining how it does it in any way. We are taking an outside-in view of code. Any implementation that makes our test pass is acceptable.

### the FIRST principles

These are a set of five principles that make tests more effective:

- Fast
- Isolated
- Repeatable
- Self-verifying
- Timely


- Unit tests need to run be fast, under 2 seconds, ideally milliseconds.
- Tests need to be isolated from one another. One test must not depend 
on another test having been run before it.
- Repeatable tests are vital to TDD. Whenever we run a test with the same production code, that test must always return the same pass or fail result. In this regard, three popular sources of misery are tests involving the database, tests against time, and tests through the user interface. Some techniques to handle these situations in are Test Doubles –Stubs and Mocks.
- Self-verifying means automated executable code to run and check whether the outputs are as expected.
- Timely tests are tests written at just the right time to be most useful. The ideal time to write a test is just before writing the code that makes that test pass. The design feedback is half of the point of doing TDD. 
Anything else is BIG upfront design!

- Try to write one assertion per test
Tests provide the most useful feedback when they are short and specific.
Use one assert per test; This prevents us from tackling too much in one test. This focuses on the error messages we get during test failures and helps us control the complexity of our code. It forces us to break things down a little further.

- The scope of a unit test
What does a unit means in a unit test? The unit refers to the test isolation itself – each test can be considered a standalone unit. As a result, the size of the code under test can vary a lot, as long as that test can run in isolation.

- Catch common errors

* Off-by-one errors
* Inverted conditional logic
* Missing conditions
* Uninitialized data
* The wrong algorithm
* Broken equality checks

- Verify error handling logic

### Principle of writing unit tests 

These principles give us maximum flexibility when implementing our methods.

- Only testing public methods
TDD is all about testing the behaviors of components, not their implementations.
We focus on what a component does – not on the how it does it.

Inside a test, this appears as calling public methods or functions on public classes and packages. The public methods are the behaviors we choose to expose to the wider application. Any private data or supporting code in classes, methods, or functions remain hidden.

A common mistake that developers make when learning TDD is that they make things public just to simplify testing. Resist the temptation. A typical mistake here is to take a private data field and expose it for testing using a public getter method. This weakens the encapsulation of that class. It is now more likely that the getter will be misused. Future developers may add methods to other classes that really belong in this one. The design of our production code is important. Fortunately, there is a simple way of preserving encapsulation without compromising testing.

- KISS principle: Keep it Simple and/or Small
Use the simplest code that could possibly work: Using the simplest code is important. There can be a temptation to use over-engineered algorithms, or perhaps use the latest language feature just for an excuse to use it. Resist this temptation. At this stage, our goal is to get the test to pass and nothing more.

- YAGNI principle: You ain’t gonna need it, until some fact proves otherwise.
Don’t overthink the implementation details: We don’t need to overthink this. We don’t need to write the perfect code on our first attempt. We can write a single line, a method, several methods, or entirely new classes. We will improve this code in the next step. Just remember to make the test pass and not go beyond what this test is covering in terms of functionality.

### Learning from our tests
Our tests are a rich source of feedback on our design. As we make decisions, we write them as test code. Seeing this code – the first usage of our production code – brings into sharp focus how good our proposed design is. When our design isn’t good, the AAA sections of our test will reveal those design issues as code smells in the test.


- A messy Arrange step
If the code in our Arrange step is messy, our object may be difficult to create and configure. It may need too many parameters in a constructor or too many optional parameters left as null in the test. It may be that the object needs too many dependencies injected, indicating that it has too many responsibilities or it might need too many primitive data parameters to pass in a lot of configuration items. These are signals that the way we create our object might benefit from a redesign.

- A messy Act step
Calling the main part of the code in the Act step is usually straightforward but it can reveal some basic design errors. For example, we might have unclear parameters that we pass in, signatures such as a list of Boolean or String objects. It is very hard to know what each one means. We could redesign this by wrapping those difficult parameters in an easy-to-understand new class, called a configuration object. Another possible problem is if the Act step requires multiple calls to be made in a specific order. That is error-prone. It is easy to call them in the wrong order or forget one of the calls. We could redesign to use a single method that wraps all of this detail.

- A messy Assert step
The Assert step will reveal whether the results of our code are difficult to use. Problem areas might include having to call accessors in a specific order or perhaps returning some conventional code smells, such as an array of results where every index has a different meaning. We can redesign to use safer constructs in either case.


### The refactor phase


*“In anything at all, perfection is finally attained not when there is no longer anything to add, but when there is no longer anything to take away, <!-- when a body has been stripped down to its nakedness -->...”*  
― Antoine de Saint-Exupéry 

This is the phase where we go into software engineering mode. We have some working, simple code with a test that passes. Now is the time to refine that into clean code – meaning code that will be easy to read later. With the confidence that a passing test provides, we are free to apply any valid refactoring technique to our code. Some examples of refactoring techniques we can use during this phase include the following:

- Extracting a method to remove duplicated code
- Renaming a method to express what it does better
- Renaming a variable to express what it contains better
- Splitting a long method into several smaller ones
- Extracting a smaller code block, struct or class
- Combining a long parameter list into its own struct or class
...

All these techniques have one goal: to make our code easier to understand.

### Limitations of unit tests
One very important idea is that an automated test can only prove the presence of a defect, not the absence. What this means is that if we think of a boundary condition, write a test for that, and the test fails, we know we have a defect in our logic. However, if all our tests pass, that does not and cannot mean our code is free of defects. It only means that our code is free of all the defects that we have thought to test for.

- Code coverage – an often-meaningless metric
Code coverage is a measure of how many lines of code have been executed in a given run. It is measured by instrumenting the code and this is something that a code coverage tool will do for us.










</main>

SOURCE:  
These are the pages for a one-semester
course at 67% speed on imperative and object-oriented programming at the
[department of Information Technology](http://it.uu.se/){target="_blank"} at [Uppsala
University](http://www.uu.se/){target="_blank"}, ran by [Tobias
Wrigstad](http://wrigstad.com/){target="_blank"}.

