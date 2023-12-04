<style>  
body {line-height: 1.2;}
pre {line-height: 90%;}
</style>
<!-- 

Fraction Slash ⁄
unicode: U+2044 ⁄  ctrl+shift U 2044 <enter>
html: &#8260;  ⁄
css: \2044   ⁄

-->
# FiT⁄Code

Whether agile or lean or old-school one should strive for  FiT/Code,
and get into the habit of Iterative, incremental software development with 
<!-- Feedback informed Test/Code -->
**F**eedback **i**nformed **T**est/**C**ode 


**todo!()**: move prereq to 80's style in DM Etter section. ------->
Break Down the Problem into chunks that can be solved more readily.

For non-trivial projects these additional prelimineries maybe necessary.
 
- Descriptive Scope of Work.

- Defines Features for the End User.
User requirements cover the different goals your users can achieve using the product and are commonly documented in the form of user stories, use cases, and scenarios.

- Functional Requirements.
Functional requirements are product features that developers must implement to enable the users to achieve their goals. They define the basic system behavior under specific conditions.

<!--
Functional requirements examples

Functional requirements need to be clear, simple, and unambiguous. Here are some examples of well-written functional requirements:

    The system must send a confirmation email whenever an order is placed.

    The system must allow blog visitors to sign up for the newsletter by leaving their email.

    The system must allow users to verify their accounts using their phone number.

Contrary to a popular misconception, functional requirements are not analogous to user stories, but stories can be a useful tool for deriving requirements with the user in mind. For example:

User story: As an existing user, I want to be able to log into my account.

    Functional requirements:

        The system must allow users to log into their account by entering their email and password.

        The system must allow users to log in with their Google accounts.

        The system must allow users to reset their password by clicking on "I forgot my password" and receiving a link to their verified email address.


Every functional requirement typically has a set of related non-functional requirements, for example:

    Functional requirement: "The system must allow the user to submit feedback through a contact form in the app."

    Non-functional requirement: "When the submit button is pressed, the confirmation screen must load within 2 seconds."



-->
... < ---------


This also helps to increase understanding of issues and makes them easier to tackle.

1. First feedback: From Requirement to test  
Can the developer write a test for the selected requirement? If not requirements are not clear and/or understood. 
What is the behavioral intent?

The `T` in FiTCode is TDD. In Tdd test are actually the *requirements as tests*. 
This is different from *unit tests* which is basically function testing.

2. Second feedback: Failing test first  
Is the failing test succint? Have you encoded the behavioral expectation within your assertion. Does the error indicate the most proximate missing feature?

Start by writing a test that calls a function and the output should match the specifications. 
To write this test, you don't need to consider any sort of implementation; 
the function is a black box. Focus on understanding the most important aspect: the expected output.

3. Third feedback: Code Interface  
Can the test easily use the code? Testable code is a must. Take some time to think of function signature and design of the interface to the code.

Focus on understanding the 2nd most important aspect: the intended inputs.

We build the code from the inside-out iteratively:

- Core interface
- Calculations and branches
- Loops and generalization
- Special cases and error handling



4. Fourth feedback: The right code works write  
Does the test pass? The code that makes a test pass is a response to a particular requirement. YAGNI is built-in.
It does the job and thats all.

5. Fifth feedback: DRY, SOLID, put best practices acronyms here  
Is it time for a refactor session? FiT code is readable, clean, lean. 




### From Requirement to test  

The answers to the "blank page"; Where to start? 
You start with the requirements. Select a requirement.

> Can the developer write a test for the selected requirement? 

If not requirements are not clear and/or understood. 
What is the behavioral intent? Understanding the requirements is the responsability of the developer. 
What do we want? What do we have? How do we go from what we have to what we want.

Invest time in data design, domain design, and the interaction between entities.

### Failing test first  

Is the failing test succint? Have you encoded the behavioral expectation within your assertion. Does the error indicate the most proximate missing feature?

Focus is shifted to how can I test this rather than how can I write this.

### Code Interface  

Can the test easily use the code? Testable code is a must. Take some time to think of function signature and design of the interface to the code. Design code for testability means paying attention to inputs and output.
Dependency injection is the norm.

#### Function Design Recipe

1. **L**ist key usage (docstrings, usage with expected output)
2. **E**xpress Expectations in one line (Type contract for weakly typed languages)
3. **R**eveal Header signature (function/method signature: name, param & return type)
4. **I**nsert Description (purpose, help text)
5. **N**urture Neat&Nimble code (code body)
6. **A**test Demo usage passes (run doctest and/or unit test and/or dev test if TDD)


###  The right code works write  

Does the test pass? The code that makes a test pass is a response to a particular requirement. YAGNI is built-in.
It does the job and thats all.

### DRY, SOLID, put best practices acronyms here  

Is it time for a refactor session? FiT code is readable, clean, lean. 


- Are the tests covering all of the functionality expected?
- Are the tests descriptive enough to allow others to understand when there are failures?
- Are the tests independent of each other?
- Are there any places in the code or tests where duplication can be eliminated?
- Can something in the test or code be written more simply, clearly or efficiently?

## The warehouse demo


### Problem Statement: Here is the COP (Challenge, Opportunity, Problem).

> The simple warehouse program is an information system for managing the
> contents of a storage facility for different kinds of wares. Every ware
> in the warehouse has a name, a description, a storage location, a
> pricetag, information about the number of items of the particular ware
> stored in the warehouse, and the cost of each item. A storage location
> is a "section" (a single letter a-z), plus a shelf (an integer).

### Expected operations

The simple warehouse program manages a database of wares and supports
the following operations:

-   Adding a ware
-   Removing a ware
-   Editing the information about a ware
-   Printing the information for a specific ware
-   Listing all wares in the database
-   Undoing the last action
-   Quitting the program


When **starting** the program, the user is presented with the main menu with
the above alternatives and picks a choice by entering the first letter
of each alternative, i.e., `A`, `R`, `E`, `P`, `L`, `U`, or `Q`.

When choosing to **quit**, the user is prompted with a yes/no question: do
you really want to quit? If the answer is `Y`, the program exits. If the
answer is `N`, the program displays the main menu again. If the answer
is anything other than `Y` or `N`, an error message is printed and the
question repeated.

When choosing to **add** a ware, the user is prompted with five consecutive
dialogues asking her to enter the required information for a ware:

-   Name
-   Description
-   Storage location
-   Price
-   Amount

The program then displays the information back to the user and asks
whether this is correct or not. The user has three options:

-   `E`dit the information
-   `S`ave the information
-   `A`bort

The choice is indicated by entering the first character of each option. 
The *first alternative* allows the user to re-enter one piece of
the required information for a ware after which the editer ware is
displayed again with the same question as above.  
The *save alternative* saves the information to the database and then returns to the main menu.  
The *abort alternative* returns to the main menu without saving.

The rest of the actions on the main menu should be fairly self-explanatory.


------------------------------------------------------------------------

## Tips:

* Read more than you write: 

-   When reading code, it's not crucial to understand the whole project 
    or the ins and outs of the entire design. Read the code. 
    Look at the comments, see what the authors are doing, and how they went about it.
-   Develop an aesthetic appreciation for code. 
    Read the source code of famous open-source tools and frameworks. 
    Train yourself to know what great code looks like and what bad code “smells” like.
-   Get used to Basic TDD

* Guidelines for names

- Functions/Method    – Say what it does. What is the outcome? Why would I call this?
- Variable            – Say what it contains. Why would I access this?

* Testable code

- Printing is detestable to tests. To make a function to be testable, Rather than print the item, 
have the function simply return the data we want. Now, we can test it.
In general: write code that interacts with abstract interfaces, which can be mocked for tests, rather than concrete implementations, which can't.
- Avoid Functional Complexities
Simplicity is important. Break down tasks into manageable units, focusing on one function or feature at a time.
- Tests should be atomic, focusing on specific behaviors and functionalities. Keep them short, enhancing readability and maintainability and easing the debugging process. Atomic tests pinpoint issues more precisely, facilitating quicker resolutions.
- Write tests to test behavior, not implementation.
- Write the Simplest Test Case first.
Begin with the basics baby steps. Write the simplest test case that reflects the expected behavior. This sets a solid foundation, allowing you to build upon it and gradually handle more complex scenarios.
- Refactor Regularly
- Test failures is Development Feedback. Analyze the failure, pinpoint the root cause, and let it inform your development decisions.
- Compiler error are Development Feedback.

---

## References:

Essential viewing: 
- [Everything You Wanted to Know About Rust Unit Testing (and then some more)](https://www.youtube.com/watch?v=_jDKeOtOiEo)


