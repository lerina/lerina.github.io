<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
[`<--` console.log](./002_console_log.html)

## Importing non-browser JS.

1. Make the file structure

```
cargo new import_js --lib
cd import_js
mkdir -p www/html www/js
```

2. Edit Cargo.toml, add crate-type and wasm-bindgen dependency

```toml
[package]
name = "import_js"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.88"

```

3. cut and paste the import-js example from github [src/lib.rs](https://github.com/rustwasm/wasm-bindgen/blob/main/examples/import_js/crate/src/lib.rs)

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


4. create the index file at `www/html/index.html`:

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

5. The first js file is `index.js`


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

6. build it

```sh
wasm-pack build --target web --out-dir www/pkg
```

7. serve it

```sh
http www
```

Open the browser at http://127.0.0.1:8000/html/  
and `ctrl-shift + I` to see the output in the browsers console log

![importing non-browser Js](./pix/import_js.png)

## What's next?



Next example: [Working with the char type `-->`](./004_working_with_the_char_type.html)

  
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
