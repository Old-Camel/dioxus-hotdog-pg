use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "fixed top-0 left-0 right-0 bg-gradient-to-r from-orange-500 to-red-500 shadow-lg z-50",
            div { class: "flex flex-row items-center justify-center gap-6 h-16 px-4",
                Link {
                    to: Route::DogView,
                    class: "hover:scale-105 transition-transform duration-200",
                    span { class: "text-2xl sm:text-3xl font-bold text-white drop-shadow-md whitespace-nowrap", "🌭 HotDog!" }
                }
                Link {
                    to: Route::Favorites,
                    class: "text-3xl hover:text-pink-300 hover:scale-110 transition-all duration-200 filter drop-shadow-md",
                    "♥️"
                }
            }
        }
        // Spacer to account for fixed nav
        div { class: "h-16" }
        Outlet::<Route> {}
    }
}
