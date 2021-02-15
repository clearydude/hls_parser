use crate::errors::{Result, *};

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::APIError(e.to_string())
    }
}

pub(crate) struct BlockingApiClient {}

impl BlockingApiClient {
    pub(crate) fn get_master_playlist(&self) -> Result<String> {
        reqwest::blocking::get(
            "https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_hdr10_all.m3u8",
        )?
        .text()
        .map_err(Into::into)
    }
}
