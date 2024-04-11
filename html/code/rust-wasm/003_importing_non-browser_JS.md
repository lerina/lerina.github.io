<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>

<div class="prevnext"><div class="button left">[<-- console.log](./002_console_log.html)</div>
<div class="button right">[Working with the char type -->](./004_working_with_the_char_type.html)</div></div>

<main>

# Importing non-browser JS

*The `#[wasm_bindgen]` attribute can be used on `extern "C" { .. }` blocks to import functionality from JS. This is how the `js-sys` and the `web-sys` crates are built, but you can also use it in your own crate!*  
_ [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html){target="_blank"}

[wasm-bindgen example](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/import_js){target="_blank"}

> PART I. Make it run

## Converting Examples in 7 steps

#### 1. Make the file structure

```sh
cargo new import_js --lib
cd import_js
mkdir -p www/html www/js
```

#### 2. Edit Cargo.toml, add crate-type and wasm-bindgen dependency

```toml
[package]
name = "import_js"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]


[dependencies]
wasm-bindgen = "0.2.88"

```

#### 3. Get the code

*Cut and paste* the import-js example from github [src/lib.rs](https://github.com/rustwasm/wasm-bindgen/blob/main/examples/import_js/crate/src/lib.rs)

or the rust code in 

[wasm-bindgen: import-js](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html)

Note:

    We need to modify `#[wasm_bindgen(module = "/defined-in-js.js")]`
    as we made the commitment not to mix Rust and web code.
    Our version will have `defined-in-js.js` in the `js` directory
    with our `index.js` file. 

```rust
#[wasm_bindgen(module = "/www/js/defined-in-js.js")]
...
```

So the full code is :

```rust
// src/lib.rs

use wasm_bindgen::prelude::*;

// our webserver's  root is www
#[wasm_bindgen(module = "/www/js/defined-in-js.js")]
extern "C" {
    fn name() -> String;

    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
fn run() {
    log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"

    let x = MyClass::new();
    assert_eq!(x.number(), 42);
    x.set_number(10);
    log(&x.render());
}
```


#### 4. create the index file at `www/html/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Wasm no NPM no Webpack</title>
</head>
<body>


  <script type="module" src="../js/index.js"></script>
</body>
</html>
```

#### 5. The first js file is `index.js`


```javascript
// www/js/index.js

import init from "../pkg/import_js.js";

async function run() {
    const wasm = await init();
}

run();
```
Our second javascript file `defined-in-js.js`

```js
// www/js/defined-in-js.js

export function name() {
    return 'Rust';
}

export class MyClass {
    constructor() {
        this._number = 42;
    }

    get number() {
        return this._number;
    }

    set number(n) {
        return this._number = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}

```

#### 6. build it

```sh
wasm-pack build --target web --no-typescript --out-dir www/pkg
```

#### 7. serve it

```sh
http www
```

open `index.html`

```sh
firefox http://localhost:8000/html/
```

Open the browser at http://127.0.0.1:8000/html/  
and `ctrl-shift + I` to see the output in the browsers console log

![importing non-browser Js](./pix/import_js.png)



---

> PART II. Understand the Code

## Understand the Code


```rust
// our webserver's  root is www
#[wasm_bindgen(module = "/www/js/defined-in-js.js")]
extern "C" {
    fn name() -> String;

    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}
```

We made the commitment not to mix Rust and web code.  
As such, `defined-in-js.js` is in the `js` directory with our `index.js`file. 

The original code looks for this file next to the Cargo.toml file at the root directory with

```
#[wasm_bindgen(module = "/defined-in-js.js")]
```

We point it to `www/js/` like this

```rust
#[wasm_bindgen(module = "/www/js/defined-in-js.js")]
extern "C" {
   ...
}
...
```

- `module = "..."`   
[wasm-bindgen docs: attributes](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/module.html){target="_blank"}
 
- "*The #[wasm_bindgen] attribute can be used on extern "C" { .. } blocks to import functionality from JS.*"  
[wasm-bindgen docs: Importing non-browser JS](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html){target="_blank"}


- [wasm-bindgen docs: constructor](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/constructor.html){target="_blank"}


- [wasm-bindgen docs: method](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/method.html){target="_blank"}
- [wasm-bindgen docs: getter-and-setter](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/getter-and-setter.html){target="_blank"}


```rust
// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
```

Here we are making use of Javascript's console.log() method. 

[wasm-bindgen docs: js_namespace](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/js_namespace.html){target="_blank"}


```rust
#[wasm_bindgen(start)]
fn run() {
    log(&format!("Hello from {}!", name())); // should output "Hello from Rust!"

    let x = MyClass::new();
    assert_eq!(x.number(), 42);
    x.set_number(10);
    log(&x.render());
}
```
- The start function here should be started up automatically when the wasm module is loaded.  
[wasm-bindgen docs: start](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html){target="_blank"}


## What's next?


<div class="prevnext"><div class="button left">[<-- console.log](./002_console_log.html)</div>
<div class="button right">[Working with the char type -->](./004_working_with_the_char_type.html)</div></div>

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
