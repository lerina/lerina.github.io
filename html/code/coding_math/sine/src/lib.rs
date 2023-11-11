extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console; // usefull for debugging

use std::f64::consts::PI;

// When attached to a pub function this attribute will configure the start 
// section of the wasm executable to be emitted, executing the tagged function 
// as soon as the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")? //  get_context returns a Result<Option<Object>>
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    //context.set_fill_style(&"#0000FF".into());     
    //context.set_fill_style(&"rgb(150,50,0)".into());
    // modern way
    let color = JsValue::from_str("#ff0000");
    context.set_fill_style(&color); // rect uses fill not stroke.
    
    let init_y_pos = canvas.height() as f64 / 2.0;
    let frequency:f64 = 0.005;
    let amplitude:f64  = 100.0;

    for n in 0..canvas.width() {
        let x = n as f64 * frequency * PI; // * 2.0
        let y =  init_y_pos + x.sin() as f64 * amplitude; 
        context.fill_rect(n as f64, y , 5.0, 5.0);
    }


    context.stroke(); // outline
    context.fill(); // inside
    


    // log_1 expects one argument 
    // [more here](https://www.webassemblyman.com/rustwasm/web_sys_console_log2.html)
    console::log_2(&"Color : %s ".into(), &context.fill_style().into());
    


    Ok(())
}

