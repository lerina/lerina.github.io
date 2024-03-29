<div class="bg_lrn"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# Secure coding with Rust & Friends

## Rust, Wasm and more 

- [Rust and webassembly](./rust_wasm/index.html)

- Rust and Python

- Rust and C

- Rust and JavaScript

## Coding Math

This is a Rust/wasm port of 
[`Coding Math`](https://www.youtube.com/playlist?list=PL7wAPgl1JVvUEb0dIygHzO4698tmcwLk9){target="_blank"}
by Keith Peters

- Episode 001 to 005
- Episode 006 to 010
- Episode 011 to 015
- Episode 016 to 020

## Game Dev

### Games for the browser

- [Invaders](./demo/index.html)
 
### Developer's Notes and Tech Tips

- Rust [Input / Output](./dev_notes/input_output.html)
- Rust [Iterators and Combinators](./dev_notes/iterators_and_combinators.html)
- Rust [Serde / Reqwest]()
- Git [git actions]()
- hx [Helix editor]()

---

see also:

- Video: [my tech and programming heroes](./my_heroes.html)
- papers:

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
<li><a href="./coding_math/index.html">Coding Math</a></li>
<li><a href="./game_dev/index.html">Game Dev</a></li>
<li><a href="./index.html">Rust&Friends</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
