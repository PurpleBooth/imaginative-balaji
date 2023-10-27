use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;


pub fn Footer(cx: Scope<'_>) -> Element<'_> {
    return cx.render(
        rsx! {
            footer { class: "bg-white",
                div { class: "mx-auto max-w-screen-xl px-4 pb-8 pt-16 sm:px-6 lg:px-8 lg:pt-24 text-center",
                    p { class: "text-xs text-gray-500", "© Is bullshit." }
                }
            }
        }
    );
}
