use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};

use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};

fn parse_value(attr_str: &str) -> nom::IResult<&str, String> {
    map(is_not(",\n"), |attr: &str| attr.to_string())(attr_str)
}

fn parse_quoted_value(attr_str: &str) -> nom::IResult<&str, String> {
    map(
        delimited(tag("\""), is_not("\""), tag("\"")),
        |attr: &str| attr.to_string(),
    )(attr_str)
}

fn parse_attribute(attr_str: &str) -> nom::IResult<&str, (String, String)> {
    map(
        separated_pair(
            take_until("="),
            tag("="),
            alt((parse_quoted_value, parse_value)),
        ),
        |(key, value)| (key.to_string(), value.to_string()),
    )(attr_str)
}

fn parse_attributes(attrs_str: &str) -> nom::IResult<&str, Vec<(String, String)>> {
    separated_list1(tag(","), parse_attribute)(attrs_str)
}

fn parse_tag_name(tag_str: &str) -> nom::IResult<&str, String> {
    map(take_until(":"), |name: &str| name.to_string())(tag_str)
}

fn parse_tag_and_attributes(tag_str: &str) -> nom::IResult<&str, (String, Vec<(String, String)>)> {
    separated_pair(parse_tag_name, tag(":"), parse_attributes)(tag_str)
}

#[derive(Debug, PartialEq)]
struct TagWithURI {
    name: String,
    attributes: Vec<(String, String)>,
    uri: String,
}

#[derive(Debug, PartialEq)]
struct TagWithAttributes {
    name: String,
    attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
struct SimpleTag {
    name: String,
}

fn parse_variant_stream(variant_stream_str: &str) -> nom::IResult<&str, Tag> {
    map(
        separated_pair(parse_tag_and_attributes, tag("\n"), parse_value),
        |((name, attributes), uri)| {
            Tag::TagWithURI(TagWithURI {
                name,
                attributes,
                uri,
            })
        },
    )(variant_stream_str)
}

#[derive(Debug, PartialEq)]
enum Tag {
    TagWithAttributes(TagWithAttributes),
    TagWithURI(TagWithURI),
    SimpleTag(SimpleTag),
}

fn parse_tag_with_attributes(tag_w_attributes_str: &str) -> nom::IResult<&str, Tag> {
    map(parse_tag_and_attributes, |(name, attributes)| {
        Tag::TagWithAttributes(TagWithAttributes { name, attributes })
    })(tag_w_attributes_str)
}

fn parse_simple_tag(simple_tag_str: &str) -> nom::IResult<&str, Tag> {
    map(is_not("\n"), |name: &str| {
        Tag::SimpleTag(SimpleTag {
            name: name.to_string(),
        })
    })(simple_tag_str)
}

fn parse_master_playlist(playlist_str: &str) -> nom::IResult<&str, Vec<Tag>> {
    many1(alt((
        parse_variant_stream,
        parse_tag_with_attributes,
        parse_simple_tag,
    )))(playlist_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    // still need to strip out whitespace
    #[test]
    fn parses_all_tags() {
        let tags_str = "#EXTM3U\n#EXT-X-INDEPENDENT-SEGMENTS\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-128k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_128k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=2483789,AVERAGE-BANDWIDTH=1762745,CODECS=\"mp4a.40.2,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8\n\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=222552,CODECS=\"hvc1.2.4.L93.90\",RESOLUTION=1280x720,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/3300k/vod-iframe.m3u8\"\n\n";

        let parsed = parse_master_playlist(tags_str);

        println!("{:#?}", parsed);
    }

    // #[test]
    // fn parses_master_playlist() {
    //     let parsed = parse_master_playlist(FILE_STR);
    //
    //     println!("{:?}", parsed);
    // }

    // need to strip whitespace before both lines here
    // there's also whitespace after the uri that we need to strip
    #[test]
    fn parses_variant_stream() {
        let tags = "#EXT-X-STREAM-INF:BANDWIDTH=1352519,AVERAGE-BANDWIDTH=959558,CODECS=\"mp4a.40.2,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-64k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8";

        let parsed = parse_variant_stream(tags);

        let expected_variant_stream = TagWithURI {
            name: "#EXT-X-STREAM-INF".to_string(),
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
        let attr_str = r#"BANDWIDTH=77758,CODECS="hvc1.2.4.L63.90",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI="hdr10/unenc/900k/vod-iframe.m3u8""#;

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
        let attr_str = r#"TYPE=AUDIO,GROUP-ID="aac-64k",NAME="English",LANGUAGE="en",DEFAULT=YES,AUTOSELECT=YES,CHANNELS="2",URI="audio/unenc/aac_64k/vod.m3u8""#;

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
        let attr_str = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;
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
        let attr_str = r#""ec-3,hvc1.2.4.L63.90""#;
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
        let attr_str = r#"AUDIO="atmos""#;
        assert_eq!(
            parse_attribute(attr_str),
            Ok(("", ("AUDIO".to_string(), "atmos".to_string())))
        )
    }

    #[test]
    fn parses_attribute_with_list_value_into_key_value() {
        let attr_str = r#"CODECS="ec-3,hvc1.2.4.L63.90""#;
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

    const FILE_STR: &str = "#EXTM3U\n#EXT-X-INDEPENDENT-SEGMENTS\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-128k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_128k/vod.m3u8\"\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-64k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_64k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=2483789,AVERAGE-BANDWIDTH=1762745,CODECS=\"mp4a.40.2,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=15811232,AVERAGE-BANDWIDTH=10058085,CODECS=\"mp4a.40.2,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/10000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=20655057,AVERAGE-BANDWIDTH=13066616,CODECS=\"mp4a.40.2,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/13000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=25702333,AVERAGE-BANDWIDTH=16502314,CODECS=\"mp4a.40.2,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/16500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=3790212,AVERAGE-BANDWIDTH=2621435,CODECS=\"mp4a.40.2,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/2500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=5044473,AVERAGE-BANDWIDTH=3403100,CODECS=\"mp4a.40.2,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/3300k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=6986073,AVERAGE-BANDWIDTH=4717677,CODECS=\"mp4a.40.2,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/4600k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=9533270,AVERAGE-BANDWIDTH=6100009,CODECS=\"mp4a.40.2,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/6000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=12156778,AVERAGE-BANDWIDTH=7766087,CODECS=\"mp4a.40.2,hvc1.2.4.H120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/7700k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=13911387,AVERAGE-BANDWIDTH=8880895,CODECS=\"mp4a.40.2,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-128k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/8800k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=705826,AVERAGE-BANDWIDTH=514769,CODECS=\"mp4a.40.2,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-64k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/450k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=1352519,AVERAGE-BANDWIDTH=959558,CODECS=\"mp4a.40.2,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"aac-64k\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8\n\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=222552,CODECS=\"hvc1.2.4.L93.90\",RESOLUTION=1280x720,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/3300k/vod-iframe.m3u8\"\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=77758,CODECS=\"hvc1.2.4.L63.90\",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/900k/vod-iframe.m3u8\"\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"eac3\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"6\",URI=\"audio/unenc/ec3_256k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=2612376,AVERAGE-BANDWIDTH=1891332,CODECS=\"ec-3,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=15939819,AVERAGE-BANDWIDTH=10186672,CODECS=\"ec-3,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/10000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=20783644,AVERAGE-BANDWIDTH=13195203,CODECS=\"ec-3,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/13000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=25830920,AVERAGE-BANDWIDTH=16630901,CODECS=\"ec-3,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/16500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=3918799,AVERAGE-BANDWIDTH=2750022,CODECS=\"ec-3,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/2500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=5173060,AVERAGE-BANDWIDTH=3531687,CODECS=\"ec-3,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/3300k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=7114660,AVERAGE-BANDWIDTH=4846264,CODECS=\"ec-3,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/4600k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=9661857,AVERAGE-BANDWIDTH=6228596,CODECS=\"ec-3,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/6000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=12285365,AVERAGE-BANDWIDTH=7894674,CODECS=\"ec-3,hvc1.2.4.H120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/7700k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=14039974,AVERAGE-BANDWIDTH=9009482,CODECS=\"ec-3,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/8800k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=898051,AVERAGE-BANDWIDTH=706994,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/450k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=1544744,AVERAGE-BANDWIDTH=1151783,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"eac3\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8\n\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"atmos\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"16/JOC\",URI=\"audio/unenc/atmos_1024k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=3380396,AVERAGE-BANDWIDTH=2659352,CODECS=\"ec-3,hvc1.2.4.L90.90\",RESOLUTION=960x540,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/1650k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=16707839,AVERAGE-BANDWIDTH=10954692,CODECS=\"ec-3,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/10000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=21551664,AVERAGE-BANDWIDTH=13963223,CODECS=\"ec-3,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/13000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=26598940,AVERAGE-BANDWIDTH=17398921,CODECS=\"ec-3,hvc1.2.4.H150.90\",RESOLUTION=3840x2160,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/16500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=4686819,AVERAGE-BANDWIDTH=3518042,CODECS=\"ec-3,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/2500k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=5941080,AVERAGE-BANDWIDTH=4299707,CODECS=\"ec-3,hvc1.2.4.L93.90\",RESOLUTION=1280x720,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/3300k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=7882680,AVERAGE-BANDWIDTH=5614284,CODECS=\"ec-3,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/4600k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=10429877,AVERAGE-BANDWIDTH=6996616,CODECS=\"ec-3,hvc1.2.4.L120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/6000k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=13053385,AVERAGE-BANDWIDTH=8662694,CODECS=\"ec-3,hvc1.2.4.H120.90\",RESOLUTION=1920x1080,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/7700k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=14807994,AVERAGE-BANDWIDTH=9777502,CODECS=\"ec-3,hvc1.2.4.L150.90\",RESOLUTION=2560x1440,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/8800k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=1666071,AVERAGE-BANDWIDTH=1475014,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/450k/vod.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8\n\n";
}
