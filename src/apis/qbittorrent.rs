use std::num::ParseFloatError;
use std::sync::RwLock;

use dioxus_fullstack::prelude::*;
use num_traits::cast::ToPrimitive;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::apis::download_item::DownloadItem;
use crate::apis::reqwest::default_reqwest_client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub size: i64,
    pub downloaded: i64,
    pub hash: String,
    pub progress: f64,
    pub state: String,
}

#[derive(Debug)]
pub struct QBitTorrent {
    endpoint: Url,
    client: Client,
    cookies: RwLock<String>,
    pub password: String,
    pub username: String,
}

impl QBitTorrent {
    pub fn new(endpoint: Url, username: String, password: String) -> Self {
        Self {
            endpoint,
            client: default_reqwest_client(),
            cookies: RwLock::new(String::new()),
            username,
            password,
        }
    }

    pub(crate) async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), ServerFnError> {
        let url = self.authenticate_url()?;

        let request = self
            .client
            .post(url.clone())
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .header(
                reqwest::header::HOST,
                url.host()
                    .map(|h| h.to_string())
                    .unwrap_or_else(|| "localhost".to_string()),
            )
            .form(&[("username", username), ("password", password)])
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let sid = response
            .cookies()
            .find(|cookie| cookie.name() == "SID")
            .map(|cookie| cookie.value().to_string())
            .unwrap_or_default();

        self.cookies
            .write()
            .as_deref_mut()
            .map(|x| *x = sid)
            .map_err(|err| ServerFnError::ServerError(err.to_string()))?;
        Ok(())
    }

    fn authenticate_url(&self) -> Result<Url, ServerFnError> {
        let mut url = self.endpoint.clone();
        url.path_segments_mut()
            .map_err(|_| ServerFnError::ServerError("Invalid base url".to_string()))?
            .push("api")
            .push("v2")
            .push("auth")
            .push("login");
        Ok(url)
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, ServerFnError> {
        let initial_response = self.get_items_without_auth().await?;

        let response = if is_auth_failure(&initial_response) {
            self.authenticate(self.username.as_str(), self.password.as_str())
                .await?;
            self.get_items_without_auth().await?
        } else {
            initial_response
        };

        response.json().await.map_err(ServerFnError::from)
    }

    async fn get_items_without_auth(&self) -> Result<Response, ServerFnError> {
        let url = self.get_items_url()?;
        let request = self
            .client
            .get(url.clone())
            .header(
                reqwest::header::HOST,
                url.host()
                    .map(|h| h.to_string())
                    .unwrap_or_else(|| "localhost".to_string()),
            )
            .header(
                reqwest::header::COOKIE,
                format!(
                    "SID={}",
                    match self.cookies.read() {
                        Ok(cookie) => String::from(cookie.as_str()),
                        _ => String::new(),
                    }
                ),
            )
            .build()?;
        let response = self.client.execute(request).await?;
        Ok(response)
    }

    fn get_items_url(&self) -> Result<Url, ServerFnError> {
        let mut call = self.endpoint.clone();
        call.path_segments_mut()
            .map_err(|_| ServerFnError::ServerError("Invalid base url".to_string()))? // Only can happen with an invalid base url
            .push("api")
            .push("v2")
            .push("torrents")
            .push("info");
        call.query_pairs_mut()
            .append_pair("filter", "downloading")
            .finish();
        Ok(call)
    }
}

impl TryInto<DownloadItem> for Item {
    type Error = ParseFloatError;

    fn try_into(self) -> Result<DownloadItem, Self::Error> {
        Ok(DownloadItem {
            download_item_id: format!("qbittorrent-{}", self.hash),
            download_item_name: self.name,
            download_mb_left: (self.size.to_f64().unwrap_or_default()
                + self.downloaded.to_f64().unwrap_or_default())
                / 1024.0
                / 1024.0,
            download_mb_total: (self.size.to_f64().unwrap_or_default()) / 1024.0 / 1024.0,
            download_percentage_complete: self.progress * 100.0,
            download_item_status: match self.state.as_str() {
                "error" => "Some error occurred, applies to paused torrents",
                "missingFiles" => "Torrent data files is missing",
                "uploading" => "Torrent is being seeded and data is being transferred",
                "pausedUP" => "Torrent is paused and has finished downloading",
                "queuedUP" => "Queuing is enabled and torrent is queued for upload",
                "stalledUP" => "Torrent is being seeded, but no connection were made",
                "checkingUP" => "Torrent has finished downloading and is being checked",
                "forcedUP" => "Torrent is forced to uploading and ignore queue limit",
                "allocating" => "Torrent is allocating disk space for download",
                "downloading" => "Torrent is being downloaded and data is being transferred",
                "metaDL" => "Torrent has just started downloading and is fetching metadata",
                "pausedDL" => "Torrent is paused and has NOT finished downloading",
                "queuedDL" => "Queuing is enabled and torrent is queued for download",
                "stalledDL" => "Torrent is being downloaded, but no connection were made",
                "checkingDL" => "Same as checkingUP, but torrent has NOT finished downloading",
                "forcedDL" => "Torrent is forced to downloading to ignore queue limit",
                "checkingResumeData" => "Checking resume data on qBt startup",
                "moving" => "Torrent is moving to another location",
                _ => "Unknown status",
            }.to_string(),
        })
    }
}

fn is_auth_failure(response: &reqwest::Response) -> bool {
    response.status() == reqwest::StatusCode::UNAUTHORIZED
}
