<div class="bg_lrn"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

## Coding Math

### Episode 001

<canvas id="canvas001" height="250" width="350"></canvas>

<!-- Note the usage  of `type=module` here as this is an ES6 module -->
<script type="module">
  // Use ES module import syntax to import functionality from the module
  // that we have compiled.
  //
  // Note that the `default` import is an initialization function which
  // will "boot" the module and make it ready to use. Currently browsers
  // don't support natively imported WebAssembly as an ES module, but
  // eventually the manual initialization won't be required!
  import init, {} from './ep001/canvas.js';

  async function run() {
    // First up we need to actually load the wasm file, so we use the
    // default export to inform it where the wasm file is located on the
    // server, and then we wait on the returned promise to wait for the
    // wasm to be loaded.
    //
    // It may look like this: `await init('./pkg/canvas_bg.wasm');`,
    // but there is also a handy default inside `init` function, which uses
    // `import.meta` to locate the wasm file relatively to js file.
    //
    await init();
  }

  run();
</script>

```rust
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Math;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let width = 900;
    let height = 740;
    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.fill_rect(0.0,0.0,width as f64, height as f64);
    
    let color = JsValue::from_str("#ff0000");
    context.set_stroke_style(&color);


    for _ in 0..100 {
        context.begin_path();
        
        // Draw line.
        context
            .move_to(Math::random() * width as f64, Math::random() * height as f64);
        context
            .line_to(Math::random() * width as f64, Math::random() * height as f64);
            

           context.stroke();
    }
}
```

---

### Episode 002

<canvas id="canvas002" height="250" width="350"></canvas>
<script type="module">
  import init, {} from './ep002/canvas.js';

  async function run() {

    await init();
  }

  run();
</script>

<!--  adapted from [source](https://github.com/RustAudio/rust-portaudio/blob/master/examples/sine.rs) -->
```rust
use std::f64::consts::PI;

const TABLE_SIZE: usize = 360;

// Initialise sinusoidal wavetable.
let mut sine = [0.0; TABLE_SIZE];
for i in 0..TABLE_SIZE {
    sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
}

```
---

### Interlude: Smiley

<canvas id="face" height="250" width="350"></canvas>
<script type="module">
  import init, {} from './face/canvas.js';

  async function run() {

    await init();
  }

  run();
</script>

```rust
use std::f64; //for PI
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("face").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
```

<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://razafy.com/img/github32px.png){.link .glow}
  </a>
</footer>

<script src="https://razafy.com/js/toc.js"></script>
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
<li><a href="../../index.html">lerina</a></li>
<li><a href="../index.html">code</a></li>
<li><a href="./index.html">coding math</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
