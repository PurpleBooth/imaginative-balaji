#![feature(future_join)]
//! A read only client for `QBittorrent` and `SABnzbd`

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::missing_assert_message,
    clippy::todo,
    clippy::allow_attributes_without_reason,
    clippy::panic,
    clippy::panicking_unwrap,
    clippy::panic_in_result_fn
)]

use dioxus_fullstack::prelude::*;
use log::LevelFilter;

mod apis;
mod app;
mod components;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to initialize logger");
    LaunchBuilder::new(crate::app::app).launch();
}
