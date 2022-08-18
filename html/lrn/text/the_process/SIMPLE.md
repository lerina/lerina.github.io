<div class="bg_onja"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

⇦ [home](../../../../index.html) - [lerina](../../index.html) - [texts](../index.html) - [the process](./index.html)

# Onja ⋰⋱⋰⋱⋰⋯ (Wave) – A Minimalistic Programming Methodology

> Beautiful Code  

```
We ascribe `beauty` to that which is simple,
which has no superfluous parts;
    which exaclty answers its end,
    which stands related to all things,
    which is the mean of all extremes.
    
    _ Ralph Waldo Emerson, _The conduct of life_
```

```
“La perfection est atteinte, 
non pas lorsqu'il n'y a plus rien à ajouter, 
mais lorsqu'il n'y a plus rien à retirer.”

    ― Antoine de Saint-Exupéry
```

<!-- ↝  U+219D -->
## Context

This text is inspired by [SIMPLE: A Programming Methodology](http://wrigstad.com/ioopm18/simple.html){target=_blank} by Tobias Wrigstad, a Professor in Computing Science at [Uppsala University](http://www.uu.se/){target=_blank}.

It also draws from structured programming ideas already in usage back in the 70's and best exposed by Dolores M. Etter in Problem solving with structured FORTRAN 77 (The Benjamin/Cummings Pub 1984) and Engineering Problem Solving with C.

__Problem-Solving process or methodology__

Etter stipulates  a five point process to solve problems involving the a computer:

The process or methodology for problem solving has the following five steps:

1. State the problem clearly.
2. Describe the input and output information.
3. Work the problem by hand (or with a calculator) for a simple set of data.
4. Develop a solution and convert it to a computer program.
5. Test the solution with a variety of data.

[source: Dolores M. Etter in Problem solving with structured FORTRAN 77](https://archive.org/details/problemsolvingwi0000ette){target=_blank}

Key Steps for Problem Solving

1. Define your problem
2. Make sure you fully understand the problem
3. Break the problem down into small and manageable pieces 
4. Go as deep as you can, until you can get to easy (yes or no) questions
5. Work your way from the bottom until the problem is solved
6. Recognise some problems are completely out of your control and that's ok.

[source: Problem Solve Like a Computer Programmer | Kyle Smyth | TEDxRPLCentralLibrary](https://www.youtube.com/watch?v=x77-gT8bWLo){target=_blank}

__Programming methodology__

Wrigstad lays out a Ten Steps system he calls "SIMPLE"

1. Start with a high-level work breakdown structure  
Mine your specification for data (nouns) and actions (verbs – behaviour/functions). Make simple drawings, e.g., mindmaps, to record your insights. Making things look easy is almost always good.
2. Write code to test the validity of your thinking  
…not to drive the thinking! Thinking should invariably come before coding, especially thinking about how you check that your thinking is valid.
3. Always have a working program  
In combination with cheating, this usually means inserting dummy functions.
4. Compile after every change  
Fix errors now, not later. Fix errors one by one. Fix errors in the order they were printed. Take time to actually read the compiler message so you know you are fixing the right thing.
5. Run the program “all the time” to spot errors  
This requires always working code – preferably in combination with automated test that don’t involve ocular inspection, etc.
6. Recursively break your problems up into smaller sub problems  
Only start solving problems when they start feeling easy. Make a task for each problem or subproblem to put on the stack, take tasks from the stack in a reasonable order (preferably easiest first); when the stack is empty – you are done!
7. Break each task up into increments and start with the easy ones  
Generate new tasks to put on the stack as you go. If suitable, start with a straight-line version (without any if-statements). When the straight-line version works add conditionals, one by one. Start with the most basic or the most insteresting cases. When you are writing a loop, do the above steps first and add the looping step last.
8. Whenever you run the risk of getting stuck, cheat  
Don’t forget to push new tasks on the stack that undoes the cheating, later. This records the cheat, which is great.
9. Use dodging to help breaking complex cases up into several less complex ones  
Don’t forget to push new tasks on the stack that undoes the dodge, later. This records the dodge, which is great.
10. Alternate between thinking, coding and refactoring  
    * thinking – not so much that you get stuck, though,
    * coding – but never without first thinking about what to code, and occasionally
    * refactoring – especially to address your cheats and dodges.

[[source: SIMPLE _ A Programming Methodology](http://wrigstad.com/ioopm18/simple.html){target=_blank}

__TDD__


There are 5 steps in the TDD flow:

1. Read, understand, and process the feature or bug request.
2. Translate the requirement by writing a unit test. The unit test will run and fail as no code is implemented yet.
3. Write and implement the code that fulfills the requirement. Run all tests and they should pass, if not repeat this step.
4. Clean up your code by refactoring.
5. Rinse, lather and repeat.

[source: 5 steps of test-driven development](https://developer.ibm.com/articles/5-steps-of-test-driven-development/){target=_blank}

REM: every simgle point of TTD has been turned into a business

1. ... Tools, books and seminars about Group work and software for ticketing, bug tracking ...
2. ... Books & seminars about the benefits of tests.
3. ... Tools & seminars about Writing tests.
4. ... Books & seminars about Clean code and refactoring.
5. ... Books & seminars about CI/CD and devops.

__REACTO__

1. Repeat: make sure you do understand the problem.
2. Example: get insights by doing examples 
3. Approach: come up with your approach(es) to the problem (brute force first)
4. Code: write the code for your chosen approach
5. Testing: pass the testcases
6. Optimize: optimize the complexities (time and space) of your algorithm

[source: Whiteboard Coding Interviews: A 6 Step Process to Solve Any Problem](https://www.youtube.com/watch?v=DIR_rxusO8Q){target=_blank}

__Function Design Recipe__
UofT

__Js recipe__
find it

__ALSO of NOTE__

[How to Solve a Problem in Four Steps: The IDEA Model](https://www.youtube.com/watch?v=QOjTJAFyNrU){target=_blank}  
[Problem-Solving for Developers - A Beginner's Guide](https://www.youtube.com/watch?v=UFc-RPbq8kg){target=_blank}  
[How to Design Programs, Second Edition](http://htdp.org/2022-8-7/Book/index.html){target=_blank}  
[The Program Design Recipe](https://course.ccs.neu.edu/cs5010sp15/recipe.html){target=_blank}  
[](){target=_blank}  


## Introducing The Wave (Onja 〜 ) ⋰⋱⋰⋱⋰

There is no clear cut between programming steps accumulated over time. 
In real life tend to blend these ways of approaching the actual practice of writing software.
Through time these methods and practices have been either made systematic or rendered implicit.

In developing Onja, we took the _ancient_ but still very pertinant 5-points "Problem-Solving with a computer program solution" as exposed by D. M. Etter
and the most recent SIMPL Programming Methodology described by Wrigstad. 

Dolores M. Etter's 5 stage process can be can seen as the high-level view of tackling Problems, Opportunities or Challenges (POC). 
This high-level view can be subdivided into `Understanding the Problem` and `Validate the Solution`

Understanding the Problem:
    
1. State the problem clearly.
2. Describe the input and output information.
3. Work the problem by hand (or with a calculator) for a simple set of data.

Validate the Solution:
    
4. Develop a solution and convert it to a computer program.
5. Test the solution with a variety of data

Wrigstad's "10 rules" may be seen as the low-level expression of what actually goes through the mind of a programmer
as software is written.

__I Principles__

1. KISS (keep it small and  straightforward). Think Big _ in **small steps**.
2. TDD stands for **Thinking** Driven development  
3. BDD stands for **Build** Driven deployment  

__II Precepts__

- Breakdown Quests or Tasks until there is only one Item per sub-tasks
- Validate 
    * test assumptions and hypothesis
    * test your thinking with code
- Never stop in a Broken state or build. 

__III Rules for coding__

- You can only commit tested working builds
- Maintain the flow
    * Avoid stoping by temporarly subtituting complicated tasks with a simpler version
    * use `props` and `stand-ins` until the real think can take over the task
- Write your thoughts as comments or pseudo-code

__IV Rules for design__

- Write imperatively first. Then refactor into a functional style.
- See thing through three point of views 
    * Data - Input - Output - processing    (Procedural)
    * Noun - Verbs - Attributes             (Objects)
    * Data - Computation - Actions          (Fuctional)
- Favor push over pull when it comes to Data.
- Favor decomposition and composition at all times.

__V Keep the tempo__

Alternate between::
    
    `thinking` and `validating`
    `coding` and `verifying`

Think a little: not so much that you get stuck in analysis paralysis.  
Validate a little: Just enough tries or code to validate the thinking.  
_ Tobias Wrigstad

The Onja ⋰⋱⋰⋯ distiles these rules and stages into a rythmic wave like software writting process
one can condense into <sub>`t.h.i.n.k. a little`</sub>⋰<sup>`c.o.d.e. a little`</sup>⋱⋰⋯ 

<button>[NEXT: The wave Metaphor](./onja01.html)</button>  

</main>

<footer><a href="https://github.com/lerina" target="_blank" title="github">![github](https://razafy.com/img/github32px.png){.link .glow}</a>
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

