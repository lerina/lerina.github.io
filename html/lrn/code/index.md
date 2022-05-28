<div class="bg_lrn"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>


- [Why rust](https://www.youtube.com/watch?v=_wy4tuFEpz0)
- [Ownership](https://www.youtube.com/watch?v=TCUBSbJENO4)
- [Shared borrow](https://www.youtube.com/watch?v=61bFe3jqi1E)
- [Mutable borrow](https://www.youtube.com/watch?v=pd7PJ6q4I3M)





Like most activities in life, perticularly in the realm of knowledge and its application, 
one should approach coding with a growth mind set 

```Rust
fn is_good() -> bool {
    true
}

fn main() {
    if is_good() {
        println!("It is good");
    } else {
        println!("It isn't good, yet");
    }
}
```

not with a fixed mindset

```rust
fn is_good() -> bool {
    true
}

fn main() {
    if is_good() {
        println!("It is good");
    } else {
        println!("It is bad");
    }
}
```

But to mitigate the progress curve it is wise to adopt a language that *stops*  the possibility 
of [double free](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory), [Null deference](https://owasp.org/www-community/vulnerabilities/Null_Dereference), and [dangling pointers](https://owasp.org/www-pdf-archive/OWASP_IL_8_Dangling_Pointer.pdf). A language that makes it difficult to leak memory.

There is such a language. _Rust performs the majority of its safety checks and memory management decisions at compile time, so that your program’s runtime performance isn’t impacted_. 

> Rust is a programming language that’s focused on safety, speed, and concurrency. Its design lets you create programs that have the performance and control of a low-level language, but with the powerful abstractions of a high-level language. These properties make Rust suitable for programmers who have experience in languages like C and are looking for a safer alternative, as well as those from languages like Python who are looking for ways to write code that performs better without sacrificing expressiveness.
_ rustbook



## Secure coding

-   back to first principles
-   teaching Rust as a first language
-   Building web3 with Rust and Wasm

## Game Dev

## Coding Math

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
<li><a href="../../../index.html">⇦ home</a></li>
<li><a href="../index.html">lerina</a></li>
<li><a href="./index.html">code</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
