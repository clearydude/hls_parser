use super::*;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[test]
fn builds_variant_stream_from_parser_type() {
    let parsed_tags = given_parsed_types_of_each_tag();

    // When we convert them into a MasterPlaylist
    let typed: MasterPlaylist = parsed_tags.try_into().unwrap();

    // Then we can get the converted variant stream
    assert_eq!(
        typed.variant_streams.first().unwrap(),
        &VariantStream::example()
    );

    // And we can get the converted media tag
    assert_eq!(typed.media_tags.first().unwrap(), &MediaTag::example());

    // And we can get the converted I Frame tag
    assert_eq!(typed.i_frames.first().unwrap(), &IFrame::example());

    // And we can get the converted basic tag
    assert_eq!(typed.basic_tags.first().unwrap(), &"EXTM3U".to_string());
}

#[test]
fn rejects_bad_variant_stream() {
    let bad_tag = vec![("EXT-X-STREAM-INF".to_string(), HashMap::new())];

    let typed = MasterPlaylist::try_from(bad_tag);

    assert!(matches!(typed.unwrap_err(), Error::HLSFormat { .. }))
}

#[test]
fn rejects_bad_i_frame() {
    let bad_tag = vec![("EXT-X-I-FRAME-STREAM-INF".to_string(), HashMap::new())];

    let typed = MasterPlaylist::try_from(bad_tag);

    assert!(matches!(typed.unwrap_err(), Error::HLSFormat { .. }))
}

#[test]
fn rejects_bad_media_tag() {
    let bad_tag = vec![("EXT-X-MEDIA".to_string(), HashMap::new())];

    let typed = MasterPlaylist::try_from(bad_tag);

    assert!(matches!(typed.unwrap_err(), Error::HLSFormat { .. }))
}

fn given_parsed_types_of_each_tag() -> Vec<(String, HashMap<String, String>)> {
    let mut variant_stream_attributes = HashMap::new();
    variant_stream_attributes.insert("BANDWIDTH".to_string(), "2483789".to_string());
    variant_stream_attributes.insert("AVERAGE-BANDWIDTH".to_string(), "1762745".to_string());
    variant_stream_attributes.insert(
        "CODECS".to_string(),
        "mp4a.40.2,hvc1.2.4.L90.90".to_string(),
    );
    variant_stream_attributes.insert("RESOLUTION".to_string(), "11960x55540".to_string());
    variant_stream_attributes.insert("FRAME-RATE".to_string(), "23.97".to_string());
    variant_stream_attributes.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
    variant_stream_attributes.insert("AUDIO".to_string(), "aac-128k".to_string());
    variant_stream_attributes.insert("CLOSED-CAPTIONS".to_string(), "NONE".to_string());
    variant_stream_attributes.insert("URI".to_string(), "hdr10/unenc/1650k/vod.m3u8".to_string());

    let variant_stream = ("EXT-X-STREAM-INF".to_string(), variant_stream_attributes);

    let mut media_attributes = HashMap::new();
    media_attributes.insert("TYPE".to_string(), "AUDIO".to_string());
    media_attributes.insert("GROUP-ID".to_string(), "aac-64k".to_string());
    media_attributes.insert("NAME".to_string(), "English".to_string());
    media_attributes.insert("LANGUAGE".to_string(), "en".to_string());
    media_attributes.insert("DEFAULT".to_string(), "YES".to_string());
    media_attributes.insert("AUTOSELECT".to_string(), "YES".to_string());
    media_attributes.insert("CHANNELS".to_string(), "2".to_string());
    media_attributes.insert(
        "URI".to_string(),
        "audio/unenc/aac_64k/vod.m3u8".to_string(),
    );

    let media_tag = ("EXT-X-MEDIA".to_string(), media_attributes);

    let mut i_frame_attributes = HashMap::new();
    i_frame_attributes.insert("BANDWIDTH".to_string(), "77758".to_string());
    i_frame_attributes.insert("CODECS".to_string(), "hvc1.2.4.L63.90".to_string());
    i_frame_attributes.insert("RESOLUTION".to_string(), "640x360".to_string());
    i_frame_attributes.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
    i_frame_attributes.insert(
        "URI".to_string(),
        "hdr10/unenc/900k/vod-iframe.m3u8".to_string(),
    );

    let i_frame = ("EXT-X-I-FRAME-STREAM-INF".to_string(), i_frame_attributes);

    let basic_tag = ("EXTM3U".to_string(), HashMap::new());
    vec![variant_stream, basic_tag, media_tag, i_frame]
}

impl VariantStream {
    fn example() -> Self {
        Self {
            uri: "hdr10/unenc/1650k/vod.m3u8".to_string(),
            bandwidth: 2483789,
            average_bandwidth: 1762745,
            codecs: "mp4a.40.2,hvc1.2.4.L90.90".to_string(),
            resolution: Resolution {
                width: 11960,
                height: 55540,
            },
            video_range: "PQ".to_string(),
            frame_rate: "23.97".to_string(),
            audio: "aac-128k".to_string(),
            closed_captions: "NONE".to_string(),
        }
    }
}

impl MediaTag {
    fn example() -> Self {
        Self {
            media_type: "AUDIO".to_string(),
            group_id: "aac-64k".to_string(),
            name: "English".to_string(),
            language: "en".to_string(),
            default: "YES".to_string(),
            autoselect: "YES".to_string(),
            channels: "2".to_string(),
            uri: "audio/unenc/aac_64k/vod.m3u8".to_string(),
        }
    }
}

impl IFrame {
    fn example() -> Self {
        Self {
            bandwidth: 77758,
            codecs: "hvc1.2.4.L63.90".to_string(),
            resolution: Resolution {
                width: 640,
                height: 360,
            },
            video_range: "PQ".to_string(),
            uri: "hdr10/unenc/900k/vod-iframe.m3u8".to_string(),
        }
    }
}
