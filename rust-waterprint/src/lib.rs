/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-22 22:27:48
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-26 11:33:59
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
                .dyn_into::<HtmlElement>();
            match node {
                Ok(node) => {
                    watermark(&document, &node);
                    document
                        .append_child(&node)
                        .expect("count append node to document");
                }
                Err(e) => {}
            }
        }
        None => {}
    }
}

fn watermark(dom: &Document, anchor: &HtmlElement) -> Result<(), ()> {
    let canvas = dom
        .create_element("canvas")
        .unwrap()
        .unchecked_into::<HtmlCanvasElement>();

    canvas.set_width(anchor.offset_width() as u32);
    canvas.set_height(anchor.offset_height() as u32);
    let ctx = canvas
        .get_context("2d")
        .expect("bbbbb")
        .expect("aaaa")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("xxx");
    ctx.set_font("24px Times New Roman");
    ctx.set_fill_style(&"#0000FF".into());
    ctx.set_text_align("right");
    ctx.fill_text(
        "I am waterprint",
        (canvas.width() - 20) as f64,
        (canvas.height() - 20) as f64,
    )
    .expect("xxxxx");
    let base64 = canvas.to_data_url_with_type("image/png").unwrap();
    anchor
        .style()
        .set_property(
            "background-image",
            &("url(\"".to_string() + &base64 + "\")")[..],
        )
        .expect("yyyyy");
    Ok(())
}
