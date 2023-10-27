use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use crate::apis::DownloadItem;
use smooth::Smooth;

#[derive(PartialEq, Props)]
pub struct ProgressProps<'a> {
    download_item: &'a DownloadItem
}


pub fn Progress<'a>(cx: Scope<'a, ProgressProps<'a>>) -> Element<'a> {
    return cx.render(rsx! {
        div {
            span {
                id: "ProgressLabel{cx.props.download_item.download_item_id}",
                class: "sr-only",
                "Loading"
            }
            span {
                aria_labelledby: "ProgressLabel{cx.props.download_item.download_item_id}",
                role: "progressbar",
                aria_valuenow: "{cx.props.download_item.download_percentage_complete}",
                class: "relative block rounded-full bg-gray-200",
                span { class: "absolute inset-0 flex items-center justify-center text-[10px]/4",
                    span { class: "font-bold text-white",
                        " {cx.props.download_item.download_percentage_complete.smooth_str()}% "
                    }
                }
                span {
                    style: "width: {cx.props.download_item.download_percentage_complete}%",
                    class: "block h-4 rounded-full bg-indigo-600 text-center"
                }
            }
        }
    });
}