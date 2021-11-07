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
