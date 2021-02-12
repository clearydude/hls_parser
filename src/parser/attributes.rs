use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};

use nom::combinator::map;
use nom::multi::separated_list1;
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

fn parse_tag(tag_str: &str) -> nom::IResult<&str, &str> {
    take_until(":")(tag_str)
}

fn parse_tag_and_attributes(tag_str: &str) -> nom::IResult<&str, (&str, Vec<(String, String)>)> {
    separated_pair(parse_tag, tag(":"), parse_attributes)(tag_str)
}

#[derive(Debug, PartialEq)]
struct VariantStream {
    name: String,
    attributes: Vec<(String, String)>,
    uri: String,
}

#[derive(Debug, PartialEq)]
struct IFrame {
    attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
struct Media {
    attributes: Vec<(String, String)>,
}

fn parse_variant_stream(variant_stream_str: &str) -> nom::IResult<&str, VariantStream> {
    map(
        separated_pair(parse_tag_and_attributes, tag("\n"), parse_value),
        |((name, attributes), uri)| VariantStream {
            name: name.to_string(),
            attributes,
            uri,
        },
    )(variant_stream_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    // need to strip whitespace before both lines here
    // there's also whitespace after the uri that we need to strip
    #[test]
    fn parses_variant_stream() {
        let tags = r#"
            #EXT-X-STREAM-INF:BANDWIDTH=1352519,AVERAGE-BANDWIDTH=959558,CODECS="mp4a.40.2,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="aac-64k",CLOSED-CAPTIONS=NONE
            hdr10/unenc/900k/vod.m3u8
         "#;
        let parsed = parse_variant_stream(tags);

        let expected_variant_stream = VariantStream {
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

        assert_eq!(parsed, Ok(("", expected_variant_stream)))
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
}
