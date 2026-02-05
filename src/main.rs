mod backend;
mod components;

use components::{DogView, Favorites, NavBar};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
