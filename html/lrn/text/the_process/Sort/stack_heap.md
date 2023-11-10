<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
# The Heaps and Stacks Process

A Simple alternative to Kanban board for personal productivity

While Kanban is great for collaborative work, this one is simple and very powerful 
for personal efficiency. When you have your don't disturb cap on, nothing cuts through 
deep work backlog like the Heaps and Stacks process. 

This is the heart of the [`thrust` productivity system](./thrust_productivity_system.html), which
includes additional help to keep focused one the work in a sustainable way.

## start with the Heap

* Note things on the a heap. 
* One heap per topic
* complete the heap with the dependencies and prerequisites

## stack the Stack

* start with the end in mind
* What is the last step just before the end result
* what comes just before that
* ... proceed until you back track to your current situation

## do the work. 

* Pop the first element of the stack. Assign to the register.
* Pop and do until the stack is empty

## Register

It can be a notepad, a dry-erase board, a mirror ...

* Write down the element poped for the stack
* Do NOW AFAP (as Fast as Possible): Beat the timer
</main>
<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://lerina.github.io/img/github32px.png){.link .glow}
  </a>
</footer>

<script src="https://lerina.github.io/js/toc.js"></script>
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
<li><a href="../../index.html">⇦ home</a></li>
<li><a href="../../index.html">lerina</a></li>
<li><a href="../index.html">text</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
