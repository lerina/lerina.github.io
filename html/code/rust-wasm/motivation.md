It is stated that NPM and webpack are optional, but complete examples to get wasm in the browser 
are hard to find. 

Can one code in Rust, compiled into wasm and integrated with plain vanilla JavaScript and HTML/css?  
Yes! But how?  

Information on programming Rust-wasm without bundlers is scarces. 

MDN's [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm){target="_blank"} is the best walkthrough out there.

Of course despite assuming that you are following the *with-NPM-and_webpack* tutorials, one should not dismiss the official [Rust 🦀 and WebAssembly](https://rustwasm.github.io/docs/book/){target="_blank"} 
small book that describes how to use Rust and WebAssembly together.

Also note that the documentation for [wasm-bindgen: without-a-bundler](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html){target="_blank"}
actually has everyting you need to get started with rust-wasm without NPM, ...

Unfortunatly it assumes you followed the *rust with bundler*  tutorial.
If you don't know what you are looking at your still clueless in the end.

Here is an example

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
information anymore. 

Another problem is that alternative information is squatered, not always up to date and biased toward bundlers.
Findind a paragraph or two about it brings excitement to the lonely searcher.  
Of note are the following:

- 2018-04-04 [JavaScript to Rust and Back Again: A wasm-bindgen Tale](https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/){target="_blank"}
- 2019-05-25 [WASM in Rust without NodeJS](https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c){target="_blank"}
- 2019-06-19 [wasm by example](https://wasmbyexample.dev/home.en-us.html){target="_blank"}
- 2020-07-10 [A simple way to run Rust WebAssembly in a browser](https://www.furidamu.org/blog/2020/07/10/rust-webassembly-in-the-browser/){target="_blank"}
- 2022-02-14 [Frontend Rust Without Node](https://blog.urth.org/2022/02/14/frontend-rust-without-node/){target="_blank"}
- 2022-03-10 [Rust/Wasm without npm](https://lionturkey.github.io/posts/rustwasm/rustwasm.html){target="_blank"}


