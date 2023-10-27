use std::future::join;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::ServerFnError;

use crate::apis::get_qbt_items;
use crate::apis::get_sab_items;
use crate::apis::DownloadItem;
use crate::components::download_item::DownloadItem;
use crate::components::header::Header;
use crate::components::footer::Footer;

pub fn app(cx: Scope<'_>) -> Element<'_> {
    let download_items = use_state::<Option<Result<Vec<DownloadItem>, ServerFnError>>>(cx, || None);

    let ws = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let download_items = download_items.clone();
        async move {
            loop {
                let items = get_download_items().await;
                download_items.set(Some(items));

                gloo_timers::future::sleep(Duration::from_millis(1000)).await;
            }
        }
    });
    cx.render(rsx! {
        Header {}
        match download_items.get() {
            Some(Ok(list)) => {
                render! {
                    div {
                        class: "mx-auto max-w-screen-xl grid grid-cols-1 gap-4",
                        for item in list {
                            DownloadItem {
                                download_item: item
                            }
                        }
                    }
                }
            }
            Some(Err(err)) => {
                render! {"Error: {err}"}
            }
            None => {
                render! {"Loading items"}
            }
        },
        Footer {}
    })
}

async fn get_download_items() -> Result<Vec<DownloadItem>, ServerFnError> {
    let items = join!( get_qbt_items(), get_sab_items() ).await;
    let qbt_items = items.0?;
    let sab_items = items.1?;

    Ok([qbt_items, sab_items].concat())
}
