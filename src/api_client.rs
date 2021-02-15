use crate::errors::{Result, *};

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::APIError(e.to_string())
    }
}

/// A super simple blocking API client.
/// Its only responsibility is to grab a master playlist from a very specific API endpoint.
pub(crate) struct BlockingApiClient {}

/// This trait represents the behavior we need from an Api Client.
pub(crate) trait ApiClient {
    /// Retrieve a master playlist and return its contents as a String.
    fn get_master_playlist(&self) -> Result<String>;
}

impl ApiClient for BlockingApiClient {
    /// Create a blocking HTTP client and get a master playlist.
    /// Note: We don't bother creating and storing one because we're only making this request
    /// one time -- if we were going to issue multiple requests it would be worth constructing
    /// a client to reuse.
    fn get_master_playlist(&self) -> Result<String> {
        reqwest::blocking::get(
            "https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_hdr10_all.m3u8",
        )?
        .text()
        .map_err(Into::into)
    }
}
