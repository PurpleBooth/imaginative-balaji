use reqwest::Client;

#[cfg(feature = "ssr")]
pub fn default_reqwest_client() -> Client {
    reqwest::ClientBuilder::default()
        .brotli(true)
        .cookie_store(true)
        .gzip(true)
        .deflate(true)
        .build()
        .expect("Failed to create reqwest client, this is a bug")
}
