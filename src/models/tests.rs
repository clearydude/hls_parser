use super::*;
use std::convert::TryInto;

#[test]
fn builds_variant_stream_from_parser_type() {
    let mut attributes = HashMap::new();
    attributes.insert("BANDWIDTH".to_string(), "2483789".to_string());
    attributes.insert("AVERAGE-BANDWIDTH".to_string(), "1762745".to_string());
    attributes.insert(
        "CODECS".to_string(),
        "mp4a.40.2,hvc1.2.4.L90.90".to_string(),
    );
    attributes.insert("RESOLUTION".to_string(), "11960x55540".to_string());
    attributes.insert("FRAME-RATE".to_string(), "23.97".to_string());
    attributes.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
    attributes.insert("AUDIO".to_string(), "aac-128k".to_string());
    attributes.insert("CLOSED-CAPTIONS".to_string(), "NONE".to_string());
    attributes.insert("URI".to_string(), "hdr10/unenc/1650k/vod.m3u8".to_string());

    let untyped_variant_stream = ("EXT-X-STREAM-INF".to_string(), Some(attributes));

    let mut attributes2 = HashMap::new();
    attributes2.insert("BANDWIDTH".to_string(), "15811232".to_string());
    attributes2.insert("AVERAGE-BANDWIDTH".to_string(), "10058085".to_string());
    attributes2.insert(
        "CODECS".to_string(),
        "mp4a.40.2,hvc1.2.4.L150.90".to_string(),
    );
    attributes2.insert("RESOLUTION".to_string(), "2560x1440".to_string());
    attributes2.insert("FRAME-RATE".to_string(), "23.97".to_string());
    attributes2.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
    attributes2.insert("AUDIO".to_string(), "aac-128k".to_string());
    attributes2.insert("CLOSED-CAPTIONS".to_string(), "NONE".to_string());
    attributes2.insert("URI".to_string(), "hdr10/unenc/10000k/vod.m3u8".to_string());

    let untyped_variant_stream2 = ("EXT-X-STREAM-INF".to_string(), Some(attributes2));

    let basic_tag = ("EXTM3U".to_string(), None);

    let typed = MasterPlaylist::try_from(vec![
        untyped_variant_stream,
        untyped_variant_stream2,
        basic_tag,
    ]);
}
