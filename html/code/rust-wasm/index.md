<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
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

It is stated that NPM and webpack are optional, but complete examples to get wasm in the browser 
are hard to find. 

Can one code in Rust, compiled into wasm and integrated with plain vanilla JavaScript and HTML/css?  
Yes! But how?  

Information on programming Rust-wasm without bundlers is scarces. 

Of course despite assuming that you are following the *with-NPM-and_webpack* tutorials, one should not dismiss the official [Rust 🦀 and WebAssembly](https://rustwasm.github.io/docs/book/){target="_blank"} 
small book that describes how to use Rust and WebAssembly together.
Or MDN's [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm){target="_blank"}
Just come back to it once you've gone through a couple of examples in these pages.

Also note that the documentation for [wasm-bindgen: without-a-bundler](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html){target="_blank"}
actually has everyting you need to get started with rust-wasm without NPM, ...

Unfortunatly it assumes you followed the *rust with bundler*  tutorial.
If you don't know what you are looking at your still clueless in the end.

Here is an example

```
Without a Bundler

--target web or --target no-modules

If you're not using a bundler but you're still running code in a web browser, wasm-bindgen still supports this! For this use case you'll want to use the --target web flag. You can check out a full example in the documentation, but the highlights of this output are:

    When compiling you'll pass --target web to wasm-bindgen
    The output can natively be included on a web page, and doesn't require any further postprocessing. The output is included as an ES module.
    The --target web mode is not able to use NPM dependencies.
    You'll want to review the browser requirements for wasm-bindgen because no polyfills will be available.

The CLI also supports an output mode called --target no-modules which is similar to the web target in that it requires manual initialization of the wasm and is intended to be included in web pages without any further postprocessing. See the without a bundler example for some more information about --target no-modules.
```

What do you really do with that? Ironicaly its actually very clear once you know and don't need these kind of 
information anymore. 

Another problem is that alternative information is squatered, not always up to date and biased toward bundlers.
Findind a paragraph or two about it brings excitement to the lonely searcher.
Of note are the following:

- 2019-05-25 [WASM in Rust without NodeJS](https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c){target="_blank"}
- 2022-02-14 [Frontend Rust Without Node](https://blog.urth.org/2022/02/14/frontend-rust-without-node/){target="_blank"}
- 2022-03-10 [Rust/Wasm without npm](https://lionturkey.github.io/posts/rustwasm/rustwasm.html){target="_blank"}

> The following pages brings under one location all those bits and pieces you want to know in order to understand and build wasm stuff with Rust. 

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

Let's convert each example from wasm-bindgen to a no-bundle version.

- In Part I, we'll get a example running.
- Then in Part II, we'll study the code

But before that let's prepare our workspace, and setup our tools.

It is assumed that you have Rust on your machine and cargo ready for use.
(If not get the installer [here: rustup.rs](https://rustup.rs/){target="_blank"}).

#### 0. Get wasm-pack and something to serve our website locally

##### 0.1 wasm-pack

```sh
cargo install wasm-pack
```

wasm-pack helps you build rust-generated WebAssembly packages 
Its a useful convenience that is widely used by the Rust community.

If your interested see [wasm-pack under the hood](./wasm-pack_under_the_hood.html){target="_blank"}

##### 0.2 Get a local tiny static file server

We need something to serve your website so we can test and see what we develop on our local machine.
If you don't have one installed you can use `H`ost `T`hese `T`hings `P`lease - a basic http server 
for hosting a folder fast and simply [http](https://github.com/thecoshman/http){target="_blank"} 

```bash
cargo install https
```

Note: 
    
[trunkrs.dev](https://trunkrs.dev/){target="_blank"} is getting traction in the Rust community 
but its a much more ambitious tool and beyond our needs here.

## hello_world
 
### Converting the first example

Now we are ready to tackle the first example [`hello_world`](https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html){target="_blank"}

#### Seven steps to hello world

1. Set up your file structure
2. Edit Cargo.toml: Set the crate-type and add wasm-bindgen as a dependency.
3. Get the lib.rs code for hello_world
4. Specify type module in index.html
5. import with file extension included and Wrap the code in async/await index.js
6. build with wasm-pack
7. Run the web server and open your browser

---

> PART I. Make it run

##### 1. Set up your file structure

```sh
cargo new hello_world --lib
cd hello_world
mkdir -p www/html www/js www/css www/assets
```

You should have this file structure

```sh
.
├── Cargo.toml
├── src
│   └── lib.rs
└── www
    ├── assets
    ├── css
    ├── html
    └── js
```

Note:

    We don't really need assets and css directories for this hello world example. 
Its just to show how our file structure will be when we'll do more web oriented examples

##### 2. Edit Cargo.toml: Set the crate-type and add wasm-bindgen as a dependency.

In Cargo.toml, put `crate-type = ["cdylib"]` after `edition` entry.
And add wasm-bindgen as a dependency.

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.88"

```

Note: `wasm-bindgen-cli` was already installed by wasm-pack so we are all good to go.

##### 3. Get the lib.rs code for hello_world

We'll cut and paste and modify the examples. 
The point is to get used to convert code meant to be deployed 
with NPM-Webpack ecosystem into a play vanilla no-bundle Rust wasm code.

```rust
// src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

```

##### 4. Specify type module in index.html

Here is the first difference.

We must specify that our `index.js`  is a ES6 module.
import declarations can only be present in modules, so our `html` must use

`<script type="module" src="../js/index.js"></script>`

So our index file at `www/html/index.html` look like this:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Wasm no NPM no Webpack</title>
  <link rel="stylesheet" href="../css/styles.css">
</head>
<body>


  <script type="module" src="../js/index.js"></script>
</body>
</html>

```

##### 5. import with file extension included and Wrap the code in async/await index.js

You may still come across unhelpful old information.for our purpose, information such as

```
// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

rust
  .then(m => m.greet('World!'))
  .catch(console.error);
```

We are starting to see examples of the "modern" version 
of doing things but its still bundle specific like this: 

```JavaScript
import { greet } from './pkg';

greet('World');
```

Because we are not using a bundler, we are using ES6 flavor of JavaScript.

With the ES module import syntax, we must specify the filename 
with its extension in our `import` statement:

`import ... from "../pkg/hello_world.js";`

Where did we get `hello_world.js` from? wasm-pack gets it from our crate as specified in Cargo.toml

```toml
[package]
name = "hello_world"
...
```

`init` is the `default` import.  Its an initialization function which
will "boot" the module and make it ready to use.

`import init, ... from "../pkg/hello_world.js";`

This is also where we import the `greet` function we made public in our Rust code and accessible in our JavaScript
with `#[wasm_bindgen]`

`import init, {greet} from "../pkg/hello_world.js";`

 
Finally, we need to wrap the code in an async/await function

So our full index.js is:

```
import init, { greet } from "../pkg/hello_world.js";

async function run() {
    const wasm = await init();

    greet('World');
}

run();

```


##### 6. build with wasm-pack

```sh
wasm-pack build --target web --out-dir www/pkg
```


- ` --target web` to specify nobundle
- `--out-dir www/pkg` by default `pkg` on the same level as our `src` directory.
Its cleaner to have all our web stuff in `www`.

wasm-pack through wasm-bindgen-cli will generate the following in our `pkg` directory.

```
└── pkg
    ├── hello_world_bg.wasm
    ├── hello_world_bg.wasm.d.ts
    ├── hello_world.d.ts
    ├── hello_world.js
    └── package.json
```

##### 7. Run the web server and open your browser

You can use any file server, or follow along with `http` which we installed after wasm-pack.

You can host locally the `www` directory with `http www`.
It defaults at  http://127.0.0.1:8000

You can pass the address and port number like this:

```sh
http -a 127.0.0.1 -p 8080 www
```

Specifying our directory `www` will expose the following file structure to our server

```
www
├── assets
├── css
├── html
│   └── index.html
├── js
│   └── index.js
└── pkg
    ├── hello_world_bg.wasm
    ├── hello_world_bg.wasm.d.ts
    ├── hello_world.d.ts
    ├── hello_world.js
    └── package.json
```

Open `index.html` in a browser by pointing at [http://127.0.0.1:8080/html/]

![enjoy!](./pix/hello_world.png)

---

> PART II. Understand the Code



---

##  [console.log](./002_console_log.html)

Next example: [console.log](./002_console_log.html)

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
