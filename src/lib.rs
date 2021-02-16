mod api_client;
mod errors;
pub mod models;
mod parser;

use crate::api_client::{ApiClient, BlockingApiClient};
use crate::errors::Result;
use crate::models::MasterPlaylist;
use crate::parser::HLSParser;
use std::convert::TryInto;

pub fn parse_default_hls() -> Result<()> {
    let api_client = BlockingApiClient {};
    let hls_str = &api_client
        .get_master_playlist()
        .expect("Failed to get HLS playlist");
    let parsed = parse_hls(hls_str);
    println!("{:#?}", parsed);
    Ok(())
}

pub fn parse_hls(hls_str: &str) -> Result<MasterPlaylist> {
    let parser = HLSParser {};

    let parsed = parser.parse(hls_str)?;
    parsed.try_into()
}
