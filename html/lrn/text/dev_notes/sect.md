
<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# acronym driven development
:-)

## Uphold the S.E.C.T.

TDD, BDD, the noise has reached cult-like levels.
So here is my contribution, when you code, I urge you to stand by the SECT:

`S`tory Driven        Requirements  
`E`xample Driven      Design  
`C`ode Driven         Development  
`T`est Driven         Delivery  

And by all mean, push for 100% test coverage, of those portions of code that matter.

PS:  
I promise not to write books or make seminars about it.
It's safe to try and bonus it actually works! 


## Crazy _goal_ > Smart _goal's_ 

We've all heard of SMART the an acronym used for creating Specific Measurable Attainable Relevant and Time-based goals. It doesn't say anything about execution.

CRAZY goal is the next step. 

> `C` ontrained in time and space  [At the correct place, within the right time]
> `R` elevant and robust           [meaningful and worth your total commitment]
> `A` ccountable process actions   [chunks are either done or not. No excuses.]
> `Z` oom out, zoom in: Zap it!    [Top-down design, bottom-up implementation: Beat your best time.]
> `Y` ield early, reap frequently  [feedback loop]

Notice its GOAL, not goals. Pick one goal and make sure its CRAZY.

### `C` ontrained in time and space  
Schedule the _lifeline_. Number of time-bubbles to work on it.

### `R` elevant and robust
(non-frivoulus and impactful)

### `A` ccountable process actions 
top-down view, bottom-up execution

### `Z` oom out, zoom in: Zap it!  
see the context, the why, the big picture
Dive into the task, one sub-task at a time

### `Y` ield early, reap frequently  
Measure progress by keeping count of the time-bubbles dedicated to the process of sub-task completion.

Keep in mind that 
Progress is a process:  
While one Plans progress with goals, one Makes progress with processes.



## FOCUS when you code

`F`eedback friendly  
`O`rganised  and prepared  
`C`ommited  to the task  
`U`nswayed by distraction   
`S`ingle tasking  

## FEEDback Driven
|↴  
|  `F`raction the Problem to 'f'etch a solution  
|  `E`xperiment then 'e'liminate lesser solution(s)                  
|  `E`ncode and 'e'valuate  the candidate solution(s)          
|  `D`etect anomalies                         
|←  back  

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
<li><a href="../../../index.html">⇦ home</a></li>
<li><a href="../../index.html">lerina</a></li>
<li><a href="../index.html">dev notes</a></li>
<li><a href="./sect.html">acronym driven development</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>

