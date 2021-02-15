use crate::api_client::{ApiClient, BlockingApiClient};
use crate::parser::HLSParser;

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

    println!(
        "{:#?}",
        parser.parse(&api_client.get_master_playlist().unwrap())
    );
}
