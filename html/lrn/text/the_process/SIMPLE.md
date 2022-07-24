<div class="bg_onja"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

⇦ [home](../../../../index.html) - [lerina](../../index.html) - [texts](../index.html) - [the process](./index.html)

# Onja ⋰⋱⋰⋱⋰⋯ (Wave) – A Minimalistic Programming Methodology

<!-- ↝  U+219D -->
I've been trying to explore what "loops" [software writers](https://www.youtube.com/watch?v=xPecMsFmEm4){target=_blank} actually go though both internally and externally while developing or implementing a solution to a POC (Problem, Opportunity, Challenge).

We are both [logician](https://en.wikipedia.org/wiki/Logic){target=_blank} and 
[poets](https://www.youtube.com/watch?v=-jRREn6ifEQ){target=_blank}. 

We are also [iterative artists](http://fineartdrawinglca.blogspot.com/2016/07/iterative-drawing_11.html){target=_blank} building layers of abstractions to solve a given purpose. 

What rituals and set of practices do programmers use when alone and ready to pick a problem to solve?
I had the good fortune of stumbling upon [SIMPLE: A Programming Methodology](http://wrigstad.com/ioopm18/simple.html) by Tobias Wrigstad, a Professor in Computing Science at [Uppsala University](http://www.uu.se/){target=_blank}.
It expressed better than I could what most programmers I know, including myself actually apply in some form or another.


This text is a re-interpretation of those ideas consolidated under three guiding principles and three precepts. 

__Principles__

1. KISS (keep it small and  straightforward). Think Big _ in **small steps**.
2. TDD stands for **Thinking** Driven development  
3. BDD stands for **Build** Driven development  

__Precepts__

- Break it down  
- Validate 
    * test assumptions and hypothesis
    * test your thinking with code
- Never end in a Broken state. 

__Rules for coding__

- you must test working build
- You can only commit working builds
- Breakdown Tasks until there is only one Item per sub-tasks
- Maintain the flow
    * Avoid stoping by temporarly subtituting complicated tasks with a simpler version
    * use `props` and `stand-ins` until the real think can take over the task
- Write your thoughs as comments or pseudo-code
- Never stop with a Broken build.

__Rules for design__

- Write imperatively first. Then refactor into a functional style.
- See thing in three 
    * Data - Input - Output - processing
    * Noun - Verbs - Attributes
    * Data - Computation - Actions
- Favor push over pull when it comes to Data.
- Favor decomposition and composition at all times.

---

__Keep the tempo__

Alternate between::
    
    `thinking` and `validating`
    `coding` and `verifying`

Think a little: not so much that you get stuck in analysis paralysis.  
Validate a little: Just enough tries or code to validate the thinking.  
_ Tobias Wrigstad


__WAVE tempo__

- ⋰⋱⋰⋱⋰⋯
- Think - Confirm - Create - Confirm - Think ...
    * Think a little
    * Code a little
    `C`onsidered `O`utput `D`one `E`fficiently (CODE)


"make it work
make it better
REAL SOON!"
_ Coding Mantra (Venkat Subramaniam)

"Simplicity is a great virtue 
but it requires hard work to achieve it 
and education to appreciate it. 
And to make matters worse: complexity sells better."
_ [Edsger Dijkstra](https://www.goodreads.com/quotes/215637-simplicity-is-a-great-virtue-but-it-requires-hard-work){target=_blank}

---

## The Wave (Onja 〜 ) ⋰⋱⋰⋱⋰
<!-- U+301C 〜  U+22F1 ⋱  U+22F0 ⋰ -->
≋ U+224B  
❀ (U+2740)  
⥵  U+2975  

Its not a cycle. Its a three stages wave rippling through the flow

- Trough: Think: not so much that you get stuck in analysis paralysis.
- base: Validate 
- Crest: unbroken idea or code 

```
concretize --------**------------------**------------------**----------------***--------
|                *    *              *    *              *    *            *    
Verify     ----*--------*----------*--------*----------*--------*--------*--------------
|            *            *      *            *      *            *    *            
Think      *----------------**-*----------------**-*----------------**------------------
```

Think, Verify your thinking, concretize, verify your implementation, think ...

longitudinal progress (side to side though the flow of time and working build delivered)  
transverse (up and down) phases think, verify (manual walkthrough, automated tests, code that validate the thinking), concretize (task breakdown, working build, ...) 

<!--
Waves are actually energy passing through the water, causing it to move in a circular motion.
Though waves do cause the surface water to move, the idea that waves are travelling bodies of water is misleading.
The water in waves doesn’t travel much at all. The only thing waves do transmit across the sea is energy.

Wave height: the distance between the crest and the trough.
Wavelength: the distance between two identical points on successive waves, for example crest to crest, or trough to trough.
Still water level: where the water surface would be if there were no waves present and the sea was completely calm.

source: https://oceanexplorer.noaa.gov/facts/waves.html

Period: the time it takes for two successive crests to pass a given point.
Frequency: the number of waves passing a point in a given amount of time, usually expressed as waves per second. This is the inverse of the period.
Speed: how fast the wave travels, or the distance traveled per unit of time. This is also called `celerity` (c), where

c = wavelength  x  frequency

Therefore, the longer the wavelength, the faster the wave.

 that celerity is responsible for increases and decreases in stream discharge (the hydrograph), while velocity controls how old is the water reaching the stream at a given time.
source: https://www.annascaini.com/velocity-and-celerity
-->
The point is to be as Still as possible. The shorter wavelength and the smaller the wave height the flatter and managable our activity will be.
The apparent Cerelity hit due to short wavelength is offset though sustainable high frequency.

hurdles
you will require a good understanding of the algorithms and techniques you intend to implement.

---

### Think a little

- What is the quest?
What is the desired final result? This is your ultimate output. 
    * That is last step in a chain of results to satisfy or complete the quest.
    * Define the scope of the project. 

- What are you starting with?
What are the available the inputs.

- What is missing to get from those initial input to the final output.

### Create a little

- Validate
    
- Concretize your thinking or code.

- Verify

---

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


---

⌌ ----------------------- ⌍    <!-- U+230C U+230D -->  
  ⌨ Practice time      <!-- U+2328 -->  
⌎ ----------------------- ⌏    <!-- U+230E U+230F -->  


https://www.geeksforgeeks.org/how-to-approach-a-coding-problem/

### The minimum viable product was defined in abstract terms

### Specific required features

### The rhythmic wave

stages of building our project:
• Analyzing the problem domain
• Modeling system behavior
• Building the tokenizer
• Building the parser
• Building the evaluator
• Dealing with errors
• Building a command-line application

</main>

<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://razafy.com/img/github32px.png){.link .glow}
  </a>
</footer>

<script src="https://razafy.com/js/toc.js"></script>
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
<li><a href="../../../../index.html">⇦ home</a></li>
<li><a href="../../index.html">lerina</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>

