#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod store;

use crate::store::Store;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Homepage {},

    #[route("/food-list")]
    FoodList {},
}

#[component]
fn Homepage(cx: Scope) -> Element {
    render! { h1 { "Welcome home" } }
}

#[component]
fn FoodList(cx: Scope) -> Element {
    render! { h1 { "Food list" } }
}

#[component]
fn Nav(cx: Scope) -> Element {
    render! {
       nav {
           li { Link { to: Route::Homepage {}, "Home" } }
           li { Link { to: Route::FoodList {}, "Food List"} }
       }
       div { Outlet::<Route> {} }
    }
}

fn App(cx: Scope) -> Element {
    let store = Store::new("lose-fat").unwrap();

    cx.render(rsx! {
        Router::<Route> {}
    })
}

fn main() {
    dioxus_web::launch(App);
}
