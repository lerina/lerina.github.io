# FiT/Code

4 phases  
- Intention
- Spike
- Test
- Code

3 turnings  
- Understand
- Implement
- Complition

Lets explore FiT/Code, in particular the notion of three turnings that are to be applied to each of the four phases. 

The basic idea behind these three turnings is that each of the four phaze requires first of all to be understood, which is the first turning; this understanding then needs to be followed by some implementation, the second turning; and this implementation has to be carried to its successful completion, the third turning.

### Testable code

----- > Paraphrase

virtually all code is testable. As long as you can control the inputs and observe the outputs, you can test any piece of software. By testable code, we mean code that is easy to test in an automated fashion at multiple levels of abstraction. This means that writing black-box systems tests is simple, that writing unit tests (for classes, methods, or other units of code) is not unreasonably complex, and that more advanced testing such as performance and security testing are possible.

Being able to write tests for a particular piece of code is an integral part of the code being well-written.

code with no return value,calling other code that are non determinstic will be difficult to test in isolation, and thus very difficult to unit test.

The two key concepts to keep in mind when writing code that is testable at the unit test level are:

 1. Ensure code is segmented
 2. Ensure that events are repeatable
 3. Test hwat goes in  and what comes out. Only. You are testing the what not the how. (_ lerina)

You can make a test repeatable by ensuring that all of the values that it depends on are able to be replicated.

The execution flow should only depends on local variables and/or passed in.

In order to test effectively, code should be segmented, and written in such a way that ensuring that the exact same code with the exact same values can be run multiple times. If this is the case, then the exact output should occur each time, and tests will not randomly fail.

### Dependency Injection

As we’ve seen, having dependencies hard-coded into your methods can make them difficult to test.

Dependency injection means passing the dependencies in as parameters to a method, as opposed to having hard-coded references to them. This will make it much easier to pass in test doubles or mocks.

RUST DUCK OTHER HERE

Note that you are passing in the Duck and the Otter in the constructor, thus moving the responsibility for creating them outside of the Pond class. Because of this, you don’t have to send in actual ducks and otters. Instead you can send in mocks, which can then verify that the say() method was called with the parameter “Hi!”. Dependency injection will allow you to test your methods much more easily than having dependencies automatically created.

### Unit vs Integration testing

#### Unit testing 

Unit tests focus on a single part of a whole application in total isolation, usually, a single class or function. Ideally, the tested component is free of side effects so it is as easy to isolate and test as possible.
Since these tests should be side-effect-free, you will want to run them directly without any other system’s involvement. Ideally, this includes no dependencies on the underlying operating system, such as file system access or network capabilities. In practice, some dependencies may exist. Other dependencies can be swapped out to allow for testing in isolation. This process is called mocking.

#### Integration testing

Integration testing is a software testing technique that focuses on verifying the interactions and data exchange between different components or modules of a software application. The goal of integration testing is to identify any problems or bugs that arise when different components are combined and interact with each other. Integration testing is typically performed after unit testing and before system testing. It helps to identify and resolve integration issues early in the development cycle, reducing the risk of more severe and costly problems later on.

Integration testing is the process of testing the interface between two software units or modules. It focuses on determining the correctness of the interface. The purpose of integration testing is to expose faults in the interaction between integrated units. Once all the modules have been unit tested, integration testing is performed.

Integration testing helps find issues that are not obvious by examining the application’s or a specific unit’s implementation. Integration testing discovers defects in the interplay of several application parts. Sometimes, these defects can be challenging to track or reproduce.

Integration testing is to test how parts of the application work together as a whole.
Integration testing considers side effects from the beginning. These side effects may even be desirable.

Integration testing can be done by picking module by module. This can be done so that there should be proper sequence to be followed.

For example, an integration test could use the connection to a database (a dependency in unit testing) to query and mutate the database as it usually would. You would need to prepare the database and read it out afterward correctly. Avoid “mocks away” these external resources the way mocking is used in unit tests. This results in obscuring the failures caused by APIs beyond their control.


### TDD
The first, most important step though is to just commit to actually doing TDD as you work. Everything else follows from that.

Every expected behaviour of a given module should have a test; therefore if one writes minimal code to achieve green the onus is on the tests to prove the behaviours wrong.

What is the smallest amount of code one might write for the test to pass?

It is almost as though you are two programmers fighting one another. The tester is attempting to create the production code they want and the coder is just attempting to get green.

Write Atomic Tests
Tests should be atomic, focusing on specific behaviors and functionalities. Keep them short, enhancing readability and maintainability and easing the debugging process. Atomic tests pinpoint issues more precisely, facilitating quicker resolutions.

Mindset:
In TDD, start by writing a test that calls my function and the output should match the specifications. To write this test, there is no need to consider any sort of implementation; the function is a black box. Focus on understanding the most important aspect: the expected output

https://jeremydmiller.com/2022/10/04/real-life-tdd-example/

https://www.jamesshore.com/v2/projects/lunch-and-learn

https://learnitmyway.com/tdd-example/

How to take a vague idea and turn it into tests

Test-driven development is an iterative process, meaning you work in small repeating steps. Yes, we want to make a good educated guess, but it doesn’t have to be exactly right. Don’t get stuck thinking of some tiny detail, because in software, things always change. 
One of the great things about TDD is that it makes change easier, so if we don’t get this 100% right on the first try, we’ll just do it again. And that’s exactly how it should be!

Step 1: Decide the inputs and outputs
We start this process from a high level. We don’t care about the implementation just yet.

We have the goal: X
 
 To get to that goal, we usually need some inputs… and then, we get some output based on them.

When you normally start writing code to get to some goal, you’d probably start with a function. You’d probably think of what data the function needs to work, and what kind of results it will give back. 
We can start this process by doing exactly that – we just won’t write any code for it yet.

So how would this work?
The input is easy: A

The output is also easy:  C

Step 2: Choose function signature

Now that we know what data goes in and what comes out, we need to choose the function signature – that is, what parameters the function takes, and whether it returns something.

This step is again similar to how you would approach writing code without TDD. Before you can write any code for a function, you need to decide what its parameters and return values are.

First, the parameters. 
What does our function need to work?
What about the return value? 

At this point, we can now decide what calling the function would look like in code.

Step 3: Decide on one tiny aspect of the functionality
We now know the goal, the data involved and the function signature.

In a non-TDD workflow, you’d jump into writing code for the function now. You might already have some ideas on how this would work – we need to check for this, we need to check for that, the return value is affected by X…

This is where most people run into trouble with TDD. Your head is filled with all these ideas on how to write the function… but you’re not sure of exactly how to lay out the code until you start writing it.
Instead of thinking of all the choices… let’s focus on one tiny thing only.
What is the simplest possible behavior that we need to get a tiny bit closer to our goal?
A common problem is to try and tackle a chunk of behavior that’s really big.

So what’s the simplest possible step we can take to make this function be closer to the ultimate goal

What would be the very first line (or two) of code you would write if you built this function without TDD?

What is the smallest amount of code we can add to bring the function closer to working
 
Return an empty string or None?

The output should always be false when the output is "empty".

Step 4: Implement test

And just like that, we’ve arrived into implementing the test. I hope that was easier than you expected :)
Notice how all of the previous steps were actually similar to writing code without TDD?
The main difference is that instead of focusing on implementing the function, we’re focusing on how the function would be called, and what happens as a result. That is – we’re thinking about how the function behaves under some conditions.
How the function behaves is what we want to test. Once you start testing behavior under some conditions (such as certain parameters, time of day, whatever), testing becomes a lot easier, because we can look at behavior from the outside. We don’t need to know the implementation if we’re just choosing behavior.

Notice that we easily wrote that without knowing what the exact lines of code in the function are going to be. We decided that given an empty string as a parameter, the result should be false. One simple behavior, which easily translated into a test.

Step 5: Implement code

Very self explanatory. We’ll just add the smallest amount of code that makes the test pass.

Then all we do is just repeat this: We’ll go back to step 3, and choose the next tiny step to take. Step 4, add test. Step 5, implement. Repeat.

If you keep advancing in small steps like this, TDD suddenly becomes a lot easier. 
Yes – you might end up with several tests for a fairly small amount of code, but that’s not a bad thing. TDD helps you in this way to reduce the amount of useless code you might otherwise write, because every line of code you add is verified by a test.

Between step 3 to 5;
Remember – if you implement some tests and code only to later find out it has to work differently, that’s fine! Go ahead and redo it – We don’t need perfection on the first try, seeking it only gets you stuck. This isn’t just a TDD thing either: you’ll probably need to redo and refactor parts of your code anyway, TDD simply makes it safer because you have tests that verify your code doesn’t break as a result of changing it.



----
TSARA BE
https://codeutopia.net/blog/2016/10/10/5-step-method-to-make-test-driven-development-and-unit-testing-easy/


Contex:
Trigger:
Intent:
Actual:

acronym:
another
creative
restriction
over
nurturing
your
mind

Acronyms stunt growth,fix 
mindset and turn helpfull idea into a inflecsible dogma
----- <

<aside>
Daksh (Deepak K Gupta) [Cognitive Programmer](https://www.youtube.com/@Cognitive-Programmer)
[codesports.ai](https://www.codesports.ai/)

The <span style="color:blue; font-weight: bold;">blue</span> stage, before <span style="color:red; font-weight: bold;">Red</span>. The *pseudo-code*.
</aside>

------- > PAraphrase

### Problems With Unit Testing

Just using the assertions and testing code, there’s no way to check, for example, that a particular string will be printed out, or that a window will appear, or that another method is called…
all very important things. After all, if all methods did was return different values with different input, never displaying them to the user or interacting with the environment in any way, we’d have no way of knowing what was happening with our programs.

Any behavior aside from returning a value is called a  side effect. 

Displaying a window, printing some text, connecting to another computer over the network—all of these are, from a terminological perspective, side effects of computation. Even setting a variable or writing data to disk are side effects. 

Functions and methods without side effects that only receive input from parameters are called pure. 
Pure functions will always return the same result given the same input values, and may be called an infinite number of times without modifying any other aspects of the system. 

Functions and methods that have side effects, or that may present different results based on something other than values passed in as parameters, are impure.

An example of a pure function would be a mathematical function, such as the square root function.

An example of an impure function would be printing out statistics from global variables, or any method which outputs something to the console or screen, or depends upon variables that are not specifically passed in. 

In general, if you see a method with a void return, it’s probably impure—a pure function with a void return type would be absolutely useless, since the returned value is its only way of communicating with the rest of the program.

Pure functions are usually easier to test, because passing in the same values will always return the same value, and it’s easy to test for input and output with standard unit test procedures. 

Impure functions are more difficult, since you may not have a return value to assert against. Additionally, they may depend upon or modify parts of the code outside of this particular method.

This does not mean that impure functions are bad! As we’ve seen, they’re absolutely necessary if you want to do anything other than make your processor warm. 

After all, printing anything to the screen is technically a side effect. 

However, by keeping as many functions pure as possible, and limiting impure functions to certain areas, you will make testing the system much easier.  
You can think of this process as “quarantining” the impure functions, so that you know where difficulties in testing might lie.

#### Test Doubles
A unit test should be a localized test; that is, it should check the particular method or function under test, and not worry about other aspects of the system. If there is a test failure, we want to make sure that the failure is due to the code in that particular method, not something else which that method relies upon.

How can we test  code when the code it depends upon doesn’t work?

test doubles. Test doubles are “fake” objects which you can use in your tests to “stand in” for other objects in the codebase. This has numerous benefits aside from hiding pieces of the codebase that don’t work. Test doubles also allow you to localize the source of errors.

You create a “fake” object instead of passing in an actual instantiation. You isolate the foreign object.

Whenever you use doubles, however, you are also dependent upon your assumptions of how the code you depend upon is supposed to work in the actual program.

Doubles can also be used to speed up the execution of tests. Think of a Database object which writes information out to a database and returns the status code. Under ordinary circumstances, the program needs to write out to disk and perhaps even access the network.

Test doubles should be used, as often as possible, when the code that you are unit testing uses a different class than the class under test. 

Sometimes it will be difficult to do. In order to minimize issues, you should pass in the object as a parameter whenever possible, as opposed to relying on member variables or global variables. 

Even worse, and more common, are methods that generate and use objects entirely internally. It is often not possible to use test doubles for these methods.

#### Stubs

If doubles are fake objects, stubs are fake methods.

REM: you shouldn’t call methods on a double object unless you have stubbed them. The whole reason for making a test double is so that you have an object that you have specified, instead of relying on external definitions.

#### Mocks and Verification

Dealing with no return value and thus nothing on which to assert.

The only way to test these methods is to ensure that
the method on the object was called.
We can do this using a special kind of test double called a mock. 
A mock object will allow you to assert that a particular method was called on the mocked object.
instead of asserting that a particular value is returned (since no value is ever returned), you instead can make a “meta-assertion” that
the method is called.
This is called verification since you are verifying that a method has been called. 
Note that this kind of verification has nothing to do with the kind of verification that means “checking that the software is right”. They are two different concepts that happen to share the same word.

#### Fakes
Sometimes, you need to want to have a test which depends on an object, and will require complex or non-performant behavior. In this case, you can use a fake. A fake is a special kind of test double which acts similarly to the regular object. However, it is written to be a part of the test, meaning that it runs faster and simpler.

For example, you may remove any parts of the code which write to disk (always a slowdown when writing tests). You may have it perform a simpler calculation than a very time-consuming one. You may reduce that object’s dependencies on other objects.
Fakes require more work to create than a simple test double, since they are a “lite” version of an object instead of simply specifying its external behavior. 

However, this allows you to perform more intricate tests than the relatively simple ones possible with test doubles.

The fake version will be a “stripped-down” version of the target but will keep the general behavior.

Unlike with a traditional test double or mock, we don’t have to specify what the behavior should be externally. The class itself will determine the (simplified) behavior.

### Testing System Output

Console output is a very common item to check for, but testing for it is non-intuitive.

TODO: my solution for now.
check why people use bytestreams

### Example: Building FizzBuzz via TDD.


------ <
