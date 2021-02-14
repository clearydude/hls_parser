mod models;
mod parser;

fn main() {
    //    let filename = "master_unenc_hdr10_all.m3u8";
    //
    //    println!("In file {}", filename);
    //
    //    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //
    //    println!("With text:\n{}", contents);
    let body = reqwest::blocking::get(
        "https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_hdr10_all.m3u8",
    )
    .unwrap()
    .text()
    .unwrap();

    println!("body = {:?}", body);
}

///// A media playlist file containing the I-frames of a
///// multimedia presentation.
//#[derive(Debug)]
//struct IFrameStream {
//    /// Peak segment bit rate (in bits per second).
//    bandwidth: u32,
//    /// List of formats present in media sample types.
//    codecs: Vec<String>,
//    /// Optimal pixel resolution to display video.
//    resolution: String,
//    video_range: String,
//    /// The URI that identifies the I-frame media playlist file.
//    /// (This file MUST contain an `EXT-X-I-FRAMES-ONLY` tag.)
//    uri: String
//}

//#[derive(Debug)]
//enum MediaType {
//    Audio,
//    Video,
//    Subtitles,
//    ClosedCaptions
//}

///// Used to relate media playlists that contain alternative
///// renditions of the same content.
//#[derive(Debug)]
//struct Media {
//    /// The type of media described.
//    media_type: String,
//    /// The URI identifying the media playlist file.
//    /// If the MediaType is ClosedCaptions, the uri must be None.
//    uri: String,
//    group_id: String,
//    language: String,
//    name: String,
//    default: String,
//    autoselect: String,
//    channels: String
//}

//#[derive(Debug)]
//struct MasterPlaylist {
//    //    media: Vec<Media>,
//    variant_streams: Vec<VariantStream>,
//    //    i_frame_streams: Vec<IFrameStream>
//}
