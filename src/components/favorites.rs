use crate::backend::list_dogs;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_server_future(list_dogs)?;

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-pink-50 via-white to-red-50 p-4",
            div {
                class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4",
                for (id, url) in favorites().unwrap().unwrap() {
                    div {
                        key: "{id}",
                        class: "w-full aspect-square rounded-2xl overflow-hidden shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-2xl",
                        img {
                            src: "{url}",
                            class: "w-full h-full object-cover"
                        }
                    }
                }
            }
        }
    }
}
