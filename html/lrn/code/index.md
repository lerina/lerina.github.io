⇦ [razafy.com](../../../index.html)  - [lerina](../index.html)  

<style>
.hover-menu {
  position: relative;
  overflow: hidden;
  margin: 8px;
  min-width: 340px;
  max-width: 480px;
  max-height: 290px;
  width: 100%;
  background: #000;
  text-align: center;
  box-sizing: border-box;
}

.hover-menu * {
  box-sizing: border-box;
}

.hover-menu img {
  position: relative;
  max-width: 100%;
  top: 0;
  right: 0;
  opacity: 1;
  transition: 0.3s ease-in-out;
}

.hover-menu div {
  position: absolute;
  top: 0;
  left: -120px;
  width: 120px;
  height: 100%;
  padding: 8px 4px;
  background: #000;
  transition: 0.3s ease-in-out;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.hover-menu div a {
  display: block;
  line-height: 2;
  color: #fff;
  text-decoration: none;
  opacity: 0.8;
  padding: 5px 15px;
  position: relative;
  transition: 0.3s ease-in-out;
}

.hover-menu div a:hover {
  text-decoration: underline;
}

.hover-menu:hover img {
  opacity: 0.5;
  right: -120px;
}

.hover-menu:hover div {
  left: 0;
  opacity: 1;
}
</style>

<figure class="hover-menu">
<img src="https://picsum.photos/id/1060/800/480.jpg"/>
<div>
<a href="#">Home</a>
<a href="#">Pricing</a>
<a href="#">About</a>
</div>
</figure>

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
