<div class="bg_onja"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

⇦ [home](../../../../index.html) - [lerina](../../index.html) - [texts](../index.html) - [the process](./index.html)

# Onja ⋰⋱⋰⋱⋰⋯ (Wave) – A Minimalistic Programming Methodology


"Simplicity is a great virtue   
but it requires hard work to achieve it   
and education to appreciate it.   
And to make matters worse: complexity sells better."  
_ [Edsger Dijkstra](https://www.goodreads.com/quotes/215637-simplicity-is-a-great-virtue-but-it-requires-hard-work){target=_blank}


⌌ ----------------------- ⌍    <!-- U+230C U+230D -->  
  ⌨ Practice time             <!-- U+2328 -->  
⌎ ----------------------- ⌏    <!-- U+230E U+230F -->  

https://www.geeksforgeeks.org/how-to-approach-a-coding-problem/

## Running Example: The Simple Warehouse 
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

