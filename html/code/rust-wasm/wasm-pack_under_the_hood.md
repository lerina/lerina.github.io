<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

For Debug is `--keep-debug`:

`wasm-pack build --target web --keep-debug --no-typescript --out-dir www`

---

Of interest to us, this is what wasm-pack does [under the hood](https://github.com/rustwasm/wasm-pack/blob/master/src/command/build.rs#L264).

[source: chinoto_vokro](https://www.reddit.com/r/rust/comments/kd22u5/wasmpack_dissectionhow_to_work_with_wasmbindgen/)

- step_build_wasm

```sh
cargo build --lib --release --target wasm32-unknown-unknown
```

- step_install_wasm_bindgen

"step_install_wasm_bindgen is what **justifies wasm-pack's existence** because whatever version of wasm-bindgen your crate depends on requires that you use a corresponding version of wasm-bindgen-cli to generate the bindings correctly. It will either use an existing install, download a precompiled version, or run cargo install --force wasm-bindgen-cli --version ${version} --root ${tmp_dir}" _ chinoto_vokro 

        whatever version of wasm-bindgen your crate depends on 
        requires that you use a corresponding version of wasm-bindgen-cli 
        to generate the bindings correctly.

```sh
cargo install --force wasm-bindgen-cli --version ${version} --root ${tmp_dir} 
```
- step_run_wasm_bindgen 

```sh
${wasm-bindgen-cli_path} target/wasm32-unknown-unknown/release/${crate_name}.wasm --out-dir ${crate_root}/pkg --typescript --target bundler
```

- step_run_wasm_opt

fetch a pre-built version of binaryen's wasm-opt, 
then for each file in `${crate_root}/pkg` that ends with `wasm` 

```sh
{run wasm-opt -o wasm-opt.wasm ${file} && mv wasm-opt.wasm ${file}}
```

Use wasm-pack it does a good and necessary job at catering to the various development platforms
for instance here:

```rust
Tool::WasmOpt => {
            let binaries: &[&str] = match Os::get()? {
                Os::MacOS => &["bin/wasm-opt", "lib/libbinaryen.dylib"],
                Os::Linux => &["bin/wasm-opt"],
                Os::Windows => &["bin/wasm-opt.exe"],
            };
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
<li><a href="./index.html">Learn no-bundle Wasm by example</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
