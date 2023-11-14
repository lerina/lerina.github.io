<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
> These pages are dedicated to Understanding and developing in Rust/wasm without bloat or bundles.

If you need to learn Rust or unrust your Rust these are you best free options.

- [The book](https://doc.rust-lang.org/book/title-page.html)
- The best youtube resource to learn Rust is [LetsGetRusty](https://www.youtube.com/c/LetsGetRusty/playlists)

## Motivation
 
Sometime back Rust kept popping up on my radar.  
I drank the Cool-aid and fell for the Rust evangelism.  
I love the language. It make programming fun again. 
When webassembly started to be a thing, once again Rust kept popping up as the Language of choice
to build webassembly/JavaScript apps for the web or target some specific module that need speeding, 
memory safety, or both.

To my dismay however, most tutorial including the official one lean heavily on "NPM and co". 
I just wanted to wrap my head around the Rust thing, I didn't even have an up to date Node on my system.

Do I really need all these things to just have something writen in Rust, compiled into wasm and integrated with 
plain vanilla JavaScript and HTML/css?
Good news, the answer is No!  
Less joyful news however is that information on programming Rust-wasm without bloat is scarces. 
Findind a paragraph or two about it brings excitement to the lonely searcher.

- 2019-05-25 [WASM in Rust without NodeJS](https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c)
- 2022-02-14 [Frontend Rust Without Node](https://blog.urth.org/2022/02/14/frontend-rust-without-node/)
- 2022-03-10 [Rust/Wasm without npm](https://lionturkey.github.io/posts/rustwasm/rustwasm.html)

Of course one should not dismiss the official [Rust 🦀 and WebAssembly] small book that describes how to use Rust and WebAssembly together.
Or MDN's [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

[wasm-bindgen: without-a-bundler](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)
Actually has everyting you need to get started with rust-wasm without NPM, ...
Unfortunatly if you don't know what you are looking at your still clueless in the end.

here is an example

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
information. 

Webassembly is still a moving target and some tools and convenient crates make the experience more appealing.

In the following pages we'll dive into understanding how to transform most webpack, NPM loaded Rust/wasm tutorial into lean no-bloat rust-wasm with no-bundle.

We'll use and get familliar with the following tools and crates: 
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)  
[wasm-pack docs](https://rustwasm.github.io/docs/wasm-pack/introduction.html)  
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)  
[wasm-bindgen docs](https://rustwasm.github.io/docs/wasm-bindgen/)  
- [js-sys](https://lib.rs/crates/js-sys) & [web-sys](https://lib.rs/crates/web-sys)  
[wasm-bindgen: js-sys example](https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-wasm.html)  
[wasm-bindgen: web-sys examples](https://rustwasm.github.io/wasm-bindgen/examples/dom.html)  

<!-- 
[trunkrs.dev](https://trunkrs.dev/)  
-->

### Learn Wasm

#### The tools

#### My no-bloat workflow

Wasm without npm and bundlers is actually quite simple. Unfortunately it's 
not easy to find complete examples on the web.

- Get the canvas smiley face example from wasm-bindgen.

```bash
git clone https://github.com/rustwasm/wasm-bindgen/
cd wasm-bindgen/examples/canvas
```

- modify index.js

from 

```javascript
import('./pkg')
  .catch(console.error);
```

to

```javascript
import init from './pkg/canvas.js';

init()
    .catch(console.error);
```

- In index.html

add a script tag to bring into scope and specifies that index.js is a module

```{.html .numberLines}
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <canvas id="canvas" height="150" width="150"></canvas>
  
    <script type="module" src="./index.js"></script>
  </body>
</html>
```

- Compile using wasm-pack

```bash
wasm-pack build --target web
```

- Finaly 

install something to serve your website if you don't have one installed .
such as 
[http](https://github.com/thecoshman/http)  
or [devserver](https://github.com/kettle11/devserver)

```bash
cargo install https
```

and serve

```bash
http ./

```

####

I like to keep anything non-rust in `www` so all my wasm project 
uses this simple script to build and serve.

```bash
#!/bin/sh

## pre-req a web server
# cargo install https

## exit on error and  prints each executed command
set -ex

## compile for plain vanilla no javascript framework 
wasm-pack build --target web --out-dir www/pkg

## display link for easy access
echo "Serving at: http://127.0.0.1:8080/html/"

## run the web server
http -a 127.0.0.1 -p 8080 www
```

### Webassembly

- the book
- A fun dive into Webassembly 
- other `resources` elsewhere

```rust
// src/browser.rs

use anyhow::{anyhow, Result};
use std::future::Future;
use wasm_bindgen::closure::{Closure, WasmClosureFnOnce, WasmClosure};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, 
    HtmlImageElement, Response, Window, Element, HtmlElement,};
use js_sys::ArrayBuffer;



// It's a macro that allows you to log in to the console with log!
// using a syntax such as the format! function.
// Taken from https://rustwasm.github.io/book/game-of-life/debugging.html
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to CanvasRenderingContext2d",
                element
            )
        })
}

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("error fetching {:#?}", err))
}


pub async fn fetch_response(resource: &str) -> Result<Response> {
    fetch_with_str(resource)
                        .await?
                        .dyn_into()
                        .map_err(|err| anyhow!("error converting fetch to Response {:#?}", err))
}


pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp = fetch_response(json_path).await?;
    JsFuture::from( resp.json()
                        .map_err(|err| anyhow!("Could not get JSON from response {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("error fetching JSON {:#?}", err))
}


pub async fn fetch_array_buffer(resource: &str) -> Result<ArrayBuffer> {
    let array_buffer = fetch_response(resource)
                        .await?
                        .array_buffer()
                        .map_err(|err| anyhow!("Error loading array buffer {:#?}", err))?;

    JsFuture::from(array_buffer)
                        .await
                        .map_err(|err| anyhow!("Error converting array buffer into a future {:#?}", err))?
                        .dyn_into()
                        .map_err(|err| anyhow!("Error converting raw JSValue to ArrayBuffer {:#?}", err))
}



/*
This function is just a wrapper around HtmlImageElement ; there's not much to
explain. In the future, we may decide we want our own type for images, but for now,
we'll stick with the browser-provided type.
*/
pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new().map_err(|err| anyhow!("Could not create HtmlImageElement: {:#?}", err))
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;
pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    closure_wrap(Box::new(f))
}

pub fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

/*
We just mimic the exact same type signature of the
Closure::once function from wasm_bindgen .
*/
pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}


pub fn now() -> Result<f64> {
    Ok(window()?
        .performance()
        .ok_or_else(|| anyhow!("Performance object not found"))?
        .now())
}


//------------- UI
pub fn draw_ui(html: &str) -> Result<()> {
    find_ui()?
        .insert_adjacent_html("afterbegin", html)
        .map_err(|err| anyhow!("Could not insert html {:#?}", err))
}

pub fn hide_ui() -> Result<()> {
    let ui = find_ui()?;

    if let Some(child) = ui.first_child() {
        ui.remove_child(&child)
            .map(|_removed_child| ())
            .map_err(|err| anyhow!("Failed to remove child {:#?}", err))
            .and_then(|_unit| {
                canvas()?
                    .focus()
                    .map_err(|err| anyhow!("Could not set focus to canvas! {:#?}", err))
            })
    } else {
        Ok(())
    }
}

fn find_ui() -> Result<Element> {
    document().and_then(|doc| {
        doc.get_element_by_id("ui")
            .ok_or_else(|| anyhow!("UI element not found"))
    })
}

pub fn find_html_element_by_id(id: &str) -> Result<HtmlElement> {
    document()
        .and_then(|doc| {
            doc.get_element_by_id(id)
                .ok_or_else(|| anyhow!("Element with id {} not found", id))
        })
        .and_then(|element| {
            element
                .dyn_into::<HtmlElement>()
                .map_err(|err| anyhow!("Could not cast into HtmlElement {:#?}", err))
        })
}

//===============================
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_error_loading_json() {
        let json = fetch_json("not_there.json").await;
        assert_eq!(json.is_err(), true);
    }
}

```


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
