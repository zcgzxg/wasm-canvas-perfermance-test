use js_sys;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn draw(iteration: usize, done: js_sys::Function) {
    let perf = window().unwrap().performance().unwrap();

    let _ = perf.mark("start-init");
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas
        .dyn_into::<HtmlCanvasElement>()
        .expect("#canvas is not a HtmlCanvasElement");
    let ctx = canvas
        .get_context("2d")
        .expect("get context fail")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("cast to CanvasRenderingContext2d fail");
    let _ = perf.measure_with_start_mark("wasm init", "start-init");

    let _ = perf.mark("start-draw");
    for i in 1..iteration {
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        ctx.set_fill_style(&"#000000".into());
        ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        ctx.set_stroke_style(&"#feccaa".into());
        ctx.set_line_width(3.0);
        ctx.stroke_rect(i as f64, 30.0, i as f64, 60.0);
        let _ = ctx.arc(i as f64, i as f64, (i * 20) as f64, 0.0, 6.28);
        ctx.stroke();
    }
    let _ = perf.measure_with_start_mark("wasm draw", "start-draw");

    let _ = done.call0(&JsValue::undefined());
}
