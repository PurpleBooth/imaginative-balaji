use dioxus_fullstack::prelude::ServerFnError;
use reqwest::Client;
use std::num::ParseFloatError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::apis::download_item::DownloadItem;
use crate::apis::reqwest::default_reqwest_client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub mbleft: String,
    pub mb: String,
    pub nzo_id: String,
    pub percentage: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    queue: Queue,
}

#[derive(Debug, Serialize, Deserialize)]
struct Queue {
    slots: Vec<Item>,
}

#[derive(Debug)]
pub struct Sabnzbd {
    endpoint: Url,
    apikey: String,
    client: Client,
}

impl Sabnzbd {
    pub(crate) fn new(endpoint: Url, apikey: String) -> Self {
        Self {
            endpoint,
            apikey,
            client: default_reqwest_client(),
        }
    }

    fn authenticated_url(&self) -> Url {
        let mut url = self.endpoint.clone();
        url.path_segments_mut().expect("Invalid path").push("api"); // should never fail, checked in new.
        url.query_pairs_mut()
            .append_pair("apikey", &self.apikey)
            .append_pair("output", "json")
            .finish();
        url
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, ServerFnError> {
        let call = self
            .authenticated_url()
            .query_pairs_mut()
            .append_pair("mode", "queue")
            .finish()
            .to_string();
        Ok(self
            .client
            .get(call)
            .send()
            .await?
            .error_for_status()?
            .json::<Response>()
            .await?
            .queue
            .slots)
    }
}

impl TryInto<DownloadItem> for Item {
    type Error = ParseFloatError;

    fn try_into(self) -> Result<DownloadItem, Self::Error> {
        Ok(DownloadItem {
            download_item_id: self.nzo_id,
            download_item_name: self.name,
            download_mb_left: f64::from_str(&self.mbleft)?,
            download_mb_total: f64::from_str(&self.mb)?,
            download_percentage_complete: f64::from_str(&self.percentage)?,
            download_item_status: self.status
        })
    }
}
