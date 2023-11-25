<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
[`<--` Working with the char type](./004_working_with_the_char_type.html)

---

## js-sys: WebAssembly in WebAssembly

[wasm-bindgen example](https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-wasm.html){target="_blank"}

### setup the project

```sh
cargo new wasm-in-wasm --lib
cd wasm-in-wasm
cargo add wasm-bindgen
```

edit Cargo.toml to add `crate-type`

```toml
[lib]
crate-type = ["cdylib",]
```

#### make the wasm file to be used by wasm-in-wasm later

The code for `add.wasm`

```
use wasm_biindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}
```

Temporarly rename the crate `add`


```toml
[package]
name = "add"
#name = "wasm-in-wasm"
...
[lib]
crate-type = ["cdylib",]
```

Build the wasm file
```sh
wasm-pack build --release --target web --out-dir www/pkg
```

move and rename the `www/pkg/add_bg.wasm` to `src/add.wasm` 
 
and clean the project for our real code

```sh
mv target/wasm32-unknown-unknown/release/add_bg.wasm ./src/add.wasm
cargo clean
'rm -fr www/pkg'
```

### The real code

Now that we have the wasm file to be used in `lib.rs
we can revert to real name in Cargo.toml

```toml
[package]
name = "wasm-in-wasm"
...
[lib]
crate-type = ["cdylib,"]
```

in `www/html/index.html` we have

```html
<!DOCTYPE html>
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>Using wasm in Rust</title>
  </head>
  <body>
    <p>Everything happens in rust/wasm <br/ >
    The developer console should have messages in it</p>

    <script type="module" src="../js/index.js"></script>
  </body>
</html>
```

and in `www/js/index.js`

```js
import init from "../pkg/wasm_in_wasm.js"

init();

/* 
    //async is handled directly in lib.rs
    async function run() {
        const wasm = await init();
    }

    run();
*/
```

Note: 

The build outputs the file as `wasm_in_wasm.js` not `wasm-in-wasm.js`
we've seen that before (ie: the crate `wasm-bindgen` is used as `wasm_bindgen`)

### Everything happens in src


```rust
// src/lib.rs

use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("add.wasm"); // path relative to lib.rs

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new wasm module directly");

    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let add = Reflect::get(c.as_ref(), &"add".into())?
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console_log!("1 + 2 = {:?}", three);
    let mem = Reflect::get(c.as_ref(), &"memory".into())?
        .dyn_into::<WebAssembly::Memory>()
        .expect("memory export wasn't a `WebAssembly.Memory`");
    console_log!("created module has {} pages of memory", mem.grow(0));
    console_log!("giving the module 4 more pages of memory");
    mem.grow(4);
    console_log!("now the module has {} pages of memory", mem.grow(0));

    Ok(())
}

#[wasm_bindgen(start)]
fn run() {
    spawn_local(async {
        run_async().await.unwrap_throw();
    });
}
```

Note:

We need to add the two new crates brought into scope 

```sh
cargo add js-sys
cargo add wasm-bindgen-futures
```

## build and serve

```sh
wasm-pack build --target web --no-typescript --out-dir www/pkg

http www
```

open `index.html`

```sh
firefox http://localhost:8000/html/
```

---

## What's next?

Next example: [web-sys: DOM hello world `-->`](./006_DOM.html)

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
