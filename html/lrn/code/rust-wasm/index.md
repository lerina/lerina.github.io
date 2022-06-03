<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

## Rust, Wasm and Webassembly

## About Rust

Rust is a programming language that’s focused on safety, speed, and concurrency. Its design lets you create programs that have the performance and control of a low-level language, but with the powerful abstractions of a high-level language. These properties make Rust suitable for programmers who have experience in languages like C and are looking for a safer alternative, as well as those from languages like Python who are looking for ways to write code that performs better without sacrificing expressiveness.  source: the [rustbook](https://doc.rust-lang.org/book/foreword.html)

- [more about Rust...](./code/secure_coding/index.html)  

- Webassembly: Wasm

"WebAssembly... defines a portable, size- and load-time-efficient format and execution model" _ [Luke Wagner](https://blog.mozilla.org/luke/2015/06/17/webassembly/){taget: _blank}

WebAssembly  is a safe, portable, and low-level binary instruction format.
Originaly designed to serve as a compilation target for the Web, 
it can also be used to run applications outside of the browser, thanks to WASI.

- WASI, the WebAssembly System Interface

It is a modular collection of standardized APIs. None of the APIs are required to be implemented to have a compliant runtime. Instead, host environments can choose which APIs make sense for their use cases.
For instance, filesystem access, environment variable support, and support for clocks and random number generators are commonly implemented.

- WAGI: WebAssembly Gateway Interface 

WAGI is a system for building HTTP services with simple WASM+WASI modules. 
It allows you to run WebAssembly WASI binaries as HTTP handlers. 

### Learn Rust
- [The book](https://doc.rust-lang.org/book/title-page.html)
- The best youtube resource to learn Rust is [LetsGetRusty](https://www.youtube.com/c/LetsGetRusty/playlists)
- my quick guide


### Learn Wasm
- the book
- My no-bloat workflow

### Webassembly
- the book
- A fun dive into Webassembly 

</main>

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
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
