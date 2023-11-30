<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
[`<--` web-sys: Closures](./007_closures.html)

## web-sys: performance.now

*Want to profile some Rust code in the browser? No problem! You can use the performance.now() API and friends to get timing information to see how long things take.*
_ [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/examples/performance.html){target="_blank"}

[web-sys: performance.now](https://github.com/rustwasm/wasm-bindgen/tree/main/examples/performance){target="_blank"}

### setup the project

```sh
cargo new performance --lib
cd performance
mkdir -p www/js www/html
cargo add wasm-bindgen
```

edit Cargo.toml to add `crate-type`

```toml
[lib]
crate-type = ["cdylib",]
```


in `www/html/index.html` we have

```html
<!doctype html>
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>performance: nobundle</title>
  </head>
  <body>
    <p>The developer console should have timing log messages in it</p>

    <script type="module" src="../js/index.js"></script>
  </body>
</html>
```

and in `www/js/index.js`

```js
import init from "../pkg/performance.js"

async function run() {
    const wasm = await init();
}

run();
```




## Everything happens in src

```rust
// src/lib.rs
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn run() {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    console_log!("the current time (in ms) is {}", performance.now());

    let start = perf_to_system(performance.timing().request_start());
    let end = perf_to_system(performance.timing().response_end());

    console_log!("request started at {}", humantime::format_rfc3339(start));
    console_log!("request ended at {}", humantime::format_rfc3339(end));
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}
```

## Add extra dependencies

```toml
...

[dependencies]
wasm-bindgen = "0.2.88"
humantime = "2"

[dependencies.web-sys]
version = "0.3.4"
features = ['Window', 'Performance', 'PerformanceTiming']
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

Next example: [The fetch API `-->`](./009_fetch.html)

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


