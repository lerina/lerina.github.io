<div class="bg_lrn"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>


## Rust and Friends
Rust works well with other programming languages. 

### What problem(s) does Rust Solve?

https://www.rust-lang.org/


Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

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

source: The older "nerdy" rust 

Rust is a language that allows you to build high level abstractions, but without giving up low-level control - that is, control of how data is represented in memory, control of which threading model you want to use etc.
Rust is a language that can usually detect, during compilation, the worst parallelism and memory management errors (such as accessing data on different threads without synchronization, or using data after they have been deallocated), but gives you a hatch escape in the case you really know what you're doing.
Rust is a language that, because it has no runtime, can be used to integrate with any runtime; you can write a native extension in Rust that is called by a program node.js, or by a python program, or by a program in ruby, lua etc. and, however, you can script a program in Rust using these languages. -- "Elias Gabriel Amaral da Silva"

### memory_safe
[wikipedia Memory safety](Memory safety)

### blasingly_fast

### modern_tools


## Secure coding

- Back to first principles
- [A quick tour of Rust](./fastpaced_rust01.html)
- Teaching Rust as a first language
- Building web3 with Rust and Wasm

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
