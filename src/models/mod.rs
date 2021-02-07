use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Resolution {
    width: usize,
    height: usize
}

#[derive(Debug, PartialEq)]
enum VideoRange {
    PQ,
    SDR
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

//impl TryFrom<&str> for VariantStreamAttrs {
//    type Error = ();
//
//    // this should be from_str
//    /// Parses a comma seperated string of attributes.
//    fn try_from(attr_str: &str) -> Result<Self, Self::Error> {
//        let attrs = parse_attr_list_to_map(attr_str);
//
//        // TODO ew error handle
//        Ok(Self {
//            bandwidth: attrs.get("BANDWIDTH").unwrap().parse().unwrap(),
//            average_bandwidth: attrs.get("AVERAGE-BANDWIDTH").unwrap().parse().unwrap(),
//            codecs: vec![],
//            resolution: Resolution { width: 0, height: 0 },
//            frame_rate: 0.0,
//            video_range: VideoRange::PQ,
//            audio: Some("".to_string()),
//            closed_captions: None
//        })
//    }
//}

#[derive(Debug)]
struct VariantStream {
    attributes: VariantStreamAttrs,
    uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    fn parse_attr_list_to_map(attr_str: &str) -> HashMap<String, String> {
        let mut attr_map = HashMap::new();

        println!("String to split {}", attr_str);

        tag!("BANDWIDTH=")

//        let attrs = attr_str.split(",");
//
//        for attr in attrs {
//            println!("Attr: {}", attr);
//        let mut attr_iter = attr.splitn(2, "=");
//        let key = attr_iter.next().expect(&format!("Malformed attribute: {} does not contain key", attr));
//        let value = attr_iter.next().expect(&format!("Malformed attribute: {} does not contain value", attr));
//        attr_map.insert(key.into(), value.into());
        }

        attr_map
    }

    #[test]
    fn parses_attribute_list_into_key_value_map() {
        let attr_str = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;

        let mut expected_map: HashMap<String, String> = HashMap::new();
        expected_map.insert("BANDWIDTH".into(), "2312764".into());
        expected_map.insert("AVERAGE-BANDWIDTH".into(), "1919803".into());
        expected_map.insert("CODECS".into(), "ec-3,hvc1.2.4.L63.90".into());
        expected_map.insert("RESOLUTION".into(), "640x360".into());
        expected_map.insert("FRAME-RATE".into(), "23.97".into());
        expected_map.insert("VIDEO-RANGE".into(), "PQ".into());
        expected_map.insert("AUDIO".into(), "atmos".into());
        expected_map.insert("CLOSED-CAPTIONS".into(), "NONE".into());

        let parsed_map = parse_attr_list_to_map(attr_str);

//        assert_eq!(parsed_map, expected_map)
    }

    #[test]
    fn parses_variant_stream_attributes() {
        let variant_stream_attr = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;

        let expected_variant_stream_attr = VariantStreamAttrs {
            bandwidth: 2312764,
            average_bandwidth: 1919803,
            codecs: vec!["ec-3".to_string(), "hvc1.2.4.L63.90".to_string()],
            resolution: Resolution { width: 640, height: 360 },
            frame_rate: 23.97,
            video_range: VideoRange::PQ,
            audio: Some("atmos".to_string()),
            closed_captions: None
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
