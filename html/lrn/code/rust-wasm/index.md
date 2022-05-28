<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

## Rust, Wasm and Webassembly

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
