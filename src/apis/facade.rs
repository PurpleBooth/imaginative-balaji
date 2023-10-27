use std::env;
use std::fmt::Display;

use dioxus_fullstack::once_cell::sync::Lazy;
use dioxus_fullstack::prelude::*;
use url::Url;

use crate::apis::download_item::DownloadItem;
#[cfg(feature = "ssr")]
use crate::apis::qbittorrent::QBitTorrent;
#[cfg(feature = "ssr")]
use crate::apis::sabnzbd::Sabnzbd;

#[cfg(feature = "ssr")]
static SABNZBD: Lazy<Sabnzbd> = Lazy::new(|| {
    let endpoint = env::var("SAB_URL").and_then(Url::parse).expect("Invalid SAB url");
    Sabnzbd::new(
        endpoint,
        env::var("SAB_API").expect("Invalid SAB url").to_string(),
    )
});

#[cfg(feature = "ssr")]
static QBT: Lazy<QBitTorrent> = Lazy::new(|| {
    let endpoint = env::var("QBT_URL").and_then(Url::parse).expect("Invalid QBT url");
    let username = env::var("QBT_USERNAME").expect("Invalid QBT username");
    let password = env::var("QBT_PASSWORD").expect("Invalid QBT password");

    QBitTorrent::new(endpoint, username, password)
});

#[server]
pub async fn get_sab_items() -> Result<Vec<DownloadItem>, ServerFnError> {
    info!("Getting items from Sabnzbd");
    SABNZBD
        .get_items()
        .await
        .map_err(|err| {
            warn!("Error getting items from Sabnzbd: {}", err);
            err
        })?
        .into_iter()
        .map(TryInto::<DownloadItem>::try_into)
        .collect::<Result<Vec<DownloadItem>, _>>()
        .map_err(|err| {
            warn!("Error converting items from Sabnzbd: {}", err);
            ServerFnError::from(err)
        })
        .map(|value| {
            info!("Got items {} from Sabnzbd", value.len());
            value
        })
}

#[server]
pub async fn get_qbt_items() -> Result<Vec<DownloadItem>, ServerFnError> {
    info!("Getting items from QBittorrent");
    QBT.get_items()
        .await
        .map_err(|err| {
            warn!("Error getting items from QBittorrent: {}", err);
            err
        })?
        .into_iter()
        .map(TryInto::<DownloadItem>::try_into)
        .collect::<Result<Vec<DownloadItem>, _>>()
        .map_err(|err| {
            warn!("Error getting items from QBittorrent: {}", err);
            ServerFnError::from(err)
        })
        .map(|value| {
            info!("Got items {} from QBittorrent", value.len());
            value
        })
}

fn to_server_err<T: Display>(x: T) -> ServerFnError {
    ServerFnError::ServerError(format!("{x}"))
}
