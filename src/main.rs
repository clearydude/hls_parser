use crate::api_client::{ApiClient, BlockingApiClient};
use crate::models::MasterPlaylist;
use crate::parser::HLSParser;
use std::convert::TryInto;

mod api_client;
mod errors;
mod models;
mod parser;

fn main() {
    // lets talk about sorting options here...
    //
    //
    // options for display all
    // display variant streams
    // display i frames
    //
    // for displaying variant stream give options to sort (asc, desc) by bandwidth, avg bandwidth, resolution (w * h), audio (string), codecs (string)
    // for displaying i frame give options to sort bandwidth, codecs, resolution

    let api_client = BlockingApiClient {};

    let parser = HLSParser {};

    let parsed = parser.parse(&api_client.get_master_playlist().unwrap());
    let master_playlist: MasterPlaylist = parsed.unwrap().try_into().unwrap();

    println!("{:#?}", master_playlist);
}
