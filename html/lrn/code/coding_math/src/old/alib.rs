use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas002").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let width = 1024;
    let height = 1024;
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
    context.set_fill_style(&color); // rect uses fill not stroke.

    context.translate(0.0, height as f64 /2.0 );
    context.scale(1.0, -1.0); // direction: go up then down

    let PI2 = 31415926 *2; // Mountain then Valley

    for angle in (0..PI2).step_by(100000){ // there is only so many pixels we can see. 
        // Rust does not allow iterating over a range of f32 or f64. 
        // Instead, it forces you to use integral steps
        // ajusting the decimal point
        let angle = angle as f64 * 0.0000001  ;        
        let x = angle * 160.0;
        let y = angle.sin() * 160.0; 
        
        
        context.fill_rect(x, y, 5.0, 5.0);
   }
}
