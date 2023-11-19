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

More on that [here](./motivation.html){target="_blank"}


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
mkdir -p www/html www/js
```

You should have this file structure

```sh
.
├── Cargo.toml
├── src
│   └── lib.rs
└── www
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


Our index file at `www/html/index.html` look like this:

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

Note the `type="module"` 

##### 5. import with file extension included and Wrap the code in async/await index.js

Second difference.

Our full index.js is modified to look like this:

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
wasm-pack build --target web --no-typescript --out-dir www/pkg
```


- ` --target web` to specify nobundle
- `--no-typescript` we are not using TypeScript for these examples
- `--out-dir www/pkg` by default `pkg` on the same level as our `src` directory.
Its cleaner to have all our web stuff in `www`.

wasm-pack through wasm-bindgen-cli will generate the following in our `pkg` directory.

```
└── pkg
    ├── hello_world_bg.wasm
    ├── hello_world.js
    └── package.json
```


The output of `--target web` is included as an ES module.
Thats why we endup with an ES6 flavor of JavaScript.

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
├── html
│   └── index.html
├── js
│   └── index.js
└── pkg
    ├── hello_world_bg.wasm
    ├── hello_world.js
    └── package.json
```

Open `index.html` in a browser by pointing at [http://127.0.0.1:8080/html/]

![enjoy!](./pix/hello_world.png)

---

> PART II. Understand the Code

The following is heavily indebted to MDN's [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm){target="_blank"}

<!--
For more details have a look at
[wasm-bindgen — how does it work?!](https://fitzgen.github.io/wasm-cg-wasm-bindgen/#1) by Nick Fitzgerald
-->
## Using wasm-bindgen to communicate between Rust and JavaScript

### The lib.rs file

`wasm-pack` uses `wasm-bindgen`, to provide a bridge between the types of JavaScript and Rust. 
It allows JavaScript to call a Rust API with a string, or a Rust function to catch a JavaScript exception.

"The src/lib.rs file is the root of the Rust crate that we are compiling to WebAssembly. It uses wasm-bindgen to interface with JavaScript. It imports the window.alert JavaScript function, and exports the greet Rust function, which alerts a greeting message."
[rustwasm book:](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#wasm-game-of-lifesrclibrs){target="_blank"}


`extern` tells Rust that we want to call some externally defined functions. 
`#[wasm-bindgen]` on top of it knows how to find these functions for us in JavaScript.
in this case it will glue window.alert() from the browser's JavaScript to the Rust function header
that provides us a function signature Rust can understand.

<!--

[source](https://stackoverflow.com/questions/70437614/how-does-wasm-bindgen-determine-which-bindings-to-generate){target="_blank"}
In a nutshell, the #[wasm_bindgen] macro generates executable functions that describe the necessary bindings in Javascript inside some_binary.wasm. These functions are then executed by the wasm-bindgen CLI program to generate the Javascript bindings and a stripped WebAssembly module, i.e., some_binary_bg.wasm.
-->

Whenever you want to call JavaScript functions, 
you can add them to this file in this manner, 
and wasm-bindgen takes care of setting everything up for you.


```rust
// src/lib.rs

// To communicate between Rust and JavaScript
use wasm_bindgen::prelude::*;


// Calling external functions in JavaScript from Rust
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Producing Rust functions that JavaScript can call 
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name)); //call alert function we asked for in the extern block above
}

```

`#[wasm_bindgen]` attribute, over the greet function, is not modifying an extern block, 
but a fn;  this means that we want this Rust function to be able to be called by JavaScript. 
It's the opposite of extern.

These aren't the functions we need in Rust, 
but rather the functions we're giving out to the JavaScript world.

This function is named greet, and takes one argument, a string (written &str), name. 
It then calls the alert function we asked for in the extern block above.

We use the `format!` macro to concatenate two string-literal and convert in to a String slices `&`


So the `alert` in `greet` calls the `alert` in the `extern` block, 
which is glued to `window.alert` in the browser runtime.


For the curious, have a look at 
[Design of wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/contributing/design/index.html){target="_blank"}


## index.html and index.js files

1. index.html

`import` declarations in JavaScript can only be present in modules, so our `html` must specify that our `index.js`
file is a module.

`<script type="module" src="../js/index.js"></script>`

2. index.js

You may still come across unhelpful old information for our purpose. Information such as

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


Because we are not using a bundler, we used `wasm-pack --target web --out-dir www` to compile pour code.
This will output ES6 code in `www/pkg`. 
Hence we have to use the ES module import syntax.
That is, we must specify the filename with its extension in our `import` statement:

`import ... from "../pkg/hello_world.js";`

Where did we get `hello_world.js` from?  
`wasm-pack` gets it from our crate name as specified in Cargo.toml

```toml
[package]
name = "hello_world"
...
```

There is an initialization function `init` which
will "boot" the module and make it ready to use.
Its the `default` import. 

`import init, ... from "../pkg/hello_world.js";`

Next we import the `greet` function, we made public in our Rust code and accessible in our JavaScript
with `#[wasm_bindgen]`

`import init, {greet} from "../pkg/hello_world.js";`

 
Finally, we need to wrap the code in an async/await function



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
