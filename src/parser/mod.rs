use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};

use nom::branch::alt;
use nom::character::complete::{char, line_ending, not_line_ending, anychar, none_of};
use nom::combinator::{not, rest, value};
use nom::multi::{many0, separated_list1};
use nom::sequence::{separated_pair, delimited};
use nom::character::streaming::alpha1;

fn parse_key(attr_str: &str) -> nom::IResult<&str, &str> {
    take_until("=")(attr_str)
}

fn parse_value(attr_str: &str) -> nom::IResult<&str, &str> {
    println!("What we gotta parse? {}", attr_str);
    alt((
        take_until(","),
        take_until("eol")
    ))(attr_str)
}

fn parse_inner(attr_str: &str) -> nom::IResult<&str, &str> {
    println!("Trying to parse? {}", attr_str);
    delimited(char('"'), take_until(r#"""#), char('"'))(attr_str)
}

fn parse_attribute(attr_str: &str) -> nom::IResult<&str, (&str, &str)> {
    separated_pair(
        parse_key,
        char('='),
        parse_value,
    )(attr_str)
}

fn parse_attributes(attrs_str: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    separated_list1(char(','), parse_attribute)(attrs_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_attribute_list_into_key_value_map() {
        let attr_str = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;
        let parsed = parse_attributes(attr_str);

        let expected = Ok(("", vec![("BANDWIDTH", "2312764"), ("AVERAGE-BANDWIDTH", "1919803"), ("CODECS", "\"ec-3, hvc1.2.4.L63.90\""), ("RESOLUTION", "640x360"), ("FRAME-RATE", "23.97"), ("VIDEO-RANGE", "PQ"), ("AUDIO", "\"atmos\""), ("CLOSED-CAPTIONS","NONE")]));

        assert_eq!(parsed, expected)
    }

    // #[test]
    // fn parses_inner_list() {
    //     let attr_str = r#"CODECS="ec-3,hvc1.2.4.L63.90""#;
    //     assert_eq!(
    //         parse_inner(attr_str),
    //         Ok(("", r#""ec-3,hvc1.2.4.L63.90""#))
    //     )
    // }

    #[test]
    fn parses_key() {
        #[test]
        fn parses_attribute_into_key_value() {
            let attr_str = "BANDWIDTH=2312764";
            assert_eq!(
                parse_key(attr_str),
                Ok(("", "BANDWIDTH"))
            )
        }
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
}
