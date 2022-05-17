<div class="bg_lrn"></div>
<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>

<main>

| There are those who "think and create"
| and  those who "blink and cry hate".
|                             _ lerina

## Crazy _goal_ > Smart _goal's_ 

We've all heard of SMART the an acronym used for creating Specific Measurable Attainable Relevant and Time-based goals. It doesn't say anything about execution.

CRAZY goal is the next step. 

> `C` ontrained in time and space  
> `R` elevant, recoverable  
> `A` ccountable process actions  
> `Z` oom out, zoom in: Zap it!  
> `Y` ield early, reap frequently  

Notice its GOAL, not goals. Pick one goal and make sure its CRAZY.

## `C` ontrained in time and space  
Schedule the _lifeline_. Number of time-bubbles to work on it.

## `R` elevant, resiliant  
(non-frivoulus and impactful)

## `A` ccountable process actions 
top-down view, bottom-up execution

## `Z` oom out, zoom in: Zap it!  
see the context, the why, the big picture
Dive into the task, one sub-task at a time

## `Y` ield early, reap frequently  
Measure progress by keeping count of the time-bubbles dedicated to the process of sub-task completion.

Keep in mind that 
Progress is a process:  
While one Plans progress with goals, one Makes progress with processes.


## Hello. I'm a software writer.

That's right! writer.

More akin to writers of the film industry than the book market.  
Writing code has technical constraints and people cooperation considerations.

*- As a code author*,  
I don't write code to make computers do things. That's the job of our programs.  
I write code to be shared, maintaned and efficiently accomplish a mission.  
I write solutions to challenges, opportunities and problems that have first  
been modelled and solved to yield three imperatives:

-   clear output
-   defensive input
-   explicit nomenclature

Talking of writing, I also dabble in [poetry](./text/poems.html){.link
.black-50} and have occasional things to say about the [Software
development journey](./code/index.html){.link .black-50}.

---
Also see:


### [Selected texts](./text/index.html)
### [Code](./code/index.html)
#### [Secure Coding](./code/secure_coding/index.html)
#### [Game dev](./code/game_dev/index.html)
#### [Coding Math](./code/coding_math/index.html)

</main>

<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://razafy.com/img/github32px.png){.link .glow}
  </a>
</footer>
<script>
let anchor= document.createElement('a');
anchor.href="javascript:closeNav()"; //void(0)"; //anchor[0].onclick = closeNav();
anchor.className = "closebtn";  
anchor.innerHTML="&times;";
document.getElementById("TOC").prepend(anchor);

/* Set the width of the sidebar to 250px and the left margin of the page content to 250px */
function openNav() {
  document.getElementById("TOC").style.width = "60%";
  document.getElementsByTagName("MAIN").style.marginLeft = "250px";
}

/* Set the width of the sidebar to 0 and the left margin of the page content to 0 */
function closeNav() {
  document.getElementById("TOC").style.width = "0%";
  document.getElementsByTagName("MAIN").style.marginLeft = "0px";
}

let navCrumbs= document.createElement('div');
navCrumbs.className = "hover-nav";
navCrumbs.innerHTML = `
<div class="hover-nav">
<ul>
<li><a href="../../index.html">⇦ home</a></li>
<li><a href="./index.html">lerina</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
