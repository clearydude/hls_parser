use crate::api_client::{ApiClient, BlockingApiClient};
use crate::errors::Result;
use crate::models::MasterPlaylist;
use crate::parser::HLSParser;
use std::convert::TryInto;

mod api_client;
mod errors;
mod models;
mod parser;

fn main() -> Result<()> {
    let api_client = BlockingApiClient {};

    let parser = HLSParser {};

    let parsed = parser.parse(&api_client.get_master_playlist()?)?;
    let master_playlist: MasterPlaylist = parsed.try_into()?;

    println!("{:#?}", master_playlist);

    Ok(())
}
