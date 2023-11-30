<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

[`<--` web-sys: performance.now](./008_performance.html)

## web-sys: performance.now

*This example uses the fetch API to make an HTTP request to the GitHub API and then parses the resulting JSON.*
_ [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html){target="_blank"}

[web-sys: The fetch API](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/fetch){target="_blank"}


### setup the project

```sh
cargo new fetch --lib
cd fetch
mkdir -p www/js www/html
cargo add wasm-bindgen js-sys wasm-bindgen-futures

```

edit Cargo.toml to add `crate-type`

```toml
[lib]
crate-type = ["cdylib",]

...


[dependencies.web-sys]
version = "0.3.66"
features = [
  'Headers',
  'Request',
  'RequestInit',
  'RequestMode',
  'Response',
  'Window',
]
```


in `www/html/index.html` we have

```html
<!doctype html>
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>fetch: nobundle</title>
  </head>
  <body>
    <p>The developer console should have log messages in it</p>

    <script type="module" src="../js/index.js"></script>
  </body>
</html>
```

and in `www/js/index.js`

```js

import init, {run as rs_run} from "../pkg/fetch.js"

async function run() {
    const wasm = await init();
    const data = await rs_run("rustwasm/wasm-bindgen");
    console.log(data);

    console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
    console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
}

run();

/*
const rust = import('./pkg');

rust
  .then(m => {
      return m.run("rustwasm/wasm-bindgen").then((data) => {
          console.log(data);

          console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
          console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
      })
  })
  .catch(console.error);
*/
```




## Rust/wasm side

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn run(repo: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
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
<li><a href="../index.html">hello_world</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
