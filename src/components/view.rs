use crate::backend::save_dog;
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
pub struct DogApi {
    pub message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-orange-50 via-white to-red-50",
            // Main content container
            div { class: "max-w-4xl mx-auto px-4 py-8 sm:px-6 lg:px-8",

                // Header
                div { class: "text-center mb-8",
                    h2 { class: "text-2xl sm:text-3xl font-bold text-gray-800 mb-2", "Discover Your New Best Friend! 🐕" }
                    p { class: "text-gray-600", "Click 'Save!' to add a dog to your favorites" }
                }

                // Dog image card
                div { class: "bg-white rounded-2xl shadow-2xl p-6 mb-8 border-4 border-orange-200",
                    div { class: "flex justify-center items-center",
                        div { class: "relative group",
                            img {
                                src: img_src.cloned().unwrap_or_default(),
                                class: "w-full max-w-lg h-auto rounded-xl shadow-lg transform transition-transform duration-300 group-hover:scale-105",
                                alt: "A doggo!"
                            }
                        }
                    }
                }

                // Action buttons
                div { class: "flex flex-col sm:flex-row justify-center items-center gap-4 sm:gap-6",
                    // Skip button
                    button {
                        onclick: move |_| img_src.restart(),
                        class: "w-full sm:w-auto px-8 py-4 bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold text-lg rounded-xl border-2 border-gray-300 hover:border-gray-400 active:scale-95 transition-all duration-200 shadow-md hover:shadow-lg",
                        "Skip"
                    }

                    // Save button
                    button {
                        onclick: move |_| async move {
                            let current = img_src.cloned().unwrap();
                            img_src.restart();
                            _ = save_dog(current).await;
                        },
                        class: "w-full sm:w-auto px-10 py-4 bg-gradient-to-r from-orange-500 to-red-500 hover:from-orange-600 hover:to-red-600 text-white font-bold text-lg rounded-xl border-2 border-orange-300 hover:border-orange-400 active:scale-95 transition-all duration-200 shadow-lg hover:shadow-xl",
                        "💾 Save!"
                    }
                }

                // Footer hint
                div { class: "mt-12 text-center text-gray-500 text-sm",
                    p { "Powered by Dog API 🐶" }
                }
            }
        }
    }
}