/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-22 22:27:48
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-23 00:50:47
 */
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement};

#[wasm_bindgen]
pub fn generate_watermark(anchor: Option<String>) {
    let window = web_sys::window().expect("window is expected");
    let document = window.document().expect("document is expected");
    match anchor {
        Some(node) => {
            let node = document
                .query_selector(&node)
                .expect("I need a node")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            watermark(&document, &node);
            document
                .append_child(&node)
                .expect("count append node to document");
        }
        None => {}
    }
}

fn watermark(dom: &Document, anchor: &HtmlElement) {
    let canvas = dom
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(anchor.client_width() as u32);
    canvas.set_height(anchor.client_height() as u32);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_font("24px Times New Roman");
    ctx.set_fill_style(&"#0000FF".into());
    ctx.set_text_align("right");
    ctx.fill_text(
        "I am waterprint",
        (canvas.width() - 20) as f64,
        (canvas.height() - 20) as f64,
    );
    let base64 = canvas.to_data_url_with_type("image/png").unwrap();
    // anchor
    //     .append_child(&canvas)
    //     .expect("could not append canvas");
    anchor.style().set_property(
        "background-image",
        &("url(\"".to_string() + &base64 + "\")")[..],
    );
}
