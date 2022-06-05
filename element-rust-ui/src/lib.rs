/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-05 18:48:28
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-05 23:26:18
 */
use wasm_bindgen::prelude::*;
// use web_sys::Element;

pub trait Component {
    fn render(&self) -> Result<(), JsValue>;
}

struct ElementSelectOption {
    label: String,
    value: String,
}

struct ElementSelect {
    value: String,
    placeholder: String,
    width: u32,
    height: u32,
    options: Option<Vec<ElementSelectOption>>,
}

struct ElementInput {
    value: String,
    placeholder: String,
    width: u32,
    height: u32,
}

// struct ElementRadio {
//     value: String,
//     checked: bool,
//     width: u32,
//     height: u32,
// }

impl Component for ElementSelect {
    fn render(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let dom = document.create_element("div")?;
        dom.set_inner_html(&self.value);
        body.append_child(&dom)?;
        Ok(())
    }
}

// impl Component for ElementRadio {
//     fn render(&self) -> Result<Element, JsValue> {
//         let window = web_sys::window().expect("no global `window` exists");
//         let document = window.document().expect("should have a document on window");
//         // let body = document.body().expect("document should have a body");

//         let dom: Result<Element, JsValue> = document.create_element("radio");
//         dom
//     }
// }

impl Component for ElementInput {
    fn render(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let dom = document.create_element("div")?;
        dom.set_inner_html(&self.value);
        body.append_child(&dom)?;
        Ok(())
    }
}

// pub struct Root<T: Component> {
//     pub components: Vec<T>,
// }

// trait RootComponent {
//     fn render(&self);
// }

// impl<T> RootComponent for Root<T>
// where
//     T: Component,
// {
//     fn render(&self) {
//         for component in self.components.iter() {
//             component.render();
//         }
//     }
// }

#[wasm_bindgen]
pub fn run() {
    let select = ElementSelect {
        value: String::from("I am select"),
        width: 40,
        height: 60,
        placeholder: String::from("请选择要查询的值"),
        options: Some(vec![]),
    };
    let input = ElementInput {
        value: String::from("I am input"),
        width: 40,
        height: 60,
        placeholder: String::from("请输入要查询的值"),
    };
    let res1 = input.render();
    let res2 = select.render();
}
