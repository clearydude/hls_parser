use crate::api_client::BlockingApiClient;

mod api_client;
mod errors;
mod models;
mod parser;

fn main() {
    /// lets talk about sorting options here...
    ///
    ///
    /// options for display all
    /// display variant streams
    /// display i frames
    ///
    /// for displaying variant stream give options to sort (asc, desc) by bandwidth, avg bandwidth, resolution (w * h), audio (string), codecs (string)
    /// for displaying i frame give options to sort bandwidth, codecs, resolution
    //    let filename = "master_unenc_hdr10_all.m3u8";
    //
    //    println!("In file {}", filename);
    //
    //    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //
    //    println!("With text:\n{}", contents);
    let api_client = BlockingApiClient {};

    println!("{:#?}", api_client.get_master_playlist());
}
