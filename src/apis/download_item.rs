use serde::{Deserialize, Serialize};
use smooth::Smooth;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DownloadItem {
    pub download_item_id: String,
    pub download_item_name: String,
    pub download_mb_left: f64,
    pub download_mb_total: f64,
    pub download_percentage_complete: f64,
    pub download_item_status: String,
}

impl DownloadItem {
    pub fn new(
        download_item_id: String,
        download_item_name: String,
        download_mb_left: f64,
        download_mb_total: f64,
        download_percentage_complete: f64,
        download_item_status: String,
    ) -> Self {
        Self {
            download_item_id,
            download_item_name,
            download_mb_left,
            download_mb_total,
            download_percentage_complete,
            download_item_status,
        }
    }

    pub fn human_amount_left(&self) -> String {
        return format!("{} MB", self.download_mb_left.smooth());
    }
    pub fn human_amount_total(&self) -> String {
        return format!("{} MB", self.download_mb_total.smooth());
    }

    pub fn human_percentage_complete(&self) -> String {
        format!("{}%", self.download_percentage_complete.smooth())
    }
}