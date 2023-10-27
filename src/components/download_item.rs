use dioxus::prelude::*;

use crate::apis::DownloadItem;
use crate::components::progress::Progress;
use bytesize::ByteSize;

#[derive(PartialEq, Props)]
pub struct DownloadItemProps<'a> {
    download_item: &'a DownloadItem,
}


pub fn DownloadItem<'a>(cx: Scope<'a, DownloadItemProps<'a>>) -> Element<'a> {
    return cx.render(rsx! {
        article { class: "flex flex-col gap-4 rounded-lg border border-gray-100 bg-white p-6 dark:border-gray-800 dark:bg-gray-900",
            h2 { class: "flex flex-row justify-end gap-2",
                div { class: "inline-flex gap-2 self-end rounded bg-green-100 p-1 text-green-600 dark:bg-green-700 dark:text-green-50",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        stroke_width: "1.3",
                        view_box: "0 0 24 24",
                        class: "h-4 w-4",
                        color: "currentColor",
                        path {
                            stroke: "currentColor",
                            stroke_width: "1.3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 20h12M12 4v12m0 0 3.5-3.5M12 16l-3.5-3.5",
                        }
                    }
                    span { class: "text-xs font-medium", "67.81 MB/s" }
                }
                div { class: "inline-flex gap-2 self-end rounded bg-blue-100 p-1 text-blue-600 dark:bg-blue-700 dark:text-blue-50",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4",
                        fill: "none",
                        stroke_width: "1.3",
                        view_box: "0 0 24 24",
                        path {
                            stroke: "currentColor",
                            stroke_width: "1.3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 20h12M12 16V4m0 0 3.5 3.5M12 4 8.5 7.5"
                        }
                    }
                    span { class: "text-xs font-medium", "30.6 MB/s" }
                }
            }
            Progress { download_item: &cx.props.download_item }
            div {
                strong { class: "block text-sm font-medium text-gray-500 dark:text-gray-400",
                    "{cx.props.download_item.download_item_name}"
                }
                p {
                    span { class: "text-2xl font-medium text-gray-900 dark:text-white mr-2",
                        "{cx.props.download_item.human_amount_left()}"
                    }
                    span { class: "text-xs text-gray-500 dark:text-gray-400",
                        " of {cx.props.download_item.human_amount_total()}"
                    }
                }
            }
        }
    });
}