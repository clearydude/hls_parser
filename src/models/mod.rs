#[derive(Debug, PartialEq)]
struct Resolution {
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq)]
enum VideoRange {
    PQ,
    SDR,
}

// defined by https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-00
// also https://developer.apple.com/documentation/http_live_streaming/hls_authoring_specification_for_apple_devices
/// Attributes that provide additional information about
/// a variant stream.
#[derive(Debug, PartialEq)]
struct VariantStreamAttrs {
    /// Peak segment bit rate (in bits per second).
    bandwidth: u32,
    /// Average segment bit rate (in bits per second).
    /// Although version 8 of the HTTP live streaming protocol specifies this field as optional,
    /// Apple's lists it as required.
    average_bandwidth: u32,
    /// List of formats present in media sample types.
    codecs: Vec<String>,
    /// Optimal pixel resolution to display video.
    resolution: Resolution,
    //// Maximum frame rate for video.
    frame_rate: f32,
    /// Must be specified unless all variants and renditions are SDR.
    /// For simplification purposes we'll require this attribute.
    video_range: VideoRange,
    /// Used to match this variant stream to its audio rendition.
    /// It corresponds to the `GROUP-ID` field of an `EXT-X-MEDIA` audio tag.
    audio: Option<String>,
    /// This value is either `NONE` or corresponds to the `GROUP-ID` field of an `EXT-X-MEDIA`
    /// closed-captions tag. If this value is `NONE`, all `EXT-X-STREAM-INF` tags must also be
    /// `NONE` to avoid playback inconsistencies.
    closed_captions: Option<String>,
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

#[derive(Debug)]
struct VariantStream {
    attributes: VariantStreamAttrs,
    uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_variant_stream_attributes() {
        let _variant_stream_attr = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;

        let _expected_variant_stream_attr = VariantStreamAttrs {
            bandwidth: 2312764,
            average_bandwidth: 1919803,
            codecs: vec!["ec-3".to_string(), "hvc1.2.4.L63.90".to_string()],
            resolution: Resolution {
                width: 640,
                height: 360,
            },
            frame_rate: 23.97,
            video_range: VideoRange::PQ,
            audio: Some("atmos".to_string()),
            closed_captions: None,
        };

        //        let parsed_variant_stream_attr: VariantStreamAttrs = variant_stream_attr.try_into().unwrap();
        //
        //        assert_eq!(expected_variant_stream_attr, parsed_variant_stream_attr)
    }
    //
    //    #[test]
    //    fn parses_variant_stream() {
    //        let variant_stream_str = r#"
    //            #EXT-X-STREAM-INF:BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE
    //            hdr10/unenc/900k/vod.m3u8
    //        "#;
    //
    //        println!("{}", variant_stream_str);
    //    }
}
