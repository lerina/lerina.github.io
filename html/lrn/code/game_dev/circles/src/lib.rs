#![allow(dead_code)]
#![allow(unused_variables)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rand::prelude::*;

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub counter_clockwise: bool,
}

#[wasm_bindgen]
impl Circle {
    pub fn new(
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        counter_clockwise: bool,
    ) -> Circle {
        Circle {
            x,
            y,
            radius,
            start_angle,
            end_angle,
            counter_clockwise,
        }
    }

    pub fn rnd_new() -> Circle {
        let mut rng = thread_rng();
        //  let x: f64 = rng.gen(); // random number in range [0, 1) aka Js: Math.random()
        let radius = 5.0 * rng.gen::<f32>() * 40.0;
        let x: f32 = rng.gen_range((0.0 + radius)..(600.0 - radius));
        let y: f32 = rng.gen_range((0.0 + radius)..(600.0 - radius));
        let start_angle: f32 = 0.0;
        let end_angle: f32 = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
        let counter_clockwise: bool = rng.gen::<bool>();

        Circle {
            x,
            y,
            radius,
            start_angle,
            end_angle,
            counter_clockwise,
        }
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();
        
        if self.x <= (600.0 - self.radius) {
            self.x += 0.5
        } else {
            self.x = 0.0 + self.radius
        }
        if self.y <= (600.0 - self.radius) {
            self.y += 0.5
        } else {
            self.y = 0.0 + self.radius
        }
        if self.radius <= 90.0 {
            self.radius += 1.5
        } else {
            self.radius = 5.0 * rng.gen::<f32>() * 40.0; //rng.gen_range(10.0..90.0);
        }
    }
} //^--impl Circle

#[wasm_bindgen]
pub struct World {
    pub width: i32, // pub makes it accessible from Js
    pub height: i32,
    pub circle: Circle,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> World {
        /*        let circle = Circle::new(
        width as f32 / 2.0, height as f32 / 2.0, 70.0,
        0.0, std::f32::consts::PI * 2.0,
        false );*/
        let circle = Circle::rnd_new();
        World {
            width,
            height,
            circle,
        }
    }

    pub fn update(&mut self) {
        //log(&format!("circle.x = {}",self.circle.x));
        self.circle.update();
        
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log("Rust wasm initialized!");

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    use super::*;
    use wasm_bindgen_test::*;
    //wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn this_should_pass() {
        assert_eq!(1, 1);
    }
}
