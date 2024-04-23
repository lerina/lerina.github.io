<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
# Learn Rust/Wasm without NPM and webpack
 

> These pages are dedicated to Understanding and developing in Rust/wasm without bloat or bundles.

If you need to learn Rust or unrust your Rust these are you best free options.

- [The book](https://doc.rust-lang.org/book/title-page.html){target="_blank"}
- The best youtube resource to learn Rust is [LetsGetRusty](https://www.youtube.com/c/LetsGetRusty/playlists){target="_blank"}

## Motivation
 
I drank the Cool-aid and fell for the Rust evangelism.  
I love the language. It make programming fun again. 

When webassembly started to be a thing, Rust kept popping up as the Language of choice:

- to build webassembly/JavaScript apps for the web or 
- target some specific module that need speeding, memory safety, or both.

Most Rust-wasm tutorials, however, lean heavily on "NPM and webpack", just to get a "hello world".

More on that [here](./motivation.html){target="_blank"}

> The following pages brings under one location all those bits and pieces you want to know in order to understand and build wasm stuff with Rust. 

More specifically the intent is to address these wishes:

- Rust developers should be able to produce WebAssembly packages for use in JavaScript without requiring a Node.js development environment
- JavaScript developers should be able to use WebAssembly without requiring a Rust development environment

source: [Hello wasm-pack!](https://hacks.mozilla.org/2018/04/hello-wasm-pack/){target="_bank"}

## no-bundle Wasm by example

### The tools

Webassembly is still a moving target and some tools and convenient crates make the experience more appealing.

In the following pages we'll dive into understanding how to transform most webpack, NPM loaded Rust/wasm tutorial into lean no-bloat rust-wasm with no-bundle.

We'll use and get familliar with the following tools and crates: 

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/){target="_blank"}, 
[docs](https://rustwasm.github.io/docs/wasm-pack/introduction.html){target="_blank"}  
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen){target="_blank"}, 
[docs](https://rustwasm.github.io/docs/wasm-bindgen/){target="_blank"}  
- [js-sys](https://lib.rs/crates/js-sys){target="_blank"} & [web-sys](https://lib.rs/crates/web-sys){target="_blank"}  
[js-sys example](https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-wasm.html){target="_blank"}, 
[web-sys examples](https://rustwasm.github.io/wasm-bindgen/examples/dom.html){target="_blank"}  

<!-- 
[trunkrs.dev](https://trunkrs.dev/){target="_blank"}  
-->


### Let the tutorial begin!

Wasm without npm and bundlers is actually quite simple. Unfortunately it's 
not easy to find **complete examples** on the web.

We'll convert each example from wasm-bindgen to a no-bundle version.

- In Part I, we'll get a example running.
- Then in Part II, we'll study the code

But before that let's prepare our workspace, and setup our tools.

It is assumed that you have Rust on your machine and cargo ready for use.
(If not get the installer [here: rustup.rs](https://rustup.rs/){target="_blank"}).

#### 0. Get wasm-pack and something to serve our website locally

##### 0.1 Install wasm-pack

```sh
cargo install wasm-pack
```

wasm-pack helps you build rust-generated WebAssembly packages 
Its a useful convenience that is widely used by the Rust community.

If your interested see [wasm-pack under the hood](./wasm-pack_under_the_hood.html){target="_blank"}

##### 0.2 Install a local tiny static file server

We need something to serve your website so we can test and see what we develop on our local machine.
If you don't have one installed you can use `H`ost `T`hese `T`hings `P`lease - a basic http server 
for hosting a folder fast and simply [http](https://github.com/thecoshman/http){target="_blank"} 

```bash
cargo install https
```

Note: 
    
[trunkrs.dev](https://trunkrs.dev/){target="_blank"} is getting traction in the Rust community 
but its a much more ambitious tool and beyond our needs here.
 
### Converting examples

#### Seven steps to hello world

1. Set up your file structure
2. Edit Cargo.toml: Set the crate-type and add wasm-bindgen as a dependency.
3. Get the lib.rs code for hello_world
4. Specify type module in index.html
5. import with file extension included and Wrap the code in async/await index.js
6. build with wasm-pack
7. Run the web server and open your browser


Let's see that in practice with the first example: [wasm Hello world](./001_hello_world.html)

---

## Example list{id="list"}

<!-- 

## One more thing (Shrink the size)

This is Optional, and will only do it this once for these examples series.

Wasm files are notorious for being large by default.
You can reduce the size with [wasm-gc](https://github.com/alexcrichton/wasm-gc){target="_blank"}

The wasm-pack (and wasm-bindgen) project will already run this by default for you, so there's no need to run it again. 


Using the official way through Cargo.toml:

```
[profile.release]
# optimization for size ( more aggressive )
opt-level = 'z'

# optimization for size
# opt-level = 's'

# link time optimization using using whole-program analysis
lto = true
#
wasm-opt = ['-Oz']
```
and building with the `--release` flag

```sh
wasm-pack build --release --target web --no-typescript --out-dir www/pkg
```

Original size: 1.8M hello_world.wasm
New size: 109k hello_world.wasm

However using wasm-gc ourself we get a mush smaller size


```sh
wasm-gc target/wasm32-unknown-unknown/release/hello_world.wasm 
```
Original size: 1.8M hello_world.wasm
New size: 28k hello_world.wasm


humm.
-->

--- 

Move on to the first example: [wasm Hello world](./001_hello_world.html)

</main>

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
<li><a href="../../../../index.html">⇦ home</a></li>
<li><a href="../index.html">code</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
