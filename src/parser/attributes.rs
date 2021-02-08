use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};

use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};

fn parse_key(attr_str: &str) -> nom::IResult<&str, &str> {
    take_until("=")(attr_str)
}

fn parse_value(attr_str: &str) -> nom::IResult<&str, &str> {
    is_not(",/n")(attr_str)
}

fn parse_quoted_value(attr_str: &str) -> nom::IResult<&str, &str> {
    delimited(tag("\""), is_not("\""), tag("\""))(attr_str)
}

fn parse_attribute(attr_str: &str) -> nom::IResult<&str, (&str, &str)> {
    separated_pair(parse_key, tag("="), alt((parse_quoted_value, parse_value)))(attr_str)
}

fn parse_attributes(attrs_str: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    separated_list1(tag(","), parse_attribute)(attrs_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_i_frame_attribute_list_into_key_value_map() {
        let attr_str = r#"BANDWIDTH=77758,CODECS="hvc1.2.4.L63.90",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI="hdr10/unenc/900k/vod-iframe.m3u8""#;

        let parsed = parse_attributes(attr_str);

        let expected = Ok((
            "",
            vec![
                ("BANDWIDTH", "77758"),
                ("CODECS", "hvc1.2.4.L63.90"),
                ("RESOLUTION", "640x360"),
                ("VIDEO-RANGE", "PQ"),
                ("URI", "hdr10/unenc/900k/vod-iframe.m3u8"),
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
                ("TYPE", "AUDIO"),
                ("GROUP-ID", "aac-64k"),
                ("NAME", "English"),
                ("LANGUAGE", "en"),
                ("DEFAULT", "YES"),
                ("AUTOSELECT", "YES"),
                ("CHANNELS", "2"),
                ("URI", "audio/unenc/aac_64k/vod.m3u8"),
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
                ("BANDWIDTH", "2312764"),
                ("AVERAGE-BANDWIDTH", "1919803"),
                ("CODECS", "ec-3,hvc1.2.4.L63.90"),
                ("RESOLUTION", "640x360"),
                ("FRAME-RATE", "23.97"),
                ("VIDEO-RANGE", "PQ"),
                ("AUDIO", "atmos"),
                ("CLOSED-CAPTIONS", "NONE"),
            ],
        ));

        assert_eq!(parsed, expected)
    }

    #[test]
    fn parses_inner_list() {
        let attr_str = r#""ec-3,hvc1.2.4.L63.90""#;
        assert_eq!(
            parse_quoted_value(attr_str),
            Ok(("", "ec-3,hvc1.2.4.L63.90"))
        )
    }

    #[test]
    fn parses_key() {
        let attr_str = "BANDWIDTH=2312764";
        assert_eq!(parse_key(attr_str), Ok(("=2312764", "BANDWIDTH")))
    }

    #[test]
    fn parses_attribute_into_key_value() {
        let attr_str = "BANDWIDTH=2312764";
        assert_eq!(
            parse_attribute(attr_str),
            Ok(("", ("BANDWIDTH", "2312764")))
        )
    }

    #[test]
    fn parses_attribute_with_quoted_value_into_key_value() {
        let attr_str = r#"AUDIO="atmos""#;
        assert_eq!(parse_attribute(attr_str), Ok(("", ("AUDIO", "atmos"))))
    }

    #[test]
    fn parses_attribute_with_list_value_into_key_value() {
        let attr_str = r#"CODECS="ec-3,hvc1.2.4.L63.90""#;
        assert_eq!(
            parse_attribute(attr_str),
            Ok(("", ("CODECS", "ec-3,hvc1.2.4.L63.90")))
        )
    }

    #[test]
    fn parses_attribute_with_float_value_into_key_value() {
        let attr_str = "FRAME-RATE=23.97";
        assert_eq!(parse_attribute(attr_str), Ok(("", ("FRAME-RATE", "23.97"))))
    }

    #[test]
    fn parses_attribute_with_resolution_value_into_key_value() {
        let attr_str = "RESOLUTION=640x360";
        assert_eq!(
            parse_attribute(attr_str),
            Ok(("", ("RESOLUTION", "640x360")))
        )
    }
}
