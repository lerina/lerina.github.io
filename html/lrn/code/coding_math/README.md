
## Sine [f32](https://doc.rust-lang.org/std/primitive.f32.html)

Computes the sine of a number (in radians)

```rust
let x = std::f32::consts::FRAC_PI_2;

let abs_difference = (x.sin() - 1.0).abs();

assert!(abs_difference <= f32::EPSILON);
```

## Loop demo [synth function](https://github.com/inokawa/rust-wasm-example/blob/master/rs_audio/src/lib.rs)

```rust
use std::f32::consts::PI;

pub fn synth(freq: f32, sec: u32, sample_rate: u32) -> Vec<f32> {
    let mut wave: Vec<f32> = Vec::new();
    for i in 0..sample_rate * sec {
        let v = (i as f32 * freq * PI * 2.0 / sample_rate as f32).sin();
        wave.push(v);
    }
    wave
}

 ```
