#  SIMPLE – A Minimalistic Programming Methodology

This text is an adaptation of this wonderful page [SIMPLE: A Programming Methodology](http://wrigstad.com/ioopm18/simple.html) by Tobias Wrigstad, a Professor in Computing Science at [Uppsala University](http://www.uu.se/).
I've been trying to express what [software writers](https://www.youtube.com/watch?v=xPecMsFmEm4) do.
We are both [logician](https://en.wikipedia.org/wiki/Logic) and [poets](https://www.youtube.com/watch?v=-jRREn6ifEQ). 
We are also [iterative artists](http://fineartdrawinglca.blogspot.com/2016/07/iterative-drawing_11.html)

I replaced the term cheating used in the original text and main source of inspiration for this
text with the more widespread usage "stubbing" as described 
here [The purpose of ‘stubbing out’ a function](https://courses.engr.illinois.edu/cs225/sp2019/guides/stub-out/)

##    1 SIMPLE: A flexible structured approach to code implemention
<a href="#TOC"> ` ^-- ` </a>

In a nutshell SIMPLE dictates that you:

* Use the specification in a structured manner to do both function
design and data design.
* Work in an incremental fashion, by solving many simple problems
instead of few complex ones.
* Always have a working program that you can run and test.
* Compile continuously.
* Test continuously, preferably automated but at least manually.
* Write straight-line code first, before complicated branches and loops.
* Use tricks – stubbing and dodging – to keep programming simple, and
push problems into the future.

takeaway:

function design and data design  
incremental progress
Between each incrementation the code can run and passes its tests


### [outside-in](https://www.holdenrehg.com/blog/2018-09-22_write-better-code-outside-in)
Define the entry point for the code.  
Assume that any module, function, helper, library, etc. is available, even if the code does not exist.  
Write the ideal version of the entry point.  
Write the ideal version of any code that was assumed until you start reaching “low level” functions that perform the bulk of the logic.  
In other words sketch with one line comments the minimal viable system, and breaks it down into smaller components
Repeat until you have all functions defined.  

This completes your initial minimal design.

### Inside-out
Now actually implement the logic for the “low level” functions.  
Build up functionality one piece at a time. 
Because your have already done a top-down design with the outside-in aproache, 
it is now easier to get "really" started, following a methodical approach of identifying the data-structures 
a function or method will manipulate inorder to perform a certain behaviour.  

Coding is a combination of creativity and common engineering practices.  
There are thousands of ways to approach a problem and still come up with the same answer. 
Outside in development is a way to think about software development  
from a high level so that we can work through problems efficiently on the fly  
while still producing good, readable, testable code.  

Outside-in and inside-out terminilogy are often associated with [TDD](http://xunitpatterns.com/Philosophy%20Of%20Test%20Automation.html)
since the 90's but its as actually always been the implicit way coders develope software when you really remove all the hype "du jour".

## A Summary of SIMPLE in Ten Steps

Start with a high-level work breakdown structure
- Textual-Analysis   
 Mine your specification for data (nouns) and actions (verbs – behaviour/functions). Make simple drawings, e.g., mind-maps, to record your insights. Making things look easy is almost always good.
- Spikes
Write code to test the validity of your thinking
    …not to drive the thinking! Thinking should invariably come before coding, especially thinking about how you check that your thinking is valid.
- The one rule
Always have a working program.
In combination with stubs and temping, this usually means inserting dummy functions.
Compile after every change
- Refactor early
    Fix errors now, not later. Fix errors one by one. Fix errors in the order they were printed. Take time to actually read the compiler message so you know you are fixing the right thing.
-The one Rule again
Run the program “all the time” to spot errors
    This requires always working code – preferably in combination with automated test that don’t involve ocular inspection, etc.
- The main tool (Simplify)
Recursively break your problems up into smaller sub problems
    Only start solving problems when they start feeling easy. Make a task for each problem or sub-problem to put on the stack, take tasks from the stack in a reasonable order (preferably easiest first); when the stack is empty – you are done!
Break each task up into increments and start with the easy ones
    Generate new tasks to put on the stack as you go. If suitable, start with a straight-line version (without any if-statements). When the straight-line version works add conditionals, one by one. Start with the most basic or the most interesting cases. When you are writing a loop, do the above steps first and add the looping step last.
Whenever you run the risk of getting stuck, Stub, Temp, mock  and fake
    Don’t forget to push new tasks on the stack that undoes the subtitution, later. This records the subs, which is great.
- Temping
Use Temping to help breaking complex cases up into several less complex ones
    Don’t forget to push new tasks on the stack that undoes the dodge, later. This records the dodge, which is great.
Alternate between thinking, coding and refactoring

        thinking – not so much that you get stuck, though,
        coding – but never without first thinking about what to code, and occasionally
        refactoring – especially to address your cheats and dodges (tempTypes)

    Refactoring is good to do between larger tasks. Make it a habit to continuously go back and refactor your solutions so that recently gotten insights rub off on older code too. 
    Think, code, verify.
    CI runs the automated tests.



##    2 Running Example: The Simple Warehouse
<a href="#TOC"> ` ^-- ` </a>

The simple warehouse program is an information system for managing the
contents of a storage facility for different kinds of wares.  
Every ware in the warehouse has a name, a description, a storage location, 
a pricetag, information about the number of items of the particular ware
stored in the warehouse, and the cost of each item.  
A storage location is a “section” (a single letter a-z), plus a shelf (an integer).

The simple warehouse program manages a database of wares and supports
the following operations:

-    `A`dding a ware
-    `R`emoving a ware
-    `E`diting the information about a ware
-    `P`rinting the information for a specific ware
-    `L`isting all wares in the database
-    `U`ndoing the last action
-    `Q`uitting the program

When starting the program, the user is presented with the main menu with
the above alternatives and picks a choice by entering the first letter
of each alternative, i.e., |`A`|, |`R`|, |`E`|, |`P`|, |`L`|, |`U`|, or |`Q`|.

When choosing to quit, the user is prompted with a yes/no question: do
you really want to quit? If the answer is |Y|, the program exits. If the
answer is |N|, the program displays the main menu again. If the answer
is anything other than |Y| or |N|, an error message is printed and the
question repeated.

When choosing to add a ware, the user is prompted with five consecutive
dialogues asking her to enter the required information for a ware:

* Name
* Description
* Storage location
* Price
* Amount

The program then displays the information back to the user and asks
whether this is correct or not. The user has three options:

* Edit the information
* Save the information
* Abort

The choice is again indicated by entering the first character of each
option. The first alternative allows the user to re-enter one piece of
the required information for a ware after which the editer ware is
displayed again with the same question as above. The save alternative
saves the information to the database and then returns to the main menu.
The abort alternative returns to the main menu without saving.

The rest of the actions on the main menu should be fairly self-explanatory.


##    3 The SIMPLE Approach
<a href="#TOC"> ` ^-- ` </a>

The most fundamental principle of SIMPLE is to **always have a working program**.

By a working program is meant a program that compiles without warnings
or errors, and runs. In the beginning, the program is not going to do
anything useful, which is fine.

The SIMPLE approach is all about adding functions to the program little
by little until it suddenly is finished.

By always having a working program, it is easier to test the program,
and the most shallow bugs can often be caught by just running the
program and spotting the error. Furthermore, working with a program that
runs is generally much more humanly rewarding and avoids the scary
feeling of “not having anything” for a long while until finally the
individual pieces are connected and a program magically appears.

*I am supposed to write a program. Where do I start?*

![I am supposed to write a program. Where do I start?](GTY_janet_leigh_kab_150916_12x5_992.jpg)

This is exactly the question SIMPLE is trying to answer. Along with the
question, when am I done? We will now proceed by walking through a
number of steps, and stopping and recapping once in a while.


###      3.1 Step 1. Think a little.
<a href="#TOC"> ` ^-- ` </a>

The most important answer to this question is about where you should
*not* start: don’t start to code (yet)! Before we start to code, we
should make sure to understand something more about the program. The
goal is to make sure we have a basic understanding of what the program
is supposed to accomplish, but not yet care about how it should
accomplish it.

To get a feeling for what the program is trying to accomplish, we mine
the specification. This means re-reading it, looking for some particular
kind of information. To understand the program’s *core behaviour*, we
pay close attention to the verbs and actions. Using a pen to underline
or highlight them is not a bad idea.

For concreteness, I am copying in parts of the specification from above,
*highlighting* as I go.

The simple warehouse program is an for … letter a-z), plus a shelf
(an integer).

The simple warehouse program manages a database of wares and
supports the following operations:

1. **Adding** a ware
2. **Removing** a ware
3. **Editing** the information about a ware
4. **Printing** the information for a specific ware
5. **Listing** all wares in the database
6. **Undoing** the last action
7. **Quitting** the program

When **starting** the program, the user is **presented** with the **main menu** 
with the above alternatives and **picks a choice** by entering
the *first letter* of each alternative, i.e., |`A`|, |`R`|, |`E`|, |`P`|, |`L`|,
|`U`|, or |`Q`|.

When **choosing to quit**, the user **is prompted with a yes/no question**: …

Looking at the text above, I have a pretty good overview of what the
program is supposed to do at a high-level. Of course, this specification
is extremely simple by listing the main menu with a lot of actions, but
even if it didn’t, picking the actions probably would not be that much
harder.

Now, I take a piece of paper and write these things down. A mindmap
<http://en.wikipedia.org/wiki/Mind_map> is usually a good way to capture
hierarchical structures.

![Figure 2](img1.png)

Figure 2: Initial mind map of actions in the program

The mind-map tells me what actions that the program should support, but
so-far nothing about how they should be implemented. So far, that’s fine.

*This is what we are doing:* We are abstracting and compartmentalising.
If we tried to understand the entire system before starting to code, we
would probably never start to code, because it is very hard to know when
you truly understand something. Some people are good at convincing
themselves that they “get it”, regardless of whether it is true or not.
Others are good at convincing themselves that they don’t. The key is to
not get into a situation where this matters. At least not yet.

What we are going to do is the following: we are going to alternate
between thinking a little and coding a little. We are going to think
just enough so that we understand what is a plausible next step forward.
Then we are going to evaluate whether that was a good step or not by
coding. This is good, because coding is a good way to understand
sloppily formulated ideas
<http://web.media.mit.edu/%7Eminsky/papers/Why%20programming%20is--.html>.
By forcing us to concretise, coding challenges us to understand what it
is we understand.


###      3.2 Step 2: Code a little.
<a href="#TOC"> ` ^-- ` </a>

Following the most fundamental principle of SIMPLE, we start with the
minimal working program. If we are using C, we would probably write
something like this:

```c
/* main.c */

#include <stdio.h>

int main(void)
{
    puts("Welcome to my warehouse program!");
    return 0;
}
```

We'll follow each C code snipet 

---

with a Rust version:

```rust
// main.rs

fn main() {
   println!("Welcome to my warehouse program!");
}
```

This program is pretty much hello world, but it compiles and runs.

Now, we take the actions from the mind-map and turn them into functions.
For example, for “add”, we do:

C
```c
/* main.c */

// TODO: implement
void add_action(void)
{
    puts("Add action is not implemented yet");
}
```

---

Rust
```rust
// main.rs 

fn add_action() {
   println!("Add action is not implemented yet");
   //unimplemented!("Stopping here!");
}
```

And similar functions for all the other actions. We do this in order to
have placeholders for code that we are going to write. Again we want to
compartmentalise as much as possible: by focusing on just one small
thing at a time, each problem is going to not just seem simple, but be
simple. And that is one of the secret sauces to success. (More
ingredients are coming up later.)

/Between each function you add, compile the program./

Tip

> *Compile Early. Compile Often.*

Unless you are looking for trouble, or are writing live code in front of
an audience and are desperately trying to come across as a human
compiler, you really want to compile your code all the time.

When you are new to a programming language, make a habit to compile
after each edit. After a while, you will be able to go up to a few
lines. Then a bit more, etc.

The reason for this is to compartmentalise your errors too! If you write
100 lines of code before you try to compile, you are likely to get 50
errors. Unless you are hardened by experience, this can be an
insurmountable and cause you to give up. The nastiest compiler errors
always happen in the code you wrote more than one hour ago. Not because
you were a worse programmer then, but because it was one hour ago! You
probably don’t remember what you were thinking back then, so you have to
work hard at retracing your steps. If you try to make sure the code
compiles every two minutes, [logic dictates](https://www.theobjectivestandard.com/2013/09/spocks-illogic-the-needs-of-the-many-outweigh-the-needs-of-the-few/) that solving these bugs is way way easier. Good programmers likes easy.

Tip

> *A Note About Compiler Errors*

If for some reason you do get a whole page of compiler errors there are
four important things to do, two of which are really hard even for a
seasoned programmer:

* Don’t panic!
* Read the error messages!
* Address the errors *one by one* and *in the order they were printed
on the screen*.
* Recompile all the time, and always after addressing each error.

Compilers are usually really bad at printing readable error messages.
But you can learn how to read them. It will pay off because 90% of the
time, there will be 10 or so errors. Focus on learning those first!
(Seasoned programmers often just read the line number information, and
jump there to fix the error themselves. This is just stupid, but very
human.)

It is also quite common for compilers to become confused by errors
earlier in the code. Compilers read like Westerners, i.e., top-down
left-right. Often an error on line N cause one or more errors on lines
N+X and N+Y, and since the compiler in the general case cannot rule out
that the errors are unrelated, it prints all of them. Consequently, if
the compiler says you have 10 errors, it is more likely less than 10,
sometimes just one. The error that appears closest to your cursor on the
prompt after compiling is therefore more likely to be a “faux error”
than the first error that the compiler printed. Don’t waste time fixing
something that does not need fixing. (Seasoned programmers sometimes get
carried away and forget this too.)

Since one error often create “faux errors” later in the code, it is a
very good idea to recompile all the time while fixing compile errors. It
may be that fixing one error closes all of the remaining ones. Also, it
is not uncommon to introduce a new error while you are fixing another
one, so recompile often to make sure that the error number goes down
steadily.


###      3.3 Step 3: Think a little more.
<a href="#TOC"> ` ^-- ` </a>

We now have a working program that does absolutely nothing. Let’s start
adding some functionality to our program following the specification.

The first six action all require either that there is some data in the
program, or that we are able to add some data. We haven’t done any data
design yet, so if we want to start with these, we should that. However,
the last action, quitting, is very simple and does not do much so that
is a good starting point.

We now go back to our mind-map and extend the specification of quit.

![Figure 3](img2.png)

Figure 3: Second mindmap of the program

Re-reading the specification, we now know that triggering quit should
enter into a confirmation dialogue, and if the user replies |Y|, then we
quit, if |N| we return to the main menu, otherwise repeat the question.


###      3.4 Step 4: Code a little more.
<a href="#TOC"> ` ^-- ` </a>

To keep the program as simple as possible, we are not going to add
functionality for choosing an action in the main menu just yet. That’s
not needed, because the only thing we are going to implement is quit.
So, although it is futile, let’s extend the program so that when we run
it, it asks if we want to quit. To this end, we will add a |while| loop
that loops forever and that in each turn of the loop asks us whether we
want to quit. This will allow us to test both the yes and no answers.

```c
/* main.c */

#include <stdio.h>

int main(void)
{
    while (true)
    {
      puts("Welcome to my warehouse program!");

      if (quit_action()) break;
    }
    return 0;
}
```

---


```rust
// main.rs

fn main(){
   println!("Welcome to my warehouse program!");

   loop {
      if quit_action() { break; } // quit_action()-> bool
   }
}
```

Once this edit is done, we compile and run the program. Just to be sure
we did not break anything. But oh, it does! |quit_action()| must first
be updated to return a |bool|. We fix this and add a return false; as
the default return and also include |stdbool.h| at the top of the file.

Moving on, we now have a function |quit_action()| that looks like this:

```c
/* main.c */

// TODO: implement
bool quit_action(void)
{
    puts("Quit action is not implemented yet");
    return false;
}
```

---


```rust
//main.rs

   fn quit_action()-> bool {
      println!("Quit action is not implemented yet");

      false
   }
```


Our goal now is to implement the behaviour of the specification which
was just re-iterated above. We are going to do this in a few small
increments. (I will return in a little bit to how to come up with good
increments.)

* Print confirmation question
* Read input
* Handle |Y| case
* Handle |N| case
* Handle remaining cases

Increment 1 is easy. Just change the existing |puts()|:

```c
/* main.c */

// TODO: implement
bool quit_action(void)
{
    puts("Do you want to quit? [Y/N]");
    return false;
}
```

---


```rust
//main.rs

   fn quit_action()-> bool {
      println!("Do you want to quit? [Y/N]");

      false
   }
```


Increment 2 is much harder. It requires us to know how to read input
from a user in C. This is actually pretty complicated and there are many
opportunities for errors. Therefore, I am going to use one of the best
programming tricks ever – I am going to “*cheat*”. By stubbing, I mean
the following: *I am going to assume the existence of a function that
solves the problem for me.* This makes increment 2 really simple:

```c
/* main.c */

// TODO: implement
bool quit_action(void)
{
    puts("Do you want to quit? [Y/N]");
    char input = get_char_input(); // <---- This line was added
    return false;
}
```

---


```rust
// main.rs

   fn quit_action()-> bool {
      println!("Do you want to quit? [Y/N]");
      let input = get_char_input(); // <---- This line was added

      false
   }
```


*You really need to understand that* stubbing is one of the best tools
in the programming toolbox. You should use it all the time. The first
rule of stubbing is this:

| If you don’t know immediately how to solve a problem, 
| assume the existence of a function that solves this problem for you, 
| and call that function.


This is great because usually when you are programming, you are going
somewhere and you have a particular control flow
<http://en.wikipedia.org/wiki/Control_flow> in mind. Getting stuck
implementing one step of this flow usually means your train of thought
is interrupted and because your mind is now focused on solving the
current step, you will have a harder time continuing where you got stuck
once you are unstuck.

Note

Aside: This is actually not so different from tricks used by surrealist
authors experimenting with automatic text. There a standard trick is to
choose a word or a first letter of a word that you revert to whenever
your brain gets stuck. Here our actions are slightly more sensible. (At
least to some.)

Like we did with the empty action functions from the start, we add a
dummy function to the file called |get_char_input()| to stay true to the
ideal of always having a working program:

```c
/* main.c */

// TODO: implement
char get_char_input() // <-- return type derived from the call-site
{
    return 'Y'; // <-- a good default because of how increment 3 is specified
}
```

---

```rust
// main.rs

fn get_char_input() -> String {

    String::new("Y")
}
```



We can now compile and run our program as a kind of minimal testing.

Back to our increment 3, which is dealing with the base case of the user
inputting a |Y|:

```c
/* main.c */

// TODO: implement
bool quit_action(void)
{
    puts("Do you want to quit? [Y/N]");
    char input = get_char_input();
    if (input == 'Y') return true;
    return false;
}
```

---

```rust
// main.rs

   fn quit_action()-> bool {
      println!("Do you want to quit? [Y/N]");
      let input = get_char_input();
      
      if input == "Y" {
      	return true;
      }

      false
   }
```


Increment 4 is dealing with the case of the user inputting a |N|. Turns
out it is already handled, as the function returns |false| as a default.
This leaves us with increment 5, which is the user replying with
something other than |Y| or |N|.

The reason why I left this case until now is because this is the case
that removes some of the simplicity of the code we have been working on
so far: we must now add a loop to this function to be able to repeat the
question.

We start by wrapping the entire function inside a |while|. The exit
condition of the |while| is that input is either |Y| or |N|. Otherwise,
the specification dictates`, an error message shall be printed and the
question shall be asked again indefinitely. Here is a first attempt at
doing this:

```c
/* main.c */

bool quit_action(void)
{
    while (true)
    {
      puts("Do you want to quit? [Y/N]");
      char input = get_char_input();

      if (input == 'Y') return true;
      if (input == 'N') return false;

      puts("Only Y or N are valid answers!");
    }

return false;
}
```

---

```rust
// main.rs

fn quit_action()-> bool {
	loop {
	      println!("Do you want to quit? [Y/N]");
	      let input = get_char_input();
	      
	      if input == "Y" { return true; }
	      if input == "N" { return false; }
	}
	println!("Only Y or N are valid answers!");

	false
}
```



Note how I insert “paragraph breaks” into the code to capture what
belongs together, and to help the reader “breathe”. Asking the question
and getting the reply feels like two related concepts. The two checks of
the exit conditions are clearly also related. Thus, I group these
together by separating them with a paragraph, and also put another
paragraph before the error which is the logical conclusion of the function.

Also note that I removed the |// TODO: implement| comment since the
function is now properly implemented, modulo the cheat – the call to the
(yet) non-existent function.


###      3.5 Intermission: Developing in Increments
<a href="#TOC"> ` ^-- ` </a>

The |quit_action()| function was developed in a series of steps. The
choice of the steps was very deliberate. Here are the rules to follow:

If there is any kind of branching, start with the most basic interesting
case first
In our example, Y and N are equally basic but only Y leads to
something interesting, i.e., exits the program. The non yes or no
cases require retrying which is more complicated, so we delay those
even further. In the end, the order is Y, N, and “the rest”.
Always finish a straight-line version first
This means that any code that involves loops, you start by focusing
on getting one iteration of the loop working in the working program.
Once that is done, you can more easily wrap the working code inside
a loop. Note that by this rule, the Y case would still be
implemented before “the rest” even if Y called some other functions
to save the database etc.
Each increment should accomplish something sensible
Each increment should result in a working program.
Each increment should ideally have a testable outcome
For example, when we implemented (skipped…) increment 4, we should
also have changed the |get_char_input()| to return N to test this.
And similar for increment 5.

Now: back to the running example.


###      3.6 Step 5: Remove the Cheats
<a href="#TOC"> ` ^-- ` </a>

We have finished |quit_action()|, but mostly because we cheated. To
finish implementing this part of the program, we will now remove the
cheat by implementing |get_char_input()|. However, the reason why we
didn’t write this piece of code down immediately from the start was
because we thought it was complicated and did not know exactly how to do
it. So, how do we proceed now?

Well, we cannot magically suddenly grasp how to read user input. This
knowledge generally comes from somewhere. Let’s cover the three most
basic cases:

1. Google/Stack Exchange
2. A text book
3. Man pages


####        3.6.1 Man Pages
<a href="#TOC"> ` ^-- ` </a>

Posix compliant operating systems have a |man| command that brings up
the manual for a certain topic. For example, if you write |man printf|,
you will get the manual page for |printf| /and associated functions/.
The associated functions bit is key – often you know how to do something
related, but maybe not this particular case. For example, you may know
how to read formatted data from the terminal with |scanf|, and then you
can use man |scanf| to find out about |fscanf| that reads formatted data
from a file. The man pages are a great tool to know because they are
such quick access to information, usually at a much higher quality (but
less specific) than Stack Exchange etc.


####        3.6.2 A Text Book
<a href="#TOC"> ` ^-- ` </a>

You may have a text book on the subject. If so, great. Figuring out what
to look for might be tricky. You first have to make the connection to
I/O, and then you must go through 50 pages to find a mention of the
specific functionality you are looking for. However, if you do, a text
book usually gets you there. As long as you are doing something quite
basic.


####        3.6.3 Google / Stack Exchange
<a href="#TOC"> ` ^-- ` </a>

This is usually the easiest option. And a really good one at that. A
problem with code from Stack Exchange or random blogs is that you may
not always understand exactly what the code does, and why. You normally
end up with 40+ suggested solutions to your problem, 10 of which use
libraries that you don’t have or know how to include, another 10 that
you cannot get to compile because of reasons, leaving another 20 which
you must somehow chose between.

So how do you do that? Actually, in the same way as with the text book
or examples from man pages: you try it out.

Here is the golden rule of trying code out: *start with a new empty file
that just tests the code you’re trying to get to run.*

The reason for this is *compartmentalising*. It may be that the reason
why you are stuck on something is because there is an error in your code
somewhere that breaks some part. Pasting random code into the place
where you believe that the error is, will probably not uncover anything.
It will just cause you to go through all 20 versions and (erroneously)
conclude that none of them works.

Also, by isolating the code you are looking at in a single file, there
is a lot less disturbance. You want to be working at all times with the
smallest possible thing that can work. Remember: *good programmers like
lazy.*

If the problem is getting user input, then writing a small test program
for that is easy. To simplify copying the solution across to the “real
program” you can even name the functions right from the start:

```c
/* main.c */

#include <stdio.h>

char get_char_input()
{
    // paste code here
}

int main(void)
{
    printf("You pressed: '%c'\n", get_char_input());
    return 0;
}
```


---

```rust
// main.rs

fn get_char_input() -> String {
  // paste code here
}

fn main() {
  println!("You pressed: '{}'", get_char_input());
}
```



This program will call |get_char_input()| and print out the character
between single quotes to avoid confusion. We can now spend the next 15
minutes on getting the code that we are finding online to work in this
file, and then simply copy |get_char_input()| across to our other C file.

Tip

> *Make it Easy to Write Small Test Programs*

The easiest way to understand how something works in programming is
generally to test it out. To that end, make it really easy to start with
an empty file, compile and run it. This could be hacking your editor,
downloading a plugin to your IDE, or saving an empty file plus makefile
<http://mrbook.org/tutorials/make/> in a directory that you simply copy
to make a new “testbed” for something.

Understand that testing something out simplifies life tremendously, and
that all minutes spent on simplifying simplification simplifies even more.


###      3.7 Step 6: Think a little – Code a little (Seems familiar?)
<a href="#TOC"> ` ^-- ` </a>

Now that we are done with quitting the program, it is time to move on to
the next action. However, in order to get to the next action, we must
first extend the main loop of the program so that the user can choose
between whatever action we choose to implement next, and quitting.

Looking at the specification, the user’s input is again by reading a
character. Great that we have already implemented |get_char_input()| to
do exactly that. (If we hadn’t, we could just rely on the existing
cheat, but eventually we would have to implement it of course.)

Following the instructions on how to develop in increments, etc. and
looking at how we implemented the dialogue in |quit_action()|, we can
relatively quickly write something like this:

```c
/* main.c */

#include <stdio.h>

int main(void)
{
    bool should_quit = false;

    while (!should_quit)
    {
      puts("Main menu:");
      puts("[A]dd ware");
      // ... <--- remaining actions, or just omit them for now
      puts("[Q]uit");
      char input = get_char_input();

      // Hmmm ... code formatting?
      if (input == 'A')      { add_action(); }
      else if (input == 'Q') { should_quit = quit_action(); }
      else                   { puts("Invalid choice!"); }
    }
    return 0;
}
```

---

```rust
// main.rs

fn main() {

    loop {
      println!("Main menu:");
      println!("[A]dd ware");
      // ... <--- remaining actions, or just omit them for now
      println!("[Q]uit");
      let input: String = get_char_input();

      // Hmmm ... code formatting?
      if input == "A"      { add_action(); }
      else if input == "Q" { if quit_action() == "Y" { break; } }
      else                   { println!("Invalid choice!"); }
    }

}
```


Here, I added printing of (part of) the main menu, a call to
|get_char_input()|, and I also changed the code a little to make use of
a |should_quit()| variable to control whether the main loop should
continue or not. The reason for this is simply because code with fewer
exit points is simpler to understand.

However, looking at this code and the code for |quit_action()|, I notice
that there is a small modicum of repetition. In both cases, I have to
handle the failure mode, i.e., when the user presses something that
isn’t part of the protocol.

Now *refactoring* enters into the picture. What I want to do now is to
improve the code for |get_char_input()| so that it can handle the
failure for me. A simple way to do that is to pass both the question and
the valid alternatives to the |get_char_input()| function:

```c
/* c version */

char input = get_char_input("Do you want to quit?", "YN");
char input = get_char_input("Choose menu item!", "AREPLUQ");
```

---

```rust
// rust version

let input = get_char_input("Do you want to quit?", "YN");
let input = get_char_input("Choose menu item!", "AREPLUQ");
```

The code for this is almost given in the specification of this
assignment, so won’t be repeated here. However, the resulting code for
|main()| is nice. Here rewritten with a switch statement just for fun,
and the main menu printed in its own function:

```c
/* main.c */

#include <stdio.h>
int main(void)
{
    bool should_quit = false;
    while (!should_quit)
    {
      print_main_menu(); // <--- menu now in a function
      switch (get_char_input("Choose menu item!", "AREPLUQ"))
        {
        case 'A': add_action();                break;
        case 'R': ...                          break;
        case 'E': ...                          break;
        case 'P': ...                          break;
        case 'L': ...                          break;
        case 'U': ...                          break;
        case 'Q': should_quit = quit_action(); break;
        }
    }
    return 0;
}
```

```rust
/* main.rs */

fn main() {
    loop {
      print_main_menu(); // <--- menu now in a function
      let choice = get_char_input("Choose menu item!", "AREPLUQ");
      match choice {
        'A' => add_action(),
        'R' => ...         ,
        'E' => ...         ,
        'P' => ...         ,
        'L' => ...         ,
        'U' => ...         ,
        'Q' => { quit_action() == "Y"; break;} ,
        _ => ()
        }
    }
    
}
```


Now, we can compile and run the program again, and test that it works to
trigger |add_action()|, which will print “Add action is not implemented
yet”. Time to implement it!


###      3.8 Step 7: Think a little – Code a little (again)
<a href="#TOC"> ` ^-- ` </a>

A good candidate for this is adding a ware, because editing, removing,
listing, undoing, etc. all presuppose the existence of some data in the
database. Adding something new is often simpler than editing or
removing. This means that is a good place to start.

It is now time to go back to the specification again, and read about the
add action. We extend our mind-map accordingly:

![Figure 4](img3.png)

Figure 4: Third mindmap of the program

We have now arrived at a good place to do some *data design*. In other
programs, we might have started with that if there weren’t any functions
that did not manipulate any core data for the program. But what do we
mean by *core data*?

Unsurprisingly, perhaps, the program’s *core data* is the central data
for the program. A complicated program might have tons of different
kinds of core data, but in our example, we are more lucky. To understand
a program’s core data, we simply *mine the specification* again, this
time looking for nouns <http://en.wikipedia.org/wiki/Noun>. Let’s do
that now!

Again, for concreteness, I am copying in the specification and
highlighting as I go:

Every *ware* in the warehouse has a *name*, a *description*, a
*storage location*, a *pricetag*, information about the *number of
items* of the particular wares stored in the warehouse, and the
*cost of each item*. A storage location is a *section* (a single
letter a-z), plus a *shelf* (an integer).

The simple warehouse program manages a *database* of wares and
supports the following operations:

 1. Adding a *ware*
 2. Removing a *ware*
 3. Editing the information about a *ware*
 4. Printing the information for a specific *ware*
 5. Listing all wares in the *database*
 6. Undoing the last *action*
 7. Quitting the program

Looking at the highlighted information above, and reading the
surrounding text, a pattern emerges. There is a *database*, which stores
*wares*, that each store name, description, storage location, pricetag,
and number of items. There is also a mentioning of *action*, which makes
sense: in order to undo an action, we need to somehow record what the
program did just before, and enough information to undo it. We don’t
know exactly what this information is yet, but that’s fine. It probably
does not make sense to implement undo until there are actions to be
undone. (Read more in the aside below.)

The specification gives us a (mostly) very good idea for a data type for
*ware*. Name and description are clearly strings. Price and number of
items (which I will be calling amount from now on) are clearly integers.
The storage location is less clear, however. Rather that getting stuck
on this detail, let’s cheat and assume that there is a type
|storage_location_t| that solves the problem. Now, we can write the type
for ware:

```c
/* main.c */

struct ware
{
    char *name;
    char *description;
    storage_location_t storage_location;
    int   price;
    int   amount;
};
```

By stubbing a little, we now have a type |ware|. However, in the
interest of making progress and working in an incremental fashion, we
are also going to apply another great simplification trick: *dodging*.

Getting information from the user at all is a great first step in an
incremental development methodology. Once that is done and works, we can
start caring about making sure that the information is correct. It is
time to *dodge*.

Dodging means *temporarily* simplifying the specification. In the
current example, a good dodge is to say: for now, I am going to only
store string data in wares, and worry about integers and storage
locations later. This allows us to use a single function for reading
input from the user to complete the entire specification for adding a
ware, modulo saving it to the database.

Temporarily, we change to type of |ware| to this:

```c
/* main.c */

struct ware
{
    char *name;
    char *description;
    char *storage_location; // TODO: storage_location_t
    char *price;            // TODO: int
    char *amount;           // TODO: int
};
```

And then we can make a version of |get_char_input()|, for example
|get_string_input()|. We will probably also cheat and assume its
existence on the first pass through the code. We will also cheat and
assume the existence of a |add_to_db()| function, which does nothing
except prints out the obligatory disclaimer of it not yet being
implemented.

Here is a list of suitable steps for almost implementing |add_action()|.
Each step notably results in a program that can compile and run, but
probably does not do anything useful.

1. Read all the inputs as strings and save them into a ware object,
assuming the existance of several functions (This could quite
possibly be several increments.): a. |get_string_input()|, b.
|print_ware()| (to print a ware to the user before answering the
save, edit, abort), c. |edit_ware()|, (for when the user selects
edit above) and d. |add_to_db()|.
2. Implement |get_string_input()|.
3. Implement |edit_ware()|.
4. In 1. above, we are writing straight-line code. No loops, and the
only branching is the calls to |edit_ware()| and |add_to_db()| based
on the users input. This means we do not yet implement the case
where the user makes an edit, gets a printout and makes the edit
again. This support we add in this step.
5. Make price and amount ints in ware and replace the two calls to
|get_string_input()| with calls to an assumed |get_int_input()|.
6. Implement |get_int_input()|.
7. Decide how to do the storage location. And implement that. This is
likely several steps given that we cheated on the type for storage
location before.

The last thing we will do is to deal with the database – a final piece
of the puzzle missing to finish this action.

Attention

*Multiple Possible Ways Forward*

Undo is a good example of a choice in how we choose to go forward. For
example, we could chose to implement undo for each action that we
implement, i.e,. after add we implement undo add, after edit we
implement undo edit, etc. Or we simply wait until all actions are
implemented and implement undo for all of them in a single hit. There is
no right or wrong here, just two different ways of working, each with
its own merit. For example, it is easy to argue that implementing undo X
right after implementing X is the best way because at this point, we
probably have the best understanding of how X works. On the other hand,
it is easy to argue that if we implement the entire undo behaviour at
the same time, it is easier to come up with a clean design for it that
will work for all cases. If we started with a design for undo add, we
might not yet see all the information that must be saved. (For example,
undo edit will require us to store the original values of the edited data!)


###      3.9 Step 8: Think a Little – Code a Little
<a href="#TOC"> ` ^-- ` </a>

Choosing the proper database design is of course very important for the
program at hand. There are multiple alternatives ranging from simple to
hard, for example:

1. A statically sized array of wares
2. A dynamically sized in-memory structure like a list or a tree
3. An external database back-end

There is however one very important realisation at this stage and that
is that right now, *it does not matter* (right now).

The rationale is that we are trying to piece by piece construct a piece
of software that fulfils the specification. We are just about to wrap-up
the part of the program that will be adding the first item in the
database. We don’t have the code to remove items, edit them, etc. So
really, we could not care less at this point how the database is designed.

This is a great opportunity to dodge again. This time, we are going to
dodge in three discrete steps:

1. The database holds only one single element
2. The database is an array of fixed maximal size
3. Make the final decision once the program’s done otherwise

The first database design will allow implementing everything except
adding and removing with multiple wares, and listing multiple wares. The
second design will allow us to do all that. The third design will make
the program less of a Mickey Mouse program
<http://www.catb.org/jargon/html/M/mickey-mouse-program.html>.

Following the first database design, we can now define a struct for the
database like so:

```c
/* main.c */

struct db
{
    struct ware ware; // TODO: improve later
};

typedef struct db db_t;
```

Now, we can finally finish the implementation of |add_action()|. To do
this, we will go through the program and make a few simple changes:

1. We want to create a |db_t| value in main.
2. We want to pass a pointer to the |db_t| value to |add_action()|.
3. We want to pass a pointer to the |db_t| value to |add_to_db()|.

Now we can implement |add_to_db()| like so:

```c
/* main.c */

void add_to_db(db_t *db, struct ware ware)
{
    db->ware = ware;
}
```

And now we are done with |add_action()|.

As we finish more actions and move on to the second database design, we
simply update struct db:

```c
/* main.c */

#define Max_elements_in_db 128
struct db
{
    struct ware wares[Max_elements_in_db]; // TODO: improve later
    int size;
};

```

That was easy, and now we can finalise more complex parts of out
program. Moreover, if we have isolated the how the database works from
the rest of the program correctly, only functions like |add_to_db()|
etc. will have to change as a result:

```c
/* main.c */

void add_to_db(db_t *db, struct ware ware)
{
    // Saves ware on the next available index and increase size by 1
    db->wares[db->size++] = ware;
}
```

###      3.10 Step 9, and beyond
<a href="#TOC"> ` ^-- ` </a>

Think/code/refactor/remove cheats. Repeat until done. Find a good balance.


###      3.11 Summary and Conclusions
<a href="#TOC"> ` ^-- ` </a>

Almost all hard problems can be broken down into less-hard subproblems,
which in turn can be broken down to even less-hard subsubproblems, etc.
After a while of breaking work down into smaller and smaller tasks, the
tasks finally become simple, and that is the right time to start solving
them. That’s SIMPLE in a nutshell.

As soon as we hit a bump in the road – a problem we don’t know
immediately how to solve, or don’t know how to solve well, or just
cannot be bothered with solving right now – we keep things simple by
stubbing and dodging. We reserve ourselves that right by always making
clear notes about our cheats and dodges and by promising ourselves that
we will set things straight in the future. And by keeping our promises.


##    4 A Summary of SIMPLE in Ten Steps 
<a href="#TOC"> ` ^-- ` </a>

**Start with a high-level work breakdown structure**

Mine your specification for data (nouns) and actions (verbs –
behaviour/functions). Make simple drawings, e.g., mindmaps, to
record your insights. Making things look easy is almost always good.

**Write code to test the validity of your thinking**

…not to drive the thinking! Thinking should invariably come before
coding, especially thinking about how you check that your thinking
is valid.

**Always have a working program**

In combination with stubbing, this usually means inserting dummy
functions.

**Compile after every change**

Fix errors now, not later. Fix errors one by one. Fix errors in the
order they were printed. Take time to actually read the compiler
message so you know you are fixing the right thing.

**Run the program “all the time” to spot errors**

This requires always working code – preferably in combination with
automated test that don’t involve ocular inspection, etc.

**Recursively break your problems up into smaller sub problems**

Only start solving problems when they start feeling easy. Make a
task for each problem or subproblem to put on the stack, take tasks
from the stack in a reasonable order (preferably easiest first);
when the stack is empty – you are done!

**Break each task up into increments and start with the easy ones**

Generate new tasks to put on the stack as you go. If suitable, start
with a straight-line version (without any if-statements). When the
straight-line version works add conditionals, one by one. Start with
the most basic or the most insteresting cases. When you are writing
a loop, do the above steps first and add the looping step last.

**Whenever you run the risk of getting stuck, cheat**

Don’t forget to push new tasks on the stack that undoes the
stubbing, later. This records the cheat, which is great.

**Use dodging to help breaking complex cases up into several less complex ones**

Don’t forget to push new tasks on the stack that undoes the dodge,
later. This records the dodge, which is great.
Alternate between thinking, coding and refactoring

  * thinking – not so much that you get stuck, though,
  * coding – but never without first thinking about what to code,
    and occasionally
  * refactoring – especially to address your cheats and dodges.

Refactoring is good to do between larger tasks. Make it a habit to
continuously go back and refactor your solutions so that recently
gotten insights rub off on older code too.

        Note that this is a simple programming methodology, not a fully-fledged
        software design methodology. This means that it will not scale to
        “large” software, but will work fine for relatively simple programs up
        to say a few thousand lines of code. Certain domains, especially
        mathematical domains, might benefit from more advance thinking before
        implementing because there are more “truths” in such domains than in
        more fickle domains, especially ones involving human users. Although the
        running example used a program with a user interface, SIMPLE works
        equally well with non-UI programs.

![Everyone can code](everyone-can-code.jpg)


##    5 Tickets, Tasks and Post-it Stacks
<a href="#TOC"> ` ^-- ` </a>

Programming with SIMPLE revolves around breaking problems down into sets
of smaller and smaller tasks. Actually, almost all programming works
like that.

Our first problem was “implement an information system for a warehouse”.
This is a very big and vague problem, so to solve it, we perused the
specification to find the top-level actions that the system should
support. We thus identified a set of tasks, one for each action, and
started working through the tasks one by one. We carefully chose the
order of tasks to make sure that we started as simple as possible and
never worked on a task that could not be a part of the running program
because it relied on another feature that wasn’t quite ready.

We are following a divide-and-conquer approach, where you recursively
break down tasks into increasingly smaller pieces until each piece is
trivially solvable. As is visible, from the steps above, we do not
strive to break everything down into simple problems. For example, we
started by doing |quit_action()| as soon as we realised it was simple
enough, without caring about breaking down everything else first. The
more you implement the program, the more you will understand it. You
will never understand the program less than right at the start –
therefore, trying to make a detailed planning with that limited
understanding is a bad idea. Instead, tasks will be created as part of
the implementation. At every step of the way, you will have a pile of
tasks to perform. When this pile is empty, you’re done.

The cases where tasks are created in SIMPLE are ideally:

1. As part of the initial identification of top-level tasks
2. As part of the data design
3. When you pick a task to implement, you usually start by breaking it
down into several smaller tasks
4. Stubbing creates a new task for the stubbing function
5. Dodging creates one or more new tasks to undo the simplifications

Naturally, tasks are sometimes created because you forgot to create them
elsewhere, or because of misunderstanding, etc.

*Actually having a physical pile of tasks in front of you is not a bad
idea.* It is quite common in modern software development organisations
to rely heavily on post-it notes for task planning and follow-up. You
will have one or two post-it notes that you are currently *doing*, a
growing pile of post-it notes that are *done*, and one pile that’s
*next*. Physically moving something from doing to done or seeing a large
stack of notes in the done pile is gratifying. Physically moving
something from next to doing reifies a commitment. Glancing at a number
of post-it notes in the next section of the paper or whiteboard where
you keep your tasks is a quick way to get an overview of where you are
headed.

Using a physical stack of post-it notes or some equivalent software is
an excellent way to manage the constantly growing and shrinking tasks.
Of course, adding notes in the code is great too, but why not both.

Regardless of how you manage your tasks, an important question is where
you put new tasks when they are created – to the front (so they are next
in line) or to the back (so they won’t be dealt with for a while). There
is no simple rule but there is a SIMPLE guideline: *simpler tasks go in
front of more complicated tasks.*

When you test your software, you may run into bugs or defects which
aren’t of immediate importance. For each defect that you do not
immediately fix, create a ticket, and put that in the task queue. A
ticket is a task like any other, just one that’s about fixing something
that’s inadvertently broken. Creating tickets for defects is a good way
to avoid breaking the current flow by switching into debugging mode, and
helps us remember it so it eventually gets fixed.


##    6 The Three Rules of Stubbing
<a href="#TOC"> ` ^-- ` </a>

Stubbing is one of the best tools in the programming toolbox, and a
trick you should be using all the time. The rule of stubbing is this
(also repeated above):

If you don’t know immediately how to solve a problem,
assume the existence of a function
that solves this problem for you,
and call that function.

There is also a second rule of stubbing:

If you write code that you think about as
Step 1; Step 2; Step 3; … Step n,
and one of the steps suddenly turns out to be
way longer/more complicated/more involved than the others,
assume the existence of a function that performs that step for you,
and call that function.

Finally, there is a third rule of stubbing:

If you write straight-line code,
and you suddenly come across a step that you
just cannot be bothered with implementing right now,
assume the existence of a function that performs that step for you,
and call that function.

(Yes – I deliberately formatted them as poems.)

In addition to help you stay focused on the control flow you are
implementing and not get stuck in details, stubbing has the automatic
side-effect of breaking down your code into smaller chunks. It also
isolates trickier code inside single functions. For example, the logic
for reading user input is now nicely encapsulated inside the
|get_char_input()| function and we can solve this problem completely in
isolation of the rest of the code for quitting the program although that
code depends on it.

By giving a proper name to each function invented by stubbing, we make
sure that the code reads more high-level, and it also helps us remember
what problem the function in question should solve.


##    7 Dodging: Not Quite Stubbing
<a href="#TOC"> ` ^-- ` </a>

When doing the initial database design, we made proper use of dodging
twice to make immediate simplifications that help us make progress and
construct good increments. Here are two examples of dodging guidelines:

If a feature has several similar yet not identical cases,
start by reducing them to the most basic case
and getting that to work.

and

If a feature involves a collection of objects,
investigate whether starting with just a single object
is possible and easier.

Dodging involves simplifying the requirements temporarily in order to
create a simpler specification. The word temporarily is key here, and it
is imperative that we have some system in place to help us remember what
simplifications we have made to make sure we roll them back at some
point and replace them (step by step) with the full specification.

By starting with a design of the ware struct where everything is a
string, we have reduced the number of different input methods for wares
from three to one, and also to the simplest one where input does not
need any real checking. This way, we can write a program in which the
user can input data into the database, and once that program is
sufficiently implemented, we can add more input methods and change the
types in the ware struct. Following the guidelines for increments, we
implement support for:

1. Name, description
2. Price, amount
3. Storage location

in this very order, which is the ascending order of complexity. That is
is the fact is easy to see: all input is a valid string; integer input
may fail if the user types something other than a number; storage
location requires two different inputs to be correct.

Finally, by starting with a database with only one item we have just
enough to be able to have a working program in which we do pretty much
everything except have more than one ware in the database. The logic for
everything except writing to the database should be exactly the same for
the entire program regardless of whether the database is an array of
wares, a binary tree, etc.

------------------------------------------------------------------------
<a href="#TOC"> ` ^-- ` </a>

*Want to report a bug?* [Please place an issue here](https://github.com/IOOPM-UU/course-web/issues/new?title=Bug%20on%20page%20simple.org&body=Please%20describe%20the%20issue%20clearly,%20help%20me%20locate%20it%20on%20the%20page,%20and%20if%20possible%20suggest%20a%20fix.&assignee=TobiasWrigstad).
Pull requests are graciously accepted (hint, hint).

*Ended up here randomly?* These are the pages for a one-semester course
at 67% speed on imperative and object-oriented programming at 
[the department of Information Technology](http://it.uu.se/) 
at [Uppsala University](http://www.uu.se/), ran by [Tobias Wrigstad](http://wrigstad.com/).

Author: Tobias Wrigstad

Created: 2019-04-19 Fri 17:39

[Validate](http://validator.w3.org/check?uri=referer)


