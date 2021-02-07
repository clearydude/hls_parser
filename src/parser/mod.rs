use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{recognize, rest};
use nom::sequence::separated_pair;

fn parse_key(attr_str: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_until("=")(attr_str)
}

fn parse_attribute(attr_str: &str) -> nom::IResult<&str, (&str, &str)> {
    separated_pair(parse_key, tag("="), rest)(attr_str)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(parse_attribute(attr_str), Ok(("", ("AUDIO", r#""atmos""#))))
    }

    #[test]
    fn parses_attribute_with_list_value_into_key_value() {
        let attr_str = r#"CODECS="ec-3,hvc1.2.4.L63.90""#;
        assert_eq!(
            parse_attribute(attr_str),
            Ok(("", ("CODECS", r#""ec-3,hvc1.2.4.L63.90""#)))
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

    // #[test]
    // fn parses_attribute_list_into_key_value_map() {
    //     let attr_str = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;
    //
    //     let mut expected_map: HashMap<String, String> = HashMap::new();
    //     expected_map.insert("BANDWIDTH".into(), "2312764".into());
    //     expected_map.insert("AVERAGE-BANDWIDTH".into(), "1919803".into());
    //     expected_map.insert("CODECS".into(), "ec-3,hvc1.2.4.L63.90".into());
    //     expected_map.insert("RESOLUTION".into(), "640x360".into());
    //     expected_map.insert("FRAME-RATE".into(), "23.97".into());
    //     expected_map.insert("VIDEO-RANGE".into(), "PQ".into());
    //     expected_map.insert("AUDIO".into(), "atmos".into());
    //     expected_map.insert("CLOSED-CAPTIONS".into(), "NONE".into());
    //
    //     let parsed_map = parse_attr_list_to_map(attr_str);
    //
    //    assert_eq!(parsed_map, expected_map)
    // }
}
