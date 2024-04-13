<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>

<div class="prevnext"><div class="button left">[<-- hello_world](./001_hello_world.html) </div>
<div class="button right">[Importing non-browser JS -->](./003_importing_non-browser_JS.html) </div></div>

<main>

# console.log

*This example shows off how to use console.log in a variety of ways, all the way from bare-bones usage to a println!-like macro with web_sys.*  
_ [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/examples/console-log.html){target="_blank"}

[wasm-bindgen example](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/console_log){target="_blank"}

> PART I. Make it run

## Converting Examples in 7 steps

#### 1. Set up your file structure

```sh
cargo new console_log --lib
cd console_log
mkdir -p www/html www/js
```

#### 2. Edit Cargo.toml, add crate-type and wasm-bindgen dependency

```toml
[package]
name = "console_log"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]


[dependencies]
wasm-bindgen = "0.2.88"

```

#### 3. get the code 

Cut and paste the console-log example [src/lib.rs](https://rustwasm.github.io/wasm-bindgen/examples/console-log.html){target="_blank"}


```rust
// src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

// And finally, we don't even have to define the `log` function ourselves! The
// `web_sys` crate already has it defined for us.

fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}

```

Note: 

Since `use web_sys::console;` brings into scope console from the web-sys crate we need to 
declare it in our Cargo dependencies.

```toml
...
[dependencies]
wasm-bindgen = "0.2.88"
web-sys = { version = "0.3.65", features = ['console'] }`
```

#### 4. create the index file at `www/html/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>nobundle: console_log</title>
</head>
<body>


  <script type="module" src="../js/index.js"></script>
</body>
</html>

```

#### 5. The js file 

We could have written the file like this

```javascript
import init, { run as run_me } from "../pkg/console_log.js";

async function run() {
    const wasm = await init();
    run_me();
}

run();
```

But because we used `#[wasm_bindgen(start)]` in `src/lib.rs`

```rust
...
#[wasm_bindgen(start)]
fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

...
```

our js file can run directly

```javascript
import init from "../pkg/console_log.js";

async function run() {
    const wasm = await init();

}

run();

```

#### 6. build it

```sh
wasm-pack build --target web --no-typescript --out-dir www/pkg
```

#### 7. serve it

```sh
http www
```

Open the browser at http://127.0.0.1:8000/html/  

```sh
firefox http://localhost:8000/html/
```

and `ctrl-shift + I` to see the output in the browsers console log

![Console log](./pix/console_log.png)


## Q&A

<div class="alt-pre">How would you generate this clean <br/> file structure?
&#x2BB1; &#x21AA; ↪   ➡️ &#10132; <pre >
.
├── Cargo.toml
├── src
│   └── lib.rs
└── www
    ├── html
    └── js
</pre>
<pre>
cargo new hello_world --lib

cd hello_world

mkdir -p www/html www/js
</pre>
</div>

---

> PART II. Understand the Code

## Understand the Code



## What's next?

There is nothing specific for nobundle in the [Small wasm files](https://rustwasm.github.io/wasm-bindgen/examples/add.html){target="_blank"} example so we'll pass it.

Now the [Without a Bundler](https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html){target="_blank"}  section makes sense. 

[Synchronous Instantiation](https://rustwasm.github.io/wasm-bindgen/examples/synchronous-instantiation.html){target="_blank"} We'll stick with the default method, asynchronously initializing a module gets all our examples running.

[Converting WebAssembly to JS](https://rustwasm.github.io/wasm-bindgen/examples/wasm2js.html){target="_blank"}. 
Nothing specific to nobundle, we pass.


<span class="button">[Example list](./index.html#list)</span>

<div class="prevnext"><div class="button left">[<-- hello_world](./001_hello_world.html) </div>
<div class="button right">[Importing non-browser JS -->](./003_importing_non-browser_JS.html) </div></div>

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
<li><a href="../index.html">hello_world</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
