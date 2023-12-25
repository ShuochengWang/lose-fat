#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod store;

use crate::store::Store;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let store = Store::new("lose-fat").unwrap();
    
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
