<link rel="me" href="https://hachyderm.io/@lerina">

<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# Hi, I'm Lerina, 
a software developer based in Waterloo ON, Canada<br/> but you can sometimes find me in Paris (France) or Antananarivo (Madagascar).

## I take assignments involving Rust and/or webassembly (wasm).  

These projects usualy happen within the context of a partial update or maintenance of some Python, Javascript/Ts, or C codebase .  
There are a lot of projects out there that are useful and deserve to ascend to the next level.  
This leveling-up often focus on Safer libraries, Faster modules, Leaner dependencies.  

## I Replace sensible modules with Rust and/or wasm drop-in replacements. 

Rust is a systems language that provides memory safety without garbage collection, and concurrency without race conditions.
It provides significant improvement in performance for interpreted languages such as Python or Javascript.
and a decrease the possibility of cyber-attack through bug explotation memory mismanagement.

Rust performs the majority of its safety checks and memory management decisions at compile time, so that your program’s runtime performance isn’t impacted.

see also: [Rust works well with other programming languages](./code/secure_coding/index.html)

## Is your project ready to ascend to the next level?

Does it need Safer libraries, Faster modules, Leaner dependencies?

Tell me about YOUR Pain:

`P`roject or Problem (to solve)  
`A`spiration or Angst/Anger (to fix)  
`I`tch or Irritation  (to heal)  
`N`eed or Niche (to satisfy)  

## See more about software Development
with code examples and opinionated short texts

<figure class="hover-img"><a href="./code/index.html">
<img src="../../img/code_index.jpg"/>
<figcaption>
<h3>more about <br/>software Development</h3>
</figcaption></a>
</figure>


## My Workflow

There is a fake dichotomy about Top-down vs Bottom-up approach 
In reality most practices integrate elements of both.
We alternate between "do the right job" awarness and "do the job right" eagerness.
This is Reflective equilibrium applied to software development.

[read more](./code/workflow.html)  

## Resume (CV)

2024:

- [html resume](./cv2024.html){target="_blank"}
- [pdf resume](./cv2024.pdf)

- [html Cv en français](./cv2024_fr.html){target="_blank"}
- [pdf Cv en français](./cv2024_fr.pdf)

## Dev's portfolio

* Live Projects 
* System Design Documents 
* Architectural/System Design Diagrams 
* UX/UI (in case of frontend) 
* Papers 
* Books 
* Blog articles  
* Videos 
* Podcast 

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
<li><a href="./index.html">lerina</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
