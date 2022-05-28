
<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

| Some "think and create",
| while others "blink and cry hate".
|                                   _ lerina


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


## Learn more about Rust

Rust is a programming language that’s focused on safety, speed, and concurrency. Its design lets you create programs that have the performance and control of a low-level language, but with the powerful abstractions of a high-level language. These properties make Rust suitable for programmers who have experience in languages like C and are looking for a safer alternative, as well as those from languages like Python who are looking for ways to write code that performs better without sacrificing expressiveness.  source: the [rustbook](https://doc.rust-lang.org/book/foreword.html)

- [more about Rust...](./code/secure_coding/index.html)  

## My Workflow

There is a fake dichotomy about Top-down vs Bottom-up approach 
In reality most practices integrate elements of both.
We alternate between "do the right job" awarness and "do the job right" eagerness.
This is Reflective equilibrium applied to software development.

[read more](./code/workflow.html)  

see also:

- Video: [my tech and programming heroes](./my_heroes.html)
- papers:

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
<li><a href="../../index.html">⇦ home</a></li>
<li><a href="./index.html">lerina</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
