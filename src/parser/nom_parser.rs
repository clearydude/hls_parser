use nom::character::complete::multispace0;
use nom::combinator::{all_consuming, map, verify};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::IResult;
use std::collections::HashMap;

fn value(attr_str: &str) -> nom::IResult<&str, String> {
    map(terminated(is_not(",\n"), multispace0), |attr: &str| {
        attr.to_string()
    })(attr_str)
}

fn quoted_value(attr_str: &str) -> nom::IResult<&str, String> {
    map(
        terminated(delimited(tag("\""), is_not("\""), tag("\"")), multispace0),
        |attr: &str| attr.to_string(),
    )(attr_str)
}

fn attribute_pair(attr_str: &str) -> nom::IResult<&str, (String, String)> {
    map(
        separated_pair(take_until("="), tag("="), alt((quoted_value, value))),
        |(key, value)| (key.to_string(), value.to_string()),
    )(attr_str)
}

fn attributes(attrs_str: &str) -> nom::IResult<&str, HashMap<String, String>> {
    fold_many0(
        preceded(
            tag(":"),
            terminated(separated_list1(tag(","), attribute_pair), multispace0),
        ),
        HashMap::new(),
        |mut map, attrs| {
            for (key, value) in attrs {
                map.insert(key, value);
            }
            map
        },
    )(attrs_str)
}

fn tag_name(tag_str: &str) -> nom::IResult<&str, String> {
    map(preceded(tag("#"), is_not(":\n")), |name: &str| {
        name.to_string()
    })(tag_str)
}

fn tag_and_attributes(tag_str: &str) -> nom::IResult<&str, (String, HashMap<String, String>)> {
    pair(tag_name, attributes)(tag_str)
}

fn uri(uri_str: &str) -> nom::IResult<&str, String> {
    verify(value, |uri: &str| !uri.starts_with("#"))(uri_str)
}

fn variant_stream(
    variant_stream_str: &str,
) -> nom::IResult<&str, (String, HashMap<String, String>)> {
    map(
        tuple((tag_name, attributes, preceded(multispace0, uri))),
        |(tag, mut attrs, uri)| {
            attrs.insert("URI".to_string(), uri);
            (tag, attrs)
        },
    )(variant_stream_str)
}

// Note: I did try to get this to return an Option<HashMap> in the case of tags with no
// attributes but I couldn't get the parser to play nice in the time that I had.
// An optional hashmap here would be better than returning an empty hashmap if the tag
// has no associated attributes.
pub(crate) fn master_playlist(
    playlist_str: &str,
) -> IResult<&str, Vec<(String, HashMap<String, String>)>> {
    all_consuming(many1(terminated(
        alt((variant_stream, tag_and_attributes)),
        multispace0,
    )))(playlist_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn get_media_attributes() -> HashMap<String, String> {
        let mut expected = HashMap::new();
        expected.insert("TYPE".to_string(), "AUDIO".to_string());
        expected.insert("GROUP-ID".to_string(), "aac-64k".to_string());
        expected.insert("NAME".to_string(), "English".to_string());
        expected.insert("LANGUAGE".to_string(), "en".to_string());
        expected.insert("DEFAULT".to_string(), "YES".to_string());
        expected.insert("AUTOSELECT".to_string(), "YES".to_string());
        expected.insert("CHANNELS".to_string(), "2".to_string());
        expected.insert(
            "URI".to_string(),
            "audio/unenc/aac_64k/vod.m3u8".to_string(),
        );
        expected
    }

    pub(crate) fn get_variant_stream_attributes() -> HashMap<String, String> {
        let mut expected = HashMap::new();
        expected.insert("BANDWIDTH".to_string(), "2312764".to_string());
        expected.insert("AVERAGE-BANDWIDTH".to_string(), "1919803".to_string());
        expected.insert("CODECS".to_string(), "ec-3,hvc1.2.4.L63.90".to_string());
        expected.insert("RESOLUTION".to_string(), "640x360".to_string());
        expected.insert("FRAME-RATE".to_string(), "23.97".to_string());
        expected.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
        expected.insert("AUDIO".to_string(), "atmos".to_string());
        expected.insert("CLOSED-CAPTIONS".to_string(), "NONE".to_string());
        expected
    }

    fn get_variant_attrs_with_uri() -> HashMap<String, String> {
        let mut expected = get_variant_stream_attributes();
        expected.insert("URI".to_string(), "hdr10/unenc/900k/vod.m3u8".to_string());
        expected
    }

    pub(crate) fn get_i_frame_attributes() -> HashMap<String, String> {
        let mut expected = HashMap::new();
        expected.insert("BANDWIDTH".to_string(), "77758".to_string());
        expected.insert("CODECS".to_string(), "hvc1.2.4.L63.90".to_string());
        expected.insert("RESOLUTION".to_string(), "640x360".to_string());
        expected.insert("VIDEO-RANGE".to_string(), "PQ".to_string());
        expected.insert(
            "URI".to_string(),
            "hdr10/unenc/900k/vod-iframe.m3u8".to_string(),
        );
        expected
    }

    #[test]
    fn parses_all_tags() {
        let tags_str = "#EXTM3U\n#EXT-X-INDEPENDENT-SEGMENTS\n\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac-64k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_64k/vod.m3u8\"\n\n#EXT-X-STREAM-INF:BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8\n\n#EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=77758,CODECS=\"hvc1.2.4.L63.90\",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/900k/vod-iframe.m3u8\"\n\n";

        let parsed = master_playlist(tags_str);

        let expected = vec![
            ("EXTM3U".to_string(), HashMap::new()),
            ("EXT-X-INDEPENDENT-SEGMENTS".to_string(), HashMap::new()),
            ("EXT-X-MEDIA".to_string(), get_media_attributes()),
            ("EXT-X-STREAM-INF".to_string(), get_variant_attrs_with_uri()),
            (
                "EXT-X-I-FRAME-STREAM-INF".to_string(),
                get_i_frame_attributes(),
            ),
        ];

        assert_eq!(parsed, Ok(("", expected)));
    }

    #[test]
    fn parses_variant_stream() {
        let tags = "#EXT-X-STREAM-INF:BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE\nhdr10/unenc/900k/vod.m3u8";

        let parsed = variant_stream(tags);

        assert_eq!(
            parsed,
            Ok((
                "",
                ("EXT-X-STREAM-INF".to_string(), get_variant_attrs_with_uri())
            ))
        )
    }

    #[test]
    fn parses_i_frame_attribute_list_into_key_value_map() {
        let attr_str = ":BANDWIDTH=77758,CODECS=\"hvc1.2.4.L63.90\",RESOLUTION=640x360,VIDEO-RANGE=PQ,URI=\"hdr10/unenc/900k/vod-iframe.m3u8\"";

        let parsed = attributes(attr_str);

        assert_eq!(parsed, Ok(("", get_i_frame_attributes())))
    }

    #[test]
    fn parses_media_attribute_list_into_key_value_map() {
        let attr_str = ":TYPE=AUDIO,GROUP-ID=\"aac-64k\",NAME=\"English\",LANGUAGE=\"en\",DEFAULT=YES,AUTOSELECT=YES,CHANNELS=\"2\",URI=\"audio/unenc/aac_64k/vod.m3u8\"";

        let parsed = attributes(attr_str);

        assert_eq!(parsed, Ok(("", get_media_attributes())))
    }

    #[test]
    fn parses_variant_stream_attribute_list_into_key_value_map() {
        let attr_str = ":BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS=\"ec-3,hvc1.2.4.L63.90\",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO=\"atmos\",CLOSED-CAPTIONS=NONE";
        let parsed = attributes(attr_str);

        assert_eq!(parsed, Ok(("", get_variant_stream_attributes())))
    }

    #[test]
    fn parses_inner_list() {
        let attr_str = "\"ec-3,hvc1.2.4.L63.90\"";
        assert_eq!(
            quoted_value(attr_str),
            Ok(("", "ec-3,hvc1.2.4.L63.90".to_string()))
        )
    }

    #[test]
    fn parses_attribute_into_key_value() {
        let attr_str = "BANDWIDTH=2312764";
        assert_eq!(
            attribute_pair(attr_str),
            Ok(("", ("BANDWIDTH".to_string(), "2312764".to_string())))
        )
    }

    #[test]
    fn parses_attribute_with_quoted_value_into_key_value() {
        let attr_str = "AUDIO=\"atmos\"";
        assert_eq!(
            attribute_pair(attr_str),
            Ok(("", ("AUDIO".to_string(), "atmos".to_string())))
        )
    }

    #[test]
    fn parses_attribute_with_list_value_into_key_value() {
        let attr_str = "CODECS=\"ec-3,hvc1.2.4.L63.90\"";
        assert_eq!(
            attribute_pair(attr_str),
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
            attribute_pair(attr_str),
            Ok(("", ("FRAME-RATE".to_string(), "23.97".to_string())))
        )
    }

    #[test]
    fn parses_attribute_with_resolution_value_into_key_value() {
        let attr_str = "RESOLUTION=640x360";
        assert_eq!(
            attribute_pair(attr_str),
            Ok(("", ("RESOLUTION".to_string(), "640x360".to_string())))
        )
    }
}
