# Underrated gems of Rust & WebAssembly: Errors, Async, I/O 

SOURCE:

Nov 15, 2023  
[youtube: Underrated gems of Rust & WebAssembly: Errors, Async, I/O - Alberto Schiabel - EuroRust 2023](https://www.youtube.com/watch?v=KPxCsk9lX10)  
[slides](https://jkomyno-eurorust-2023.vercel.app/1)  
[github: Underrated gems of Rust & WebAssembly: Errors · Async · I/O](https://github.com/jkomyno/eurorust-2023)

## Fantastic tool: `wasm_bindgen`

0. Create the project

```sh
cargo new schiabel_talk --lib
cd schiabel_talk
```
Install the tools 

```sh
cargo install wasm-bindgen-cli
```
wasm-pack another popular tool will manage the version synchronicity between wasm-bindgen-cli and wasm-bindgen
but we are doing things the hard way here so we need to ge the latest version 

<article id="text__asides">
<header>
<h3>Specify the crate type</h3>
</header>
<aside>
<h4>What is `cdylib`?</h4>
<p>with 
`--crate-type=dylib` at the command line , 
or `#![crate_type = "dylib"]` in the Rust file (`lib.rs`) -  
A dynamic Rust library will be produced. 
The resulting dynamic library can be used 
as a dependency for other libraries and/or executables. 
This output type will create `*.so` files on Linux, 
`*.dylib` files on macOS, and `*.dll` files on Windows.
</p>
</aside>
<p>

0. Add the wasm-bindgen crate

```sh
cargo add wasm-bindgen
```

1. Specify the type of the crate in `Cargo.toml`

```toml
...

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm_bindgen = "0.2.88"
...
```


build it

```sh
cargo build --release --target wasm32-unknown-unknown 
```

result:

```sh
target/wasm32-unknown-unknown/release/
├── build
├── deps
├── examples
├── incremental
├── schiabel_talk.d
└── schiabel_talk.wasm
```

Apply wasm-bindgen to get the glue code and helper functions

```sh
wasm-bindgen --target web --out-dir ./src/wasm/ \
target/wasm32-unknown-unknown/release/schiabel_talk.wasm                               
```

result:

```sh
src/
├── lib.rs
└── wasm
    ├── schiabel_talk_bg.wasm
    ├── schiabel_talk_bg.wasm.d.ts
    ├── schiabel_talk.d.ts
    └── schiabel_talk.js

```

</p>
<footer>
<p><a href="#top"></a></p>
</footer>
</article>


2. Mark the public function of your Rust library that yo want to 
expose to the javaScript runtime, via the `#[wasm_bindgen]` macro.

```rust
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn rust_fn(arg: i32) -> String { ... }
```


## Error & panic

"Pure" Rust vs Rust/Wasm

### Result<T, E> in Rust

- Rust denotes fallible computations with functions returning `Result<T, E>`
- These functions can be safely retried later

### Failues in Rust vs JavaScript

#### Rust uses pattern matching or the `?` operator to deal with failure

```rust 
fn devide(a: i32, b: i32) -> Result<i32, &'static str> {
  if b == 0 {
    return Err("division by zero");
  }

  Ok(a / b)
}

match divide(10, 2) {
  Ok(value) => println!("Success: {value}"),
  Err(err) => println!("Failure: {err}"),
}

fn do() -> Result<i32, &'static str> {
  let value = divide(10, 2);

  Ok(value)
}

```

#### But JavaScript adopts a `try / catch` approach

```ts
function divide(a: number, b: number): number {
  if (b == 0) {
    throw new Error('division by zero');
  } 

  return a / b;
}

try {
  const result = divide(10, 2);
  console.log('Success: ${result}')
} catch (error) {
  const e = error as Error;
  console.log('Failure: ${e.message}');
}
```

- Errors should be instances of the Error class

#### `Result<T, E>` in Rust/Wasm -> JavaScript

- Err(E) isn't necessarily translated into an Error instance in JavaScript

```rust 
use wasm_bindgen::{prelude::wasm_bindgen,};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ...)]
pub struct Event { ... }

#[wasm_bindgen(js_name = "parseWithStringError")]
pub fn parse_with_string_error(event: &str) -> Result<Event, String> {
  serde_json::from_str(event).map_err(|e| e.to_string())
}
```


- Result<_, String> results in a string being thrown

```ts
import * as wasm from '../src/wasm/demo_errors.js'

test('throws a `string`', () => {
  try {
    wasm.parseErrorWithStringError('{ "event": "EuroRust" }')
  } catch (error) {
    const e = error as string

    assert(typeof e === 'string');
    expect(e).toMatchInlineSnapshot('"missing field ..."')
  }
})
```

- `Result<_, JsError>` results in an Error instance being thrown

```rust
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ...)]
pub struct Event { ... }

#[wasm_bindgen(js_name = "parseWithError")]
pub fn parse_with_error(event: &str) -> Result<Event, JsError> {
  serde_json::from_str(event).map_err(|e| JsError::from(e))
}
```



- `Result<_, JsError>` results in an Error instance being thrown

```ts
import * as wasm from '../src/wasm/demo_errors.js'

test('thrown an `Error`', () => {
  try {
    wasm.parseWithError('{ "event": "EuroRust" }')
  } catch (error) {
    const e = error as Error;

    assert(e instanceof Error);
    expect(e.name).toEqual('Error');
    expect(e.message).toMatchInlineSnapshot('"missing field ... "')
  }
})
```


- `Result<_, dyn Error + Display>` also works fine

```rust
pub(crate) struct CustomError(pub serde_json::error::Error);

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formater) -> std::fmt::Result {
    write!(f, "[CustomError] ")?;
    
    self.0.fmt(f)
  }
}
```

- `wasm_bindgen` implements From<E> for JsError where `E: std::error::Error`










































--- 

See also:  
[2023 - workshop src](https://github.com/jkomyno/workshop-rust-wasm  
[2022 - No free lunch: Limits of Wasm as a bridge from Rust to JS](https://github.com/jkomyno/eurorust-2022)
[youtube: No free lunch: Limits of Wasm as a bridge from Rust to JS](https://www.youtube.com/watch?v=IBzNKh5WGW4)

