<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

[`<--` web-sys: performance.now](./008_performance.html)

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
