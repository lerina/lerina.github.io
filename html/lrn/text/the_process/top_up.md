⇦ [home](../../../../index.html) - [lerina](../../index.html) - [texts](../index.html)

## SIMPLE TCR

[SIMPLE](http://wrigstad.com/ioopm18/simple.html#orgb3285eb)
In a nutshell SIMPLE dictates that you:

- Use the specification in a structured manner to do both function design and data design.
- Work in an incremental fashion, by solving many simple problems instead of few complex ones.
- **Always have a working program** that you can run and test.
- Compile continuously.
- Test continuously, preferably automated but at least manually.
- Write straight-line code first, before complicated branches and loops.
- Use tricks – cheating and dodging – to keep programming simple, and push problems into the future.

[(Test && Commit ) || Revert](https://www.youtube.com/watch?v=hi99zbRzkRM)

- Always have code in a valid state.  
- Write some code, `git add .`
- Run your tests.  
    * If the tests pass, the code is immediately committed.  
    * If the tests fails, immediately revert with `git reset --hard`.  

---

- Code must compile and run between each git commit
- Test one non-trivial code at a time
- commit after each passing test.

###
|↴  
| TOP-DOWN DESIGN - [outside-in](https://www.holdenrehg.com/blog/2018-09-22_write-better-code-outside-in)  
|     ↱↴ Think a little    
|     ↑↲      Write a little  
|  
| BOTTOM-UP IMPLEMENTATION  - [inside-out](http://xunitpatterns.com/Philosophy%20Of%20Test%20Automation.html)
|     ↱↴ Code a little  
|     ↑↲      Test a little  
|←  


### [outside-in](https://www.holdenrehg.com/blog/2018-09-22_write-better-code-outside-in)
Define the entry point for the code.  
Assume that any module, function, helper, library, etc. is available, even if the code does not exist.  
Write the ideal version of the entry point.  
Write the ideal version of any code that was assumed until you start reaching “low level” functions that perform the bulk of the logic.  
In other words sketch with one line comments the minimal viable system, and breaks it down into smaller components
Repeat until you have all functions defined.  

This completes your initial minimal design.

### [inside-out](http://xunitpatterns.com/Philosophy%20Of%20Test%20Automation.html)
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

