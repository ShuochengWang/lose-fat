#![allow(non_snake_case)]
use dioxus::prelude::*;

mod store;

use crate::store::Store;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let store = Store::new("lose-fat").unwrap();

    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
