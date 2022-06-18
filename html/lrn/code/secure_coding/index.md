<div class="bg_lrn"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>


## Rust and Friends
Rust works well with other programming languages. 

### What is Rust

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

> Rust is a programming language that’s focused on safety, speed, and concurrency. Its design lets you create programs that have the performance and control of a low-level language, but with the powerful abstractions of a high-level language. These properties make Rust suitable for programmers who have experience in languages like C and are looking for a safer alternative, as well as those from languages like Python who are looking for ways to write code that performs better without sacrificing expressiveness.
_ rustbook

[Rust](https://www.rust-lang.org/)
[wikipedia: Rust programming language](https://en.wikipedia.org/wiki/Rust_(programming_language))

### What problem(s) does Rust Solve?

Featuring

- zero-cost abstractions
- move semantics
- guaranteed memory safety
- threads without data races
- trait-based generics
- pattern matching
- type inference
- minimal runtime
- efficient C bindings

source: The older "_nerdy_" rust website


Rust is a language that allows you to build high level abstractions, 
but without giving up low-level control - that is, 
control of how data is represented in memory, control of which threading model 
you want to use etc.
Rust is a language that can usually detect, during compilation, 
the worst parallelism and memory management errors (such as accessing data 
on different threads without synchronization, or using data after they have 
been deallocated), but gives you a hatch escape in the case you really know 
what you're doing.
Rust is a language that, because it has no runtime, can be used to integrate 
with any runtime; you can write a native extension in Rust that is called 
by a program node.js, or by a python program, or by a program in ruby, lua etc. 
and, however, you can script a program in Rust using these languages. 
-- "Elias Gabriel Amaral da Silva"

### memory_safe
[wikipedia Memory safety](Memory safety)

Rust makes it difficult to leak memory.  
[double free](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory), [Null deference](https://owasp.org/www-community/vulnerabilities/Null_Dereference), and [dangling pointers](https://owasp.org/www-pdf-archive/OWASP_IL_8_Dangling_Pointer.pdf).
 _Rust performs the majority of its safety checks and memory management decisions at compile time, so that your program’s runtime performance isn’t impacted_. 


- [Why rust](https://www.youtube.com/watch?v=_wy4tuFEpz0)
- [Ownership](https://www.youtube.com/watch?v=TCUBSbJENO4)
- [Shared borrow](https://www.youtube.com/watch?v=61bFe3jqi1E)
- [Mutable borrow](https://www.youtube.com/watch?v=pd7PJ6q4I3M)


### blasingly_fast

benckmarks here

### modern_tools

ecosystem here

- cargo
- crates.io

cargo-edit
cargo-watch


## Secure coding

The French government did a great job with the ANSSI GUIDELINES for Rust:  
[PROGRAMMING RULES TO DEVELOP SECURE APPLICATIONS WITH RUST](https://www.ssi.gouv.fr/uploads/2020/06/anssi-guide-programming_rules_to_develop_secure_applications_with_rust-v1.0.pdf)


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
<li><a href="../index.html">code</a></li>
<li><a href="./index.html">secure coding</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
