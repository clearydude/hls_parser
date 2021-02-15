use crate::parser::*;

#[test]
fn parses_master_playlist() {
    let filename = "master_unenc_hdr10_all.m3u8";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let parsed = parse_master_playlist(&contents);

    println!("{:#?}", parsed);
}

#[test]
fn strips_whitespace() {}

#[test]
fn strips_hash() {
    let tag = "#EXTM3U\n";
    let parsed = parse_tag_name(tag);
    println!("{:?}", parsed);
}

#[test]
fn parses_all_tags() {
    let tags_str = "#EXTM3U\n#EXT-X-INDEPENDENT-SEGMENTS\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-128k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_128k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=2483789,AVERAGE-BANDWIDTH=1762745,CODECS=\"mp4a.40.2,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8\n\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=222552,CODECS=\"hvc1.2.4.L93.90\",RESOLUTION=1280x720,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/3300k/vod-iframe.m3u8\"\n\n";

    let parsed = parse_master_playlist(tags_str);

    let expected = vec![
        Tag::BasicTag(BasicTag {
            name: "EXTM3U".to_string(),
        }),
        Tag::BasicTag(BasicTag {
            name: "EXT-X-INDEPENDENT-SEGMENTS".to_string(),
        }),
        Tag::TagWithAttributes(TagWithAttributes {
            name: "EXT-X-MEDIA".to_string(),
            attributes: vec![
                ("TYPE".to_string(), "AUDIO".to_string()),
                ("GROUP-ID".to_string(), "aac-128k".to_string()),
                ("NAME".to_string(), "English".to_string()),
                ("LANGUAGE".to_string(), "en".to_string()),
                ("DEFAULT".to_string(), "YES".to_string()),
                ("AUTOSELECT".to_string(), "YES".to_string()),
                ("CHANNELS".to_string(), "2".to_string()),
                (
                    "URI".to_string(),
                    "audio/unenc/aac_128k/vod.m3u8".to_string(),
                ),
            ],
        }),
        Tag::TagWithURI(TagWithURI {
            name: "EXT-X-STREAM-INF".to_string(),
            attributes: vec![
                ("BANDWIDTH".to_string(), "2483789".to_string()),
                ("AVERAGE-BANDWIDTH".to_string(), "1762745".to_string()),
                (
                    "CODECS".to_string(),
                    "mp4a.40.2,hvc1.2.4.L90.90".to_string(),
                ),
                ("RESOLUTION".to_string(), "960x540".to_string()),
                ("FRAME-RATE".to_string(), "23.97".to_string()),
                ("VIDEO-RANGE".to_string(), "PQ".to_string()),
                ("AUDIO".to_string(), "aac-128k".to_string()),
                ("CLOSED-CAPTIONS".to_string(), "NONE".to_string()),
            ],
            uri: "hdr10/unenc/1650k/vod.m3u8".to_string(),
        }),
        Tag::TagWithAttributes(TagWithAttributes {
            name: "EXT-X-I-FRAME-STREAM-INF".to_string(),
            attributes: vec![
                ("BANDWIDTH".to_string(), "222552".to_string()),
                ("CODECS".to_string(), "hvc1.2.4.L93.90".to_string()),
                ("RESOLUTION".to_string(), "1280x720".to_string()),
                ("VIDEO-RANGE".to_string(), "PQ".to_string()),
                (
                    "URI".to_string(),
                    "hdr10/unenc/3300k/vod-iframe.m3u8".to_string(),
                ),
            ],
        }),
    ];

    assert_eq!(parsed, Ok(("", expected)));
}

#[test]
fn parses_variant_stream() {
    let tags = "#EXT-X-STREAM-INF:BANDWIDTH=1352519,AVERAGE-BANDWIDTH=959558,CODECS=\"mp4a.40.2,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-64k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8";

    let parsed = parse_variant_stream(tags);

    let expected_variant_stream = TagWithURI {
        name: "EXT-X-STREAM-INF".to_string(),
        attributes: vec![
            ("BANDWIDTH".to_string(), "1352519".to_string()),
            ("AVERAGE-BANDWIDTH".to_string(), "959558".to_string()),
            (
                "CODECS".to_string(),
                "mp4a.40.2,hvc1.2.4.L63.90".to_string(),
            ),
            ("RESOLUTION".to_string(), "640x360".to_string()),
            ("FRAME-RATE".to_string(), "23.97".to_string()),
            ("VIDEO-RANGE".to_string(), "PQ".to_string()),
            ("AUDIO".to_string(), "aac-64k".to_string()),
            ("CLOSED-CAPTIONS".to_string(), "NONE".to_string()),
        ],
        uri: "hdr10/unenc/900k/vod.m3u8".to_string(),
    };

    assert_eq!(parsed, Ok(("", Tag::TagWithURI(expected_variant_stream))))
}

#[test]
fn parses_i_frame_attribute_list_into_key_value_map() {
    let attr_str = "BANDWIDTH=77758,CODECS=\"hvc1.2.4.L63.90\",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/900k/vod-iframe.m3u8\"";

    let parsed = parse_attributes(attr_str);

    let expected = Ok((
        "",
        vec![
            ("BANDWIDTH".to_string(), "77758".to_string()),
            ("CODECS".to_string(), "hvc1.2.4.L63.90".to_string()),
            ("RESOLUTION".to_string(), "640x360".to_string()),
            ("VIDEO-RANGE".to_string(), "PQ".to_string()),
            (
                "URI".to_string(),
                "hdr10/unenc/900k/vod-iframe.m3u8".to_string(),
            ),
        ],
    ));

    assert_eq!(parsed, expected)
}

#[test]
fn parses_media_attribute_list_into_key_value_map() {
    let attr_str = "TYPE=AUDIO,GROUP-ID=\"aac-64k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_64k/vod.m3u8\"";

    let parsed = parse_attributes(attr_str);

    let expected = Ok((
        "",
        vec![
            ("TYPE".to_string(), "AUDIO".to_string()),
            ("GROUP-ID".to_string(), "aac-64k".to_string()),
            ("NAME".to_string(), "English".to_string()),
            ("LANGUAGE".to_string(), "en".to_string()),
            ("DEFAULT".to_string(), "YES".to_string()),
            ("AUTOSELECT".to_string(), "YES".to_string()),
            ("CHANNELS".to_string(), "2".to_string()),
            (
                "URI".to_string(),
                "audio/unenc/aac_64k/vod.m3u8".to_string(),
            ),
        ],
    ));

    assert_eq!(parsed, expected)
}

#[test]
fn parses_variant_stream_attribute_list_into_key_value_map() {
    let attr_str = "BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE";
    let parsed = parse_attributes(attr_str);

    let expected = Ok((
        "",
        vec![
            ("BANDWIDTH".to_string(), "2312764".to_string()),
            ("AVERAGE-BANDWIDTH".to_string(), "1919803".to_string()),
            ("CODECS".to_string(), "ec-3,hvc1.2.4.L63.90".to_string()),
            ("RESOLUTION".to_string(), "640x360".to_string()),
            ("FRAME-RATE".to_string(), "23.97".to_string()),
            ("VIDEO-RANGE".to_string(), "PQ".to_string()),
            ("AUDIO".to_string(), "atmos".to_string()),
            ("CLOSED-CAPTIONS".to_string(), "NONE".to_string()),
        ],
    ));

    assert_eq!(parsed, expected)
}

#[test]
fn parses_inner_list() {
    let attr_str = "\"ec-3,hvc1.2.4.L63.90\"";
    assert_eq!(
        parse_quoted_value(attr_str),
        Ok(("", "ec-3,hvc1.2.4.L63.90".to_string()))
    )
}

#[test]
fn parses_attribute_into_key_value() {
    let attr_str = "BANDWIDTH=2312764";
    assert_eq!(
        parse_attribute(attr_str),
        Ok(("", ("BANDWIDTH".to_string(), "2312764".to_string())))
    )
}

#[test]
fn parses_attribute_with_quoted_value_into_key_value() {
    let attr_str = "AUDIO=\"atmos\"";
    assert_eq!(
        parse_attribute(attr_str),
        Ok(("", ("AUDIO".to_string(), "atmos".to_string())))
    )
}

#[test]
fn parses_attribute_with_list_value_into_key_value() {
    let attr_str = "CODECS=\"ec-3,hvc1.2.4.L63.90\"";
    assert_eq!(
        parse_attribute(attr_str),
        Ok((
            "",
            ("CODECS".to_string(), "ec-3,hvc1.2.4.L63.90".to_string())
        ))
    )
}

#[test]
fn parses_attribute_with_float_value_into_key_value() {
    let attr_str = "FRAME-RATE=23.97";
    assert_eq!(
        parse_attribute(attr_str),
        Ok(("", ("FRAME-RATE".to_string(), "23.97".to_string())))
    )
}

#[test]
fn parses_attribute_with_resolution_value_into_key_value() {
    let attr_str = "RESOLUTION=640x360";
    assert_eq!(
        parse_attribute(attr_str),
        Ok(("", ("RESOLUTION".to_string(), "640x360".to_string())))
    )
}
